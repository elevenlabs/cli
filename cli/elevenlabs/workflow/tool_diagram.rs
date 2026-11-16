//! `tools diagram`: one tool's own structure, drawn as a tree in the terminal
//! or printed as a Mermaid flowchart (`--format`).
//!
//! `agents workflow diagram` shows a tool node by its ID and `agents graph`
//! shows which agents use which tools; neither shows what is inside a tool.
//! This draws a webhook's method and URL, its path/query/body parameters with
//! where each value comes from (the LLM, a dynamic variable, a constant, the
//! system, or left out), its headers and auth connection, and what it does
//! with the response; a client tool's parameters and whether it waits for a
//! response; and a system tool's transfer rules.
//!
//! Input is the `tool_configs/*.json` wire format, which is also what the
//! SDKs' tool builders write, bare or wrapped in `tool_config` (as the API
//! returns it). An agent config draws its built-in system tools and inline
//! tools. Like `agents workflow diagram`, rendering works on
//! [`serde_json::Value`] so fields this build doesn't model still draw.
//!
//! Output is deterministic: maps are drawn in key order and lists in their
//! own order, so a diagram only changes when the tool does.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::agents::find_agent;
use super::mermaid::{
    fence, quote, str_field, terminal_width, truncate, use_color, Format, MermaidIds,
    NODE_LABEL_MAX,
};
use super::{api, project};

/// Register `tools diagram`.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["tools"],
        clap::Command::new("diagram").about(
            "Draw a tool's parameters, value sources, headers and response handling, \
             in the terminal or as Mermaid",
        ),
        handle_diagram,
    )
}

#[derive(clap::Args)]
struct DiagramArgs {
    /// A tool config file (bare, or wrapped in `tool_config`), an agent config
    /// file (draws its system and inline tools), a tool ID or name from
    /// tools.json, or an agent ID or name from agents.json. A tool ID found in
    /// neither is fetched from ElevenLabs.
    target: String,
    /// What to print: a tree for the terminal, Mermaid source, or Mermaid in
    /// a ```mermaid fence, ready to paste into Markdown.
    #[arg(long, value_enum, default_value_t = Format::Ascii)]
    format: Format,
}

fn handle_diagram(args: DiagramArgs, ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("tools.diagram");
    let (tools, source) = load(&args.target, ctx)?;
    let views: Vec<ToolView> = tools.iter().map(view).collect();
    match args.format {
        Format::Ascii => print!("{}", render_terminal(&views, use_color(), terminal_width())),
        Format::Mermaid => print!("{}", render_mermaid(&views)),
        Format::MermaidMd => print!("{}", fence(&render_mermaid(&views))),
    }
    if views.is_empty() {
        eprintln!("{source} has no tools to draw.");
    }
    Ok(())
}

// ── Loading ─────────────────────────────────────────────────────────

/// The tool configs to draw and where they came from (for messages).
fn load(target: &str, ctx: &AppContext) -> Result<(Vec<Value>, String), CliError> {
    if Path::new(target).is_file() {
        let value = project::read_value(Path::new(target))?;
        return Ok((tools_in(&value, target)?, target.to_string()));
    }
    if Path::new(project::TOOLS_FILE).exists() {
        let registry = project::load_tools()?;
        let found = registry.tools.iter().find(|t| {
            t.id.as_deref() == Some(target)
                || project::config_name(&t.config).as_deref() == Some(target)
        });
        if let Some(tool) = found {
            let value = project::read_value_in_project(&tool.config)?;
            return Ok((tools_in(&value, &tool.config)?, tool.config.clone()));
        }
    }
    // An agent from agents.json: its system and inline tools.
    if Path::new(project::AGENTS_FILE).exists() {
        let registry = project::load_agents()?;
        if let Some(agent) = find_agent(&registry, target) {
            let value = project::read_value_in_project(&agent.config)?;
            return Ok((tools_in(&value, &agent.config)?, agent.config.clone()));
        }
    }
    // Names have no spaces either, but only IDs can be fetched.
    if target.is_empty() || target.contains(char::is_whitespace) || target.contains('/') {
        return Err(CliError::Validation(format!(
            "'{target}' is not a config file, or a tool or agent ID or name in tools.json or agents.json"
        )));
    }
    let fetched = api::get_tool(ctx, target).map_err(|e| {
        CliError::Validation(format!(
            "'{target}' is not a config file, or a tool or agent in tools.json or agents.json, \
             and fetching it from ElevenLabs as a tool ID failed: {e}"
        ))
    })?;
    Ok((tools_in(&fetched, target)?, target.to_string()))
}

/// The tool configs in a file: the file itself when it is a tool config, its
/// `tool_config` when wrapped (as the API returns it), or an agent config's
/// inline `prompt.tools` and non-null `prompt.built_in_tools`.
fn tools_in(value: &Value, source: &str) -> Result<Vec<Value>, CliError> {
    if let Some(inner) = value.get("tool_config").filter(|v| v.is_object()) {
        return Ok(vec![inner.clone()]);
    }
    if value.get("type").is_some_and(Value::is_string) && value.get("name").is_some() {
        return Ok(vec![value.clone()]);
    }
    let prompt = value
        .pointer("/conversation_config/agent/prompt")
        .or_else(|| value.pointer("/agent/prompt"))
        .or_else(|| value.get("prompt"));
    let Some(prompt) = prompt else {
        return Err(CliError::Validation(format!(
            "{source} is not a tool config (no `type` and `name`, or `tool_config`) \
             or an agent config"
        )));
    };
    let mut tools: Vec<Value> = prompt
        .get("tools")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if let Some(built_in) = prompt.get("built_in_tools").and_then(Value::as_object) {
        for (key, tool) in built_in {
            let Some(obj) = tool.as_object() else {
                continue;
            };
            let mut obj = obj.clone();
            obj.entry("name")
                .or_insert_with(|| Value::String(key.clone()));
            obj.entry("type")
                .or_insert_with(|| Value::String("system".into()));
            tools.push(Value::Object(obj));
        }
    }
    Ok(tools)
}

/// Tool ID → name, from tools.json in the current directory. Empty when
/// there is none or it can't be read: names are a nicety for diagrams.
pub(super) fn tool_names_by_id() -> BTreeMap<String, String> {
    if !Path::new(project::TOOLS_FILE).exists() {
        return BTreeMap::new();
    }
    let Ok(registry) = project::load_tools() else {
        return BTreeMap::new();
    };
    registry
        .tools
        .iter()
        .filter_map(|t| Some((t.id.clone()?, project::config_name(&t.config)?)))
        .collect()
}

// ── The view: a tree both renderers draw ────────────────────────────

/// What a piece of text is, for colour in the terminal.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Tone {
    Plain,
    Strong,
    Dim,
    Llm,
    Var,
    Constant,
    System,
    Omitted,
    Secret,
    Env,
    Placeholder,
    Warn,
}

impl Tone {
    fn ansi(self) -> Option<&'static str> {
        Some(match self {
            Tone::Plain => return None,
            Tone::Strong => "1",
            Tone::Dim | Tone::Omitted => "2",
            Tone::Llm => "36",
            Tone::Var => "35",
            Tone::Constant => "33",
            Tone::System => "34",
            Tone::Secret => "31",
            Tone::Env => "32",
            Tone::Placeholder => "1;33",
            Tone::Warn => "1;31",
        })
    }
}

#[derive(Clone, Debug)]
struct Span {
    text: String,
    tone: Tone,
}

fn span(text: impl Into<String>, tone: Tone) -> Span {
    Span {
        text: text.into(),
        tone,
    }
}

fn plain_text(spans: &[Span]) -> String {
    spans.iter().map(|s| s.text.as_str()).collect()
}

/// One node of the tree: a head line, notes under it, and children.
struct Item {
    /// Unique within the tool, for Mermaid IDs.
    key: String,
    head: Vec<Span>,
    notes: Vec<Vec<Span>>,
    /// Mermaid class.
    class: &'static str,
    children: Vec<Item>,
}

impl Item {
    fn new(key: impl Into<String>, head: Vec<Span>, class: &'static str) -> Self {
        Item {
            key: key.into(),
            head,
            notes: Vec::new(),
            class,
            children: Vec::new(),
        }
    }

    fn section(key: impl Into<String>, title: &str) -> Self {
        Item::new(key, vec![span(title, Tone::Strong)], "t_section")
    }
}

/// One tool, ready to draw.
struct ToolView {
    /// `Webhook tool`, `Client tool`, `System tool (transfer_to_number)`.
    kind: String,
    name: String,
    /// Description and settings lines under the title.
    summary: Vec<Vec<Span>>,
    items: Vec<Item>,
}

fn view(tool: &Value) -> ToolView {
    let tool_type = str_field(tool, "type").unwrap_or("unknown");
    let name = str_field(tool, "name").unwrap_or("(unnamed)").to_string();
    let mut summary = Vec::new();
    if let Some(desc) = str_field(tool, "description").filter(|d| !d.trim().is_empty()) {
        summary.push(vec![span(one_line(desc), Tone::Plain)]);
    }
    let settings = settings(tool, tool_type);
    if !settings.is_empty() {
        summary.push(vec![span(settings.join(" · "), Tone::Dim)]);
    }
    let (kind, mut items) = match tool_type {
        "webhook" => ("Webhook tool".to_string(), webhook_items(tool)),
        "client" => ("Client tool".to_string(), client_items(tool)),
        "system" => {
            let system_type = tool
                .get("params")
                .and_then(|p| str_field(p, "system_tool_type"))
                .unwrap_or(&name)
                .to_string();
            let kind = if system_type == name {
                "System tool".to_string()
            } else {
                format!("System tool ({system_type})")
            };
            (kind, system_items(tool))
        }
        other => (format!("{other} tool"), Vec::new()),
    };
    if tool_type != "webhook" {
        if let Some(response) = response_item(tool, None) {
            items.push(response);
        }
    }
    if let Some(defaults) = placeholder_defaults(tool) {
        items.push(defaults);
    }
    ToolView {
        kind,
        name,
        summary,
        items,
    }
}

/// Settings worth a mention, skipping the defaults.
fn settings(tool: &Value, tool_type: &str) -> Vec<String> {
    let mut out = Vec::new();
    if tool_type == "client" {
        out.push(
            if tool.get("expects_response").and_then(Value::as_bool) == Some(true) {
                "waits for a response"
            } else {
                "does not wait for a response"
            }
            .to_string(),
        );
    }
    if let Some(secs) = tool.get("response_timeout_secs").and_then(Value::as_i64) {
        out.push(if secs < 0 {
            "no timeout".to_string()
        } else {
            format!("timeout {secs}s")
        });
    }
    for (key, label, default) in [
        ("execution_mode", "execution", "immediate"),
        ("pre_tool_speech", "pre-tool speech", "auto"),
        ("interruption_mode", "interruptions", "allow"),
        ("tool_error_handling_mode", "on error", "auto"),
    ] {
        if let Some(value) = str_field(tool, key).filter(|v| *v != default) {
            out.push(format!("{label} {value}"));
        }
    }
    if tool.get("force_pre_tool_speech").and_then(Value::as_bool) == Some(true) {
        out.push("pre-tool speech force".to_string());
    }
    if tool.get("disable_interruptions").and_then(Value::as_bool) == Some(true) {
        out.push("interruptions disabled".to_string());
    }
    if tool.get("follow_redirects").and_then(Value::as_bool) == Some(true) {
        out.push("follows redirects".to_string());
    }
    out
}

// ── Webhook ─────────────────────────────────────────────────────────

fn webhook_items(tool: &Value) -> Vec<Item> {
    let empty = Value::Null;
    let api = tool.get("api_schema").unwrap_or(&empty);
    let method = str_field(api, "method").unwrap_or("GET");
    let url = str_field(api, "url").unwrap_or("");
    let mut head = vec![span(method, Tone::Strong), span(" ", Tone::Plain)];
    head.extend(url_spans(url));
    let mut request = Item::new("request", head, "t_request");

    let parts = url_parts(url);
    if !parts.templates.is_empty() {
        let mut section = Item::section("request/templates", "URL variables");
        for template in &parts.templates {
            let head = template_spans(template);
            let class = if env_label(template).is_some() {
                "t_env"
            } else {
                "t_var"
            };
            section.children.push(Item::new(
                format!("request/templates/{template}"),
                head,
                class,
            ));
        }
        request.children.push(section);
    }

    // Path params, in URL order, then any the URL doesn't use.
    let schema = api.get("path_params_schema").and_then(Value::as_object);
    let auth_resolved: BTreeSet<&str> = api
        .get("auth_resolved_params")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let mut path = Item::section("request/path", "Path params");
    for placeholder in &parts.placeholders {
        let key = format!("request/path/{placeholder}");
        if let Some(prop) = schema.and_then(|s| s.get(placeholder.as_str())) {
            path.children.push(param(
                &key,
                placeholder,
                prop,
                Some(true),
                Side::Request,
                false,
            ));
        } else if auth_resolved.contains(placeholder.as_str()) {
            path.children.push(Item::new(
                key,
                vec![
                    span(placeholder.clone(), Tone::Strong),
                    span(" [from auth connection]", Tone::Secret),
                ],
                "t_secret",
            ));
        } else {
            path.children.push(Item::new(
                key,
                vec![
                    span(placeholder.clone(), Tone::Strong),
                    span(" [no schema in path_params_schema]", Tone::Warn),
                ],
                "t_warn",
            ));
        }
    }
    if let Some(schema) = schema {
        for (name, prop) in schema {
            if parts.placeholders.contains(name) {
                continue;
            }
            let mut item = param(
                &format!("request/path/{name}"),
                name,
                prop,
                Some(true),
                Side::Request,
                false,
            );
            item.notes.push(vec![span(
                format!("{{{name}}} is not in the URL"),
                Tone::Warn,
            )]);
            path.children.push(item);
        }
    }
    if !path.children.is_empty() {
        request.children.push(path);
    }

    if let Some(query) = api.get("query_params_schema").filter(|q| q.is_object()) {
        let mut section = Item::section("request/query", "Query params");
        section.children = properties("request/query", query, Side::Request, false);
        if !section.children.is_empty() {
            request.children.push(section);
        }
    }

    if let Some(body) = api.get("request_body_schema").filter(|b| b.is_object()) {
        let content_type = str_field(api, "content_type").unwrap_or("application/json");
        let mut section = Item::new(
            "request/body",
            vec![
                span("Body", Tone::Strong),
                span(format!(" {content_type}"), Tone::Dim),
            ],
            "t_section",
        );
        let source = Source::of(body);
        if !matches!(source, Source::Llm) {
            section.head.extend(source.tag());
        }
        if let Some(desc) = description(body) {
            section.notes.push(vec![span(desc, Tone::Dim)]);
        }
        section.children = properties(
            "request/body",
            body,
            Side::Request,
            !matches!(source, Source::Llm),
        );
        request.children.push(section);
    }

    if let Some(headers) = api
        .get("request_headers")
        .and_then(Value::as_object)
        .filter(|h| !h.is_empty())
    {
        let mut section = Item::section("request/headers", "Headers");
        for (name, value) in headers {
            section.children.push(header(name, value));
        }
        request.children.push(section);
    }

    if let Some(auth) = api.get("auth_connection").filter(|a| a.is_object()) {
        let (tag, class) = if let Some(id) = str_field(auth, "auth_connection_id") {
            (
                span(format!(" [auth connection {id}]"), Tone::Secret),
                "t_secret",
            )
        } else if let Some(label) = str_field(auth, "env_var_label") {
            (span(format!(" [env {label}]"), Tone::Env), "t_env")
        } else {
            (span(format!(" {}", compact(auth)), Tone::Dim), "t_section")
        };
        request.children.push(Item::new(
            "request/auth",
            vec![span("Auth", Tone::Strong), tag],
            class,
        ));
    }

    let mut items = vec![request];
    if let Some(response) = response_item(tool, Some(api)) {
        items.push(response);
    }
    items
}

fn header(name: &str, value: &Value) -> Item {
    let key = format!("request/headers/{name}");
    let name_span = span(name, Tone::Strong);
    if let Some(text) = value.as_str() {
        let mut head = vec![name_span, span(" = ", Tone::Dim)];
        head.extend(template_text_spans(&format!("\"{text}\""), Tone::Constant));
        return Item::new(key, head, "t_const");
    }
    let (tag, class) = if let Some(id) = str_field(value, "secret_id") {
        (span(format!(" [secret {id}]"), Tone::Secret), "t_secret")
    } else if let Some(var) = str_field(value, "variable_name") {
        (span(format!(" [var {var}]"), Tone::Var), "t_var")
    } else if let Some(label) = str_field(value, "env_var_label") {
        (span(format!(" [env {label}]"), Tone::Env), "t_env")
    } else {
        (span(format!(" {}", compact(value)), Tone::Dim), "t_const")
    };
    Item::new(key, vec![name_span, tag], class)
}

/// The response: its documented schema, filter, and assignments to dynamic
/// variables. `api` is the webhook's `api_schema`.
fn response_item(tool: &Value, api: Option<&Value>) -> Option<Item> {
    let mut section = Item::section("response", "Response");
    if let Some(schema) = api
        .and_then(|a| a.get("response_body_schema"))
        .filter(|s| s.is_object())
    {
        let mut item = Item::section("response/schema", "Schema");
        if let Some(desc) = description(schema) {
            item.notes.push(vec![span(desc, Tone::Dim)]);
        }
        item.children = properties("response/schema", schema, Side::Response, false);
        section.children.push(item);
    }
    if let Some(filter) = api
        .and_then(|a| a.get("response_filter"))
        .filter(|f| f.is_object())
    {
        let mode = str_field(filter, "mode").unwrap_or("all");
        let mut item = Item::new(
            "response/filter",
            vec![
                span("Filter", Tone::Strong),
                span(format!(" {mode}"), Tone::Plain),
            ],
            "t_section",
        );
        let paths: Vec<&str> = filter
            .get("filters")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        if !paths.is_empty() {
            item.notes
                .push(vec![span(format!("keeps {}", paths.join(", ")), Tone::Dim)]);
        }
        section.children.push(item);
    }
    let assignments = tool
        .get("assignments")
        .and_then(Value::as_array)
        .filter(|a| !a.is_empty());
    if let Some(assignments) = assignments {
        let mut item = Item::section("response/assignments", "Assignments");
        for (i, a) in assignments.iter().enumerate() {
            let path = str_field(a, "value_path").unwrap_or("?");
            let var = str_field(a, "dynamic_variable").unwrap_or("?");
            let mut head = vec![
                span(path, Tone::Strong),
                span(" → ", Tone::Dim),
                span(format!("{{{{{var}}}}}"), Tone::Var),
            ];
            if let Some(source) = str_field(a, "source").filter(|s| *s != "response") {
                head.insert(0, span(format!("{source}: "), Tone::Dim));
            }
            let mut flags = Vec::new();
            if a.get("sanitize").and_then(Value::as_bool) == Some(true) {
                flags.push("hidden from the LLM");
            }
            if a.get("preserve_native_type").and_then(Value::as_bool) == Some(true) {
                flags.push("native type");
            }
            if !flags.is_empty() {
                head.push(span(format!(" ({})", flags.join(", ")), Tone::Dim));
            }
            item.children.push(Item::new(
                format!("response/assignments/{i}"),
                head,
                "t_var",
            ));
        }
        section.children.push(item);
    }
    (!section.children.is_empty()).then_some(section)
}

fn placeholder_defaults(tool: &Value) -> Option<Item> {
    let defaults = tool
        .pointer("/dynamic_variables/dynamic_variable_placeholders")
        .and_then(Value::as_object)
        .filter(|d| !d.is_empty())?;
    let mut section = Item::section("defaults", "Dynamic variable defaults");
    for (name, value) in defaults {
        section.children.push(Item::new(
            format!("defaults/{name}"),
            vec![
                span(format!("{{{{{name}}}}}"), Tone::Var),
                span(" = ", Tone::Dim),
                span(compact(value), Tone::Constant),
            ],
            "t_var",
        ));
    }
    Some(section)
}

// ── URL ─────────────────────────────────────────────────────────────

/// `{name}` path placeholders and `{{var}}` templates in a URL, in order of
/// first appearance.
#[derive(Default, Debug, PartialEq)]
struct UrlParts {
    placeholders: Vec<String>,
    templates: Vec<String>,
}

/// The URL split into literal text, `{{template}}`s and `{placeholder}`s.
fn url_tokens(url: &str) -> Vec<(UrlToken, &str)> {
    let mut out = Vec::new();
    let mut rest = url;
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix("{{") {
            if let Some(end) = after.find("}}") {
                out.push((UrlToken::Template, &after[..end]));
                rest = &after[end + 2..];
                continue;
            }
        } else if let Some(after) = rest.strip_prefix('{') {
            if let Some(end) = after.find('}').filter(|e| !after[..*e].contains('{')) {
                out.push((UrlToken::Placeholder, &after[..end]));
                rest = &after[end + 1..];
                continue;
            }
        }
        // Literal text up to the next brace (or one char, for a stray brace).
        let next = rest[1..].find('{').map_or(rest.len(), |i| i + 1);
        out.push((UrlToken::Text, &rest[..next]));
        rest = &rest[next..];
    }
    out
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum UrlToken {
    Text,
    Template,
    Placeholder,
}

fn url_parts(url: &str) -> UrlParts {
    let mut parts = UrlParts::default();
    for (token, text) in url_tokens(url) {
        let list = match token {
            UrlToken::Text => continue,
            UrlToken::Template => &mut parts.templates,
            UrlToken::Placeholder => &mut parts.placeholders,
        };
        let text = text.trim().to_string();
        if !list.contains(&text) {
            list.push(text);
        }
    }
    parts
}

fn url_spans(url: &str) -> Vec<Span> {
    template_text_spans(url, Tone::Plain)
}

/// Text with `{placeholder}`s and `{{template}}`s picked out.
fn template_text_spans(text: &str, base: Tone) -> Vec<Span> {
    url_tokens(text)
        .into_iter()
        .map(|(token, t)| match token {
            UrlToken::Text => span(t, base),
            UrlToken::Placeholder => span(format!("{{{t}}}"), Tone::Placeholder),
            UrlToken::Template => {
                let tone = if env_label(t.trim()).is_some() {
                    Tone::Env
                } else {
                    Tone::Var
                };
                span(format!("{{{{{t}}}}}"), tone)
            }
        })
        .collect()
}

/// `api_host` for `system__env_api_host`.
fn env_label(template: &str) -> Option<&str> {
    template.strip_prefix("system__env_")
}

fn template_spans(template: &str) -> Vec<Span> {
    let mut head = vec![span(format!("{{{{{template}}}}}"), Tone::Strong)];
    match env_label(template) {
        Some(label) => head.push(span(format!(" [env {label}]"), Tone::Env)),
        None => head.push(span(format!(" [var {template}]"), Tone::Var)),
    }
    head
}

// ── Parameters ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum Side {
    /// Sent to the tool: each value has a source.
    Request,
    /// The documented response: structure only.
    Response,
}

/// Where a parameter's value comes from. The API makes these mutually
/// exclusive; the first set wins here.
enum Source {
    Llm,
    Var(String),
    Constant(String),
    System,
    Omitted,
}

impl Source {
    fn of(schema: &Value) -> Source {
        if schema.get("is_omitted").and_then(Value::as_bool) == Some(true) {
            return Source::Omitted;
        }
        // The API returns `""` for an unset constant.
        match schema.get("constant_value") {
            None | Some(Value::Null) => {}
            Some(Value::String(s)) if s.is_empty() => {}
            Some(v) => return Source::Constant(compact(v)),
        }
        if schema.get("is_system_provided").and_then(Value::as_bool) == Some(true) {
            return Source::System;
        }
        if let Some(var) = str_field(schema, "dynamic_variable").filter(|v| !v.is_empty()) {
            return Source::Var(var.to_string());
        }
        Source::Llm
    }

    fn tag(&self) -> Vec<Span> {
        let (text, tone) = match self {
            Source::Llm => ("[LLM]".to_string(), Tone::Llm),
            Source::Var(v) => (format!("[var {v}]"), Tone::Var),
            Source::Constant(c) => (format!("[constant {c}]"), Tone::Constant),
            Source::System => ("[system]".to_string(), Tone::System),
            Source::Omitted => ("[omitted]".to_string(), Tone::Omitted),
        };
        vec![span(" ", Tone::Plain), span(text, tone)]
    }

    fn class(&self) -> &'static str {
        match self {
            Source::Llm => "t_llm",
            Source::Var(_) => "t_var",
            Source::Constant(_) => "t_const",
            Source::System => "t_system",
            Source::Omitted => "t_omitted",
        }
    }
}

/// The properties of an object schema (or query params), in key order, each
/// marked required or optional from its `required` list.
fn properties(prefix: &str, schema: &Value, side: Side, inherited: bool) -> Vec<Item> {
    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|r| r.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let Some(props) = schema.get("properties").and_then(Value::as_object) else {
        return Vec::new();
    };
    props
        .iter()
        .map(|(name, prop)| {
            let req =
                (side == Side::Request && !inherited).then(|| required.contains(&name.as_str()));
            param(
                &format!("{prefix}/{name}"),
                name,
                prop,
                req,
                side,
                inherited,
            )
        })
        .collect()
}

/// One parameter and, for an object or array, what is inside it.
/// `inherited` is set inside a parameter whose whole value comes from
/// elsewhere, so its parts have no source of their own.
fn param(
    key: &str,
    name: &str,
    schema: &Value,
    required: Option<bool>,
    side: Side,
    inherited: bool,
) -> Item {
    let ty = type_label(schema);
    let mut head = vec![span(name, Tone::Strong), span(format!(": {ty}"), Tone::Dim)];
    match required {
        Some(true) => head.push(span(" required", Tone::Plain)),
        Some(false) => head.push(span(" optional", Tone::Dim)),
        None => {}
    }
    let has_props = schema
        .get("properties")
        .and_then(Value::as_object)
        .is_some_and(|p| !p.is_empty());
    let is_container = has_props || schema.get("items").is_some_and(Value::is_object);
    let source = Source::of(schema);
    let own_source = side == Side::Request && !inherited;
    // A container the LLM fills is filled part by part; the parts say so.
    let show_tag = own_source && !(is_container && matches!(source, Source::Llm));
    let class = if show_tag { source.class() } else { "t_param" };
    if show_tag {
        head.extend(source.tag());
    }
    let mut item = Item::new(key, head, class);

    if let Some(desc) = description(schema) {
        let tone = if show_tag && matches!(source, Source::Llm) {
            Tone::Llm
        } else {
            Tone::Dim
        };
        item.notes.push(vec![span(desc, tone)]);
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        let values: Vec<String> = values.iter().map(compact_str).collect();
        if !values.is_empty() {
            item.notes.push(vec![
                span("one of ", Tone::Dim),
                span(values.join(" | "), Tone::Plain),
            ]);
        }
    }
    let allowed = schema
        .get("allowed_values")
        .and_then(|a| str_field(a, "dynamic_variable"))
        .or_else(|| str_field(schema, "allowed_values_dynamic_variable").filter(|v| !v.is_empty()));
    if let Some(var) = allowed {
        item.notes.push(vec![
            span("must be in ", Tone::Dim),
            span(format!("{{{{{var}}}}}"), Tone::Var),
        ]);
    }
    if let Some(constraints) = schema.get("required_constraints").filter(|c| c.is_object()) {
        for (key, label) in [("all_of", "requires all of"), ("any_of", "requires one of")] {
            let groups: Vec<String> = constraints
                .get(key)
                .and_then(Value::as_array)
                .map(|gs| {
                    gs.iter()
                        .filter_map(|g| g.get("required").and_then(Value::as_array))
                        .map(|fields| {
                            let fields: Vec<&str> =
                                fields.iter().filter_map(Value::as_str).collect();
                            format!("({})", fields.join(" + "))
                        })
                        .collect()
                })
                .unwrap_or_default();
            if !groups.is_empty() {
                item.notes.push(vec![
                    span(format!("{label} "), Tone::Dim),
                    span(groups.join(" | "), Tone::Plain),
                ]);
            }
        }
    }

    let inner_inherited = inherited || (own_source && !matches!(source, Source::Llm));
    if has_props {
        item.children = properties(key, schema, side, inner_inherited);
    } else if let Some(items) = schema.get("items").filter(|i| i.is_object()) {
        item.children.push(param(
            &format!("{key}/[]"),
            "[]",
            items,
            None,
            side,
            inner_inherited,
        ));
    }
    item
}

/// `string`, `integer`, `string|null`, `array<string>`, `object`.
fn type_label(schema: &Value) -> String {
    let base = match schema.get("type") {
        Some(Value::String(t)) => t.clone(),
        Some(Value::Array(ts)) => ts
            .iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join("|"),
        _ if schema.get("properties").is_some() => "object".to_string(),
        _ => "any".to_string(),
    };
    match schema.get("items").map(type_label) {
        Some(inner) if base == "array" => format!("array<{inner}>"),
        _ => base,
    }
}

fn description(schema: &Value) -> Option<String> {
    str_field(schema, "description")
        .filter(|d| !d.trim().is_empty())
        .map(one_line)
}

// ── System tools ────────────────────────────────────────────────────

fn system_items(tool: &Value) -> Vec<Item> {
    let empty = Value::Null;
    let params = tool.get("params").unwrap_or(&empty);
    let mut items = Vec::new();
    match str_field(params, "system_tool_type") {
        Some("transfer_to_agent") => {
            let mut section = Item::section("transfers", "Transfers");
            for (i, t) in array(params.get("transfers")).enumerate() {
                let agent = str_field(t, "agent_id").unwrap_or("?");
                let mut item = Item::new(
                    format!("transfers/{i}"),
                    condition_head(t, vec![span(format!("agent {agent}"), Tone::Strong)]),
                    "t_transfer",
                );
                let mut details = Vec::new();
                for (key, label) in [("branch_id", "branch"), ("node_id", "node")] {
                    if let Some(v) = str_field(t, key) {
                        details.push(format!("{label} {v}"));
                    }
                }
                if let Some(ms) = t
                    .get("delay_ms")
                    .and_then(Value::as_i64)
                    .filter(|ms| *ms > 0)
                {
                    details.push(format!("after {ms}ms"));
                }
                for (key, label) in [
                    (
                        "enable_transferred_agent_first_message",
                        "with its first message",
                    ),
                    ("preserve_voice_settings", "keeps the voice"),
                    ("preserve_client_tts_overrides", "keeps TTS overrides"),
                ] {
                    if t.get(key).and_then(Value::as_bool) == Some(true) {
                        details.push(label.to_string());
                    }
                }
                if !details.is_empty() {
                    item.notes.push(vec![span(details.join(" · "), Tone::Dim)]);
                }
                if let Some(message) = str_field(t, "transfer_message").filter(|m| !m.is_empty()) {
                    item.notes.push(vec![
                        span("says ", Tone::Dim),
                        span(format!("\"{}\"", one_line(message)), Tone::Plain),
                    ]);
                }
                section.children.push(item);
            }
            items.push(section);
        }
        Some("transfer_to_number") => {
            let transfers: Vec<&Value> = array(params.get("transfers")).collect();
            if !transfers.is_empty() {
                let mut section = Item::section("transfers", "Transfers");
                for (i, t) in transfers.into_iter().enumerate() {
                    let destination = destination(t.get("transfer_destination"));
                    let mut item = Item::new(
                        format!("transfers/{i}"),
                        condition_head(t, destination),
                        "t_transfer",
                    );
                    let mut details = vec![format!(
                        "{} transfer",
                        str_field(t, "transfer_type").unwrap_or("conference")
                    )];
                    if t.get("sip_refer_play_dialtone").and_then(Value::as_bool) == Some(true) {
                        details.push("plays a dial tone".to_string());
                    }
                    let headers = array(t.get("custom_sip_headers")).count();
                    if headers > 0 {
                        details.push(format!("{headers} SIP header(s)"));
                    }
                    item.notes.push(vec![span(details.join(" · "), Tone::Dim)]);
                    section.children.push(item);
                }
                items.push(section);
            }
            let rules: Vec<&Value> = array(params.get("transfer_rules")).collect();
            if !rules.is_empty() {
                let mut section = Item::section("rules", "Transfer rules");
                for (i, rule) in rules.into_iter().enumerate() {
                    let mut item = Item::new(
                        format!("rules/{i}"),
                        condition_head(rule, Vec::new()),
                        "t_transfer",
                    );
                    for (j, provider) in array(rule.get("provider_configs")).enumerate() {
                        let empty = Value::Null;
                        let config = provider.get("config").unwrap_or(&empty);
                        let mut head = vec![
                            span(str_field(provider, "type").unwrap_or("?"), Tone::Strong),
                            span(
                                format!(" {}", str_field(config, "type").unwrap_or("?")),
                                Tone::Dim,
                            ),
                            span(" → ", Tone::Dim),
                        ];
                        head.extend(destination(config.get("transfer_destination")));
                        item.children
                            .push(Item::new(format!("rules/{i}/{j}"), head, "t_transfer"));
                    }
                    section.children.push(item);
                }
                items.push(section);
            }
            if params.get("enable_client_message").and_then(Value::as_bool) == Some(false) {
                items.push(Item::new(
                    "client_message",
                    vec![span(
                        "No message to the caller while transferring",
                        Tone::Dim,
                    )],
                    "t_section",
                ));
            }
        }
        _ => {}
    }
    // Anything else in params, for system tools with simple settings.
    if let Some(obj) = params.as_object() {
        let skip = [
            "system_tool_type",
            "transfers",
            "transfer_rules",
            "enable_client_message",
        ];
        let rest: Vec<(&String, &Value)> = obj
            .iter()
            .filter(|(k, _)| !skip.contains(&k.as_str()))
            .collect();
        if !rest.is_empty() {
            let mut section = Item::section("params", "Params");
            for (k, v) in rest {
                section.children.push(Item::new(
                    format!("params/{k}"),
                    vec![
                        span(k.clone(), Tone::Strong),
                        span(" = ", Tone::Dim),
                        span(compact(v), Tone::Constant),
                    ],
                    "t_const",
                ));
            }
            items.push(section);
        }
    }
    items
}

/// `"condition" → destination`.
fn condition_head(transfer: &Value, destination: Vec<Span>) -> Vec<Span> {
    let condition = str_field(transfer, "condition").unwrap_or("");
    let mut head = vec![span(format!("\"{}\"", one_line(condition)), Tone::Llm)];
    if !destination.is_empty() {
        head.push(span(" → ", Tone::Dim));
        head.extend(destination);
    }
    head
}

/// A phone or SIP destination, in either the legacy (`phone_number` /
/// `sip_uri`) or transfer-rule (`destination`) shape.
fn destination(dest: Option<&Value>) -> Vec<Span> {
    let Some(dest) = dest.filter(|d| d.is_object()) else {
        return vec![span("?", Tone::Warn)];
    };
    let value = str_field(dest, "destination")
        .or_else(|| str_field(dest, "phone_number"))
        .or_else(|| str_field(dest, "sip_uri"))
        .unwrap_or("?");
    match str_field(dest, "type").unwrap_or("") {
        "phone" => vec![span(format!("phone {value}"), Tone::Strong)],
        "sip_uri" => vec![span(format!("SIP {value}"), Tone::Strong)],
        "phone_dynamic_variable" => vec![
            span("phone ", Tone::Strong),
            span(format!("{{{{{value}}}}}"), Tone::Var),
        ],
        "sip_uri_dynamic_variable" => vec![
            span("SIP ", Tone::Strong),
            span(format!("{{{{{value}}}}}"), Tone::Var),
        ],
        other => vec![span(format!("{other} {value}"), Tone::Strong)],
    }
}

fn client_items(tool: &Value) -> Vec<Item> {
    let Some(params) = tool.get("parameters").filter(|p| p.is_object()) else {
        return Vec::new();
    };
    let mut section = Item::section("params", "Parameters");
    if let Some(desc) = description(params) {
        section.notes.push(vec![span(desc, Tone::Dim)]);
    }
    section.children = properties("params", params, Side::Request, false);
    if section.children.is_empty() {
        return Vec::new();
    }
    vec![section]
}

// ── Helpers ─────────────────────────────────────────────────────────

fn array(value: Option<&Value>) -> impl Iterator<Item = &Value> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|a| a.iter())
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A JSON value on one line: strings quoted.
fn compact(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

/// An enum value: strings bare, anything else as JSON.
fn compact_str(value: &Value) -> String {
    value.as_str().map_or_else(|| compact(value), String::from)
}

// ── Terminal ────────────────────────────────────────────────────────

/// Narrowest the text after a tree prefix is wrapped to, however deep.
const MIN_TEXT_WIDTH: usize = 24;

/// Draw tools as indented trees. `width` wraps long lines to fit; `None`
/// leaves them whole. `color` adds ANSI colour by what each part is.
fn render_terminal(views: &[ToolView], color: bool, width: Option<usize>) -> String {
    let mut out = String::new();
    for (i, view) in views.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        let title = vec![
            span(view.name.clone(), Tone::Strong),
            span(format!(" · {}", view.kind), Tone::Dim),
        ];
        push_wrapped(&mut out, "", "", &title, color, width);
        for line in &view.summary {
            push_wrapped(&mut out, "", "", line, color, width);
        }
        if !view.items.is_empty() {
            out.push('\n');
        }
        let last = view.items.len().saturating_sub(1);
        for (j, item) in view.items.iter().enumerate() {
            // Top-level items are roots of their own trees.
            push_item(&mut out, item, "", None, j == last, color, width);
        }
    }
    out
}

/// `prefix` is the guide drawn for the item's ancestors; `last` is `None`
/// for a top-level item, which has no branch of its own.
fn push_item(
    out: &mut String,
    item: &Item,
    prefix: &str,
    last: Option<bool>,
    is_last_root: bool,
    color: bool,
    width: Option<usize>,
) {
    let (branch, inner) = match last {
        None => (String::new(), String::new()),
        Some(true) => (format!("{prefix}└── "), format!("{prefix}    ")),
        Some(false) => (format!("{prefix}├── "), format!("{prefix}│   ")),
    };
    let child_guide = if item.children.is_empty() {
        "    "
    } else {
        "│   "
    };
    let note_prefix = format!("{inner}{child_guide}");
    push_wrapped(out, &branch, &note_prefix, &item.head, color, width);
    for note in &item.notes {
        push_wrapped(out, &note_prefix, &note_prefix, note, color, width);
    }
    let n = item.children.len();
    for (i, child) in item.children.iter().enumerate() {
        push_item(out, child, &inner, Some(i + 1 == n), false, color, width);
    }
    if last.is_none() && !is_last_root {
        out.push('\n');
    }
}

/// One logical line, wrapped at word boundaries to `width`: the first line
/// after `first`, the rest after `rest`. Words are never split, and one too
/// wide for any line is left to overflow where it is.
fn push_wrapped(
    out: &mut String,
    first: &str,
    rest: &str,
    spans: &[Span],
    color: bool,
    width: Option<usize>,
) {
    let room = width.map(|w| w.saturating_sub(rest.chars().count()).max(MIN_TEXT_WIDTH));
    // Words, each keeping the tone of its span; a span boundary inside a
    // word does not split it.
    let mut words: Vec<Vec<Span>> = Vec::new();
    let mut gap_before: Vec<bool> = Vec::new();
    let mut pending_gap = false;
    for s in spans {
        let chars = s.text.chars();
        let mut current = String::new();
        let flush = |current: &mut String,
                     words: &mut Vec<Vec<Span>>,
                     gap_before: &mut Vec<bool>,
                     pending_gap: &mut bool| {
            if current.is_empty() {
                return;
            }
            let text = std::mem::take(current);
            match words.last_mut() {
                Some(word) if !*pending_gap && !gap_before.is_empty() => {
                    word.push(span(text, s.tone))
                }
                _ => {
                    words.push(vec![span(text, s.tone)]);
                    gap_before.push(*pending_gap);
                }
            }
            *pending_gap = false;
        };
        for c in chars {
            if c == ' ' {
                flush(&mut current, &mut words, &mut gap_before, &mut pending_gap);
                pending_gap = true;
            } else {
                current.push(c);
            }
        }
        flush(&mut current, &mut words, &mut gap_before, &mut pending_gap);
    }
    let mut line = String::from(first);
    let mut used = 0usize;
    for (word, gap) in words.iter().zip(&gap_before) {
        let len: usize = word.iter().map(|s| s.text.chars().count()).sum();
        let sep = usize::from(*gap && used > 0);
        if let Some(room) = room {
            // A word too wide for any line stays put rather than leaving a
            // short line behind it.
            if used > 0 && used + sep + len > room && len <= room {
                out.push_str(line.trim_end());
                out.push('\n');
                line = String::from(rest);
                used = 0;
            }
        }
        if *gap && used > 0 {
            line.push(' ');
            used += 1;
        }
        for s in word {
            line.push_str(&paint(&s.text, s.tone, color));
        }
        used += len;
    }
    out.push_str(line.trim_end());
    out.push('\n');
}

fn paint(text: &str, tone: Tone, color: bool) -> String {
    match tone.ansi().filter(|_| color) {
        Some(code) => format!("\u{1b}[{code}m{text}\u{1b}[0m"),
        None => text.to_string(),
    }
}

// ── Mermaid ─────────────────────────────────────────────────────────

/// `classDef` lines, in emission order.
const CLASS_DEFS: &[(&str, &str)] = &[
    ("t_tool", "fill:#fef3c7,stroke:#d97706,color:#78350f"),
    ("t_request", "fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e"),
    ("t_section", "fill:#f1f5f9,stroke:#475569,color:#0f172a"),
    ("t_param", "fill:#ffffff,stroke:#94a3b8,color:#0f172a"),
    ("t_llm", "fill:#cffafe,stroke:#0891b2,color:#164e63"),
    ("t_var", "fill:#ede9fe,stroke:#7c3aed,color:#4c1d95"),
    ("t_const", "fill:#fef9c3,stroke:#ca8a04,color:#713f12"),
    ("t_system", "fill:#dbeafe,stroke:#2563eb,color:#1e3a8a"),
    (
        "t_omitted",
        "fill:#f3f4f6,stroke:#9ca3af,stroke-dasharray:4 2,color:#6b7280",
    ),
    ("t_secret", "fill:#fee2e2,stroke:#dc2626,color:#7f1d1d"),
    ("t_env", "fill:#d1fae5,stroke:#059669,color:#064e3b"),
    ("t_transfer", "fill:#fce7f3,stroke:#db2777,color:#831843"),
    (
        "t_warn",
        "fill:#fee2e2,stroke:#dc2626,stroke-dasharray:4 2,color:#7f1d1d",
    ),
];

/// Tools as one Mermaid flowchart, a tree per tool, left to right. Always
/// ends with a newline.
fn render_mermaid(views: &[ToolView]) -> String {
    // Every node's key, prefixed by its tool so several tools can share a
    // chart. Collected first so the IDs are assigned in a stable order.
    let mut keys: Vec<String> = Vec::new();
    for (i, view) in views.iter().enumerate() {
        let root = root_key(i, view);
        keys.push(root.clone());
        fn collect(items: &[Item], root: &str, keys: &mut Vec<String>) {
            for item in items {
                keys.push(format!("{root}/{}", item.key));
                collect(&item.children, root, keys);
            }
        }
        collect(&view.items, &root, &mut keys);
    }
    let ids = MermaidIds::new(keys.iter().map(String::as_str));

    let mut out = String::from("flowchart LR\n");
    let mut edges = String::new();
    let mut classes: BTreeMap<&'static str, Vec<String>> = BTreeMap::new();
    for (i, view) in views.iter().enumerate() {
        let root = root_key(i, view);
        let mut lines = vec![format!("{} · {}", view.kind, view.name)];
        lines.extend(view.summary.iter().map(|l| plain_text(l)));
        out.push_str(&format!("    {}[[{}]]\n", ids.get(&root), label(&lines)));
        classes
            .entry("t_tool")
            .or_default()
            .push(ids.get(&root).to_string());
        fn emit(
            items: &[Item],
            parent: &str,
            root: &str,
            ids: &MermaidIds,
            out: &mut String,
            edges: &mut String,
            classes: &mut BTreeMap<&'static str, Vec<String>>,
        ) {
            for item in items {
                let key = format!("{root}/{}", item.key);
                let id = ids.get(&key);
                let mut lines = vec![plain_text(&item.head)];
                lines.extend(item.notes.iter().map(|n| plain_text(n)));
                out.push_str(&format!("    {id}[{}]\n", label(&lines)));
                edges.push_str(&format!("    {} --> {id}\n", ids.get(parent)));
                classes.entry(item.class).or_default().push(id.to_string());
                emit(&item.children, &key, root, ids, out, edges, classes);
            }
        }
        emit(
            &view.items,
            &root,
            &root,
            &ids,
            &mut out,
            &mut edges,
            &mut classes,
        );
    }
    out.push_str(&edges);
    for (class, style) in CLASS_DEFS {
        if let Some(members) = classes.get(class) {
            out.push_str(&format!("    classDef {class} {style}\n"));
            out.push_str(&format!("    class {} {class}\n", members.join(",")));
        }
    }
    out
}

fn root_key(i: usize, view: &ToolView) -> String {
    // The index keeps two tools of the same name apart; the name keeps IDs
    // readable.
    if i == 0 {
        view.name.clone()
    } else {
        format!("{}_{i}", view.name)
    }
}

/// Longest first line of a Mermaid label. It names the node (a URL, a
/// parameter and its source), so it gets more room than the notes under it.
const HEAD_LABEL_MAX: usize = 100;

/// A quoted Mermaid label: each line truncated and escaped, joined by
/// `<br/>`, which mermaid.js renders as a line break.
fn label(lines: &[String]) -> String {
    let escaped: Vec<String> = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let max = if i == 0 {
                HEAD_LABEL_MAX
            } else {
                NODE_LABEL_MAX
            };
            let q = quote(&truncate(l, max));
            q[1..q.len() - 1].to_string()
        })
        .collect();
    format!("\"{}\"", escaped.join("<br/>"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ascii(tool: Value) -> String {
        render_terminal(&[view(&tool)], false, None)
    }

    fn mermaid(tool: Value) -> String {
        render_mermaid(&[view(&tool)])
    }

    #[test]
    fn url_parts_finds_placeholders_and_templates_in_order() {
        let parts =
            url_parts("https://{{system__env_api_host}}/{tenant}/orders/{order_id}?x={tenant}");
        assert_eq!(parts.placeholders, ["tenant", "order_id"]);
        assert_eq!(parts.templates, ["system__env_api_host"]);
        assert_eq!(
            url_parts("https://a/{unclosed").placeholders,
            Vec::<String>::new()
        );
    }

    #[test]
    fn every_value_source_is_labelled() {
        let out = ascii(json!({
            "type": "client", "name": "t",
            "parameters": {
                "type": "object",
                "properties": {
                    "a_llm": { "type": "string", "description": "Picked by the LLM", "enum": ["x", "y"] },
                    "b_var": { "type": "string", "dynamic_variable": "user_id" },
                    "c_const": { "type": "integer", "constant_value": 3 },
                    "d_sys": { "type": "string", "is_system_provided": true },
                    "e_omit": { "type": "boolean", "is_omitted": true },
                    "f_guard": { "type": "string", "description": "A workspace", "allowed_values": { "dynamic_variable": "allowed_ids" } },
                    "g_api_defaults": { "type": "string", "description": "Status", "constant_value": "", "dynamic_variable": "", "is_omitted": false, "is_system_provided": false }
                },
                "required": ["a_llm", "c_const"]
            }
        }));
        assert!(out.contains("a_llm: string required [LLM]"), "{out}");
        assert!(out.contains("Picked by the LLM"), "{out}");
        assert!(out.contains("one of x | y"), "{out}");
        assert!(
            out.contains("b_var: string optional [var user_id]"),
            "{out}"
        );
        assert!(
            out.contains("c_const: integer required [constant 3]"),
            "{out}"
        );
        assert!(out.contains("d_sys: string optional [system]"), "{out}");
        assert!(out.contains("e_omit: boolean optional [omitted]"), "{out}");
        assert!(out.contains("must be in {{allowed_ids}}"), "{out}");
        // The API's `""` and `false` defaults are not sources.
        assert!(
            out.contains("g_api_defaults: string optional [LLM]"),
            "{out}"
        );
    }

    #[test]
    fn a_whole_object_from_a_variable_has_no_sources_inside() {
        let out = ascii(json!({
            "type": "client", "name": "t",
            "parameters": { "type": "object", "properties": {
                "user": { "type": "object", "dynamic_variable": "profile",
                          "properties": { "id": { "type": "string", "description": "x" } } }
            } }
        }));
        assert!(out.contains("user: object optional [var profile]"), "{out}");
        assert!(out.contains("── id: string\n"), "{out}");
    }

    #[test]
    fn arrays_show_their_items() {
        let out = ascii(json!({
            "type": "client", "name": "t",
            "parameters": { "type": "object", "properties": {
                "tags": { "type": "array", "description": "Tags", "items": { "type": "string", "description": "A tag" } }
            } }
        }));
        assert!(out.contains("tags: array<string> optional\n"), "{out}");
        assert!(out.contains("── []: string [LLM]"), "{out}");
    }

    #[test]
    fn path_params_flag_mismatches_with_the_url() {
        let out = ascii(json!({
            "type": "webhook", "name": "t",
            "api_schema": {
                "url": "https://x/{a}/{tenant}/{b}", "method": "GET",
                "auth_resolved_params": ["tenant"],
                "path_params_schema": {
                    "a": { "type": "string", "description": "A" },
                    "stray": { "type": "string", "description": "S" }
                }
            }
        }));
        assert!(out.contains("a: string required [LLM]"), "{out}");
        assert!(out.contains("tenant [from auth connection]"), "{out}");
        assert!(out.contains("b [no schema in path_params_schema]"), "{out}");
        assert!(out.contains("{stray} is not in the URL"), "{out}");
    }

    #[test]
    fn header_sources_are_labelled() {
        let out = ascii(json!({
            "type": "webhook", "name": "t",
            "api_schema": {
                "url": "https://x", "method": "POST",
                "request_headers": {
                    "A": "plain {{user_id}}",
                    "B": { "secret_id": "sec_1" },
                    "C": { "variable_name": "caller" },
                    "D": { "env_var_label": "key" }
                },
                "auth_connection": { "auth_connection_id": "auth_1" }
            }
        }));
        assert!(out.contains("A = \"plain {{user_id}}\""), "{out}");
        assert!(out.contains("B [secret sec_1]"), "{out}");
        assert!(out.contains("C [var caller]"), "{out}");
        assert!(out.contains("D [env key]"), "{out}");
        assert!(out.contains("Auth [auth connection auth_1]"), "{out}");
    }

    #[test]
    fn long_lines_wrap_under_their_branch() {
        let tool = json!({
            "type": "client", "name": "t",
            "parameters": { "type": "object", "properties": {
                "a": { "type": "string", "description": "one two three four five six seven eight nine ten eleven twelve" },
                "b": { "type": "string", "description": "x" }
            } }
        });
        let out = render_terminal(&[view(&tool)], false, Some(40));
        for line in out.lines() {
            assert!(line.chars().count() <= 40, "{line:?}\n{out}");
        }
        assert!(out.contains("│       one two"), "{out}");
    }

    #[test]
    fn colour_is_only_added_when_asked() {
        let tool = json!({ "type": "client", "name": "t" });
        assert!(!render_terminal(&[view(&tool)], false, None).contains('\u{1b}'));
        assert!(render_terminal(&[view(&tool)], true, None).contains('\u{1b}'));
    }

    #[test]
    fn mermaid_labels_cannot_break_out() {
        let out = mermaid(json!({
            "type": "client", "name": "t",
            "description": "Say \"hi\" | <b>",
        }));
        assert!(out.contains("#quot;hi#quot; #124; #lt;b#gt;"), "{out}");
    }

    #[test]
    fn wrapped_and_bare_configs_are_the_same_tool() {
        let bare = json!({ "type": "client", "name": "t" });
        let wrapped = json!({ "id": "tool_1", "tool_config": bare.clone() });
        assert_eq!(
            tools_in(&bare, "x").unwrap(),
            tools_in(&wrapped, "x").unwrap()
        );
    }

    #[test]
    fn an_agent_config_yields_its_system_and_inline_tools() {
        let agent = json!({
            "name": "Support",
            "conversation_config": { "agent": { "prompt": {
                "tools": [{ "type": "client", "name": "inline" }],
                "built_in_tools": {
                    "end_call": { "name": "end_call", "type": "system", "params": { "system_tool_type": "end_call" } },
                    "skip_turn": null
                }
            } } }
        });
        let tools = tools_in(&agent, "a.json").unwrap();
        let names: Vec<&str> = tools.iter().filter_map(|t| str_field(t, "name")).collect();
        assert_eq!(names, ["inline", "end_call"]);
        assert!(tools_in(&json!({ "foo": 1 }), "x").is_err());
    }

    const FIXTURES: &[(&str, &str, &str, &str)] = &[
        (
            "webhook",
            include_str!("fixtures/tool_diagram/webhook.json"),
            include_str!("fixtures/tool_diagram/webhook.mmd"),
            include_str!("fixtures/tool_diagram/webhook.txt"),
        ),
        (
            "client",
            include_str!("fixtures/tool_diagram/client.json"),
            include_str!("fixtures/tool_diagram/client.mmd"),
            include_str!("fixtures/tool_diagram/client.txt"),
        ),
        (
            "transfers",
            include_str!("fixtures/tool_diagram/transfers.json"),
            include_str!("fixtures/tool_diagram/transfers.mmd"),
            include_str!("fixtures/tool_diagram/transfers.txt"),
        ),
        (
            "transfer_rules",
            include_str!("fixtures/tool_diagram/transfer_rules.json"),
            include_str!("fixtures/tool_diagram/transfer_rules.mmd"),
            include_str!("fixtures/tool_diagram/transfer_rules.txt"),
        ),
    ];

    /// Representative tools, rendered in full in both formats. A change here
    /// changes every diagram users have committed, so it should be deliberate.
    /// `UPDATE_SNAPSHOTS=1 cargo test tool_diagram` rewrites them.
    #[test]
    fn fixtures_render_to_their_snapshots() {
        let update = std::env::var_os("UPDATE_SNAPSHOTS").is_some();
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("cli/elevenlabs/workflow/fixtures/tool_diagram");
        for (name, input, mmd, txt) in FIXTURES {
            let value: Value = serde_json::from_str(input).expect(name);
            let views: Vec<ToolView> = tools_in(&value, name)
                .expect(name)
                .iter()
                .map(view)
                .collect();
            let got_mmd = render_mermaid(&views);
            let got_txt = render_terminal(&views, false, None);
            if update {
                std::fs::write(dir.join(format!("{name}.mmd")), &got_mmd).unwrap();
                std::fs::write(dir.join(format!("{name}.txt")), &got_txt).unwrap();
                continue;
            }
            assert_eq!(got_mmd, *mmd, "fixture {name} (mermaid)");
            assert_eq!(got_txt, *txt, "fixture {name} (ascii)");
        }
    }

    /// The Mermaid has to parse: mmdflux draws every fixture.
    #[test]
    fn fixture_mermaid_parses() {
        for (name, input, _, _) in FIXTURES {
            let value: Value = serde_json::from_str(input).unwrap();
            let views: Vec<ToolView> = tools_in(&value, name).unwrap().iter().map(view).collect();
            super::super::mermaid::draw_text(&render_mermaid(&views), false, "x")
                .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        }
    }
}
