//! `agents workflow diagram` / `export`: an agent's workflow as a flowchart,
//! drawn in the terminal or printed as Mermaid source.
//!
//! Workflows are stored as raw wire JSON (`workflow.nodes` / `workflow.edges`,
//! both maps keyed by ID), which round-trips losslessly but is unreadable in
//! a diff. GitHub and most docs tooling render Mermaid natively, so this gives
//! reviewers a picture of the conversation flow with no extra tooling.
//! `diagram` draws the same flowchart in the terminal via mmdflux, a pure-Rust
//! Mermaid renderer, so a workflow can be read without leaving the shell.
//!
//! Local files only: no API key and no network, so it runs in CI on forks.
//! Rendering works on [`serde_json::Value`] rather than the generated types so
//! a node or condition type this build doesn't know still renders (generically)
//! instead of failing to deserialize.
//!
//! Output is deterministic — nodes and edges are emitted sorted by ID — so a
//! diagram only changes when the workflow does.

use std::collections::{BTreeMap, BTreeSet};
use std::io::IsTerminal;
use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::agents::{agent_display_name, find_agent, require_agents};
use super::{api, project};

/// Register the `agents workflow` command group.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents", "workflow"],
        clap::Command::new("diagram")
            .about("Draw an agent's workflow in the terminal (local files only)"),
        handle_diagram,
    )
    .command_under_typed_with(
        &["agents", "workflow"],
        clap::Command::new("export").about(
            "Print an agent's workflow as Mermaid source, to save or paste (local files only)",
        ),
        handle_export,
    )
}

#[derive(clap::Args)]
struct DiagramArgs {
    /// An agent config file, or an agent ID or name from agents.json.
    #[arg(required_unless_present = "all")]
    target: Option<String>,
    /// Draw every agent (and registered branch) in agents.json.
    #[arg(long, conflicts_with = "target")]
    all: bool,
}

fn handle_diagram(args: DiagramArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("agents.workflow.diagram");
    let color = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let Some(target) = &args.target else {
        for (i, workflow) in load_all()?.iter().enumerate() {
            if i > 0 {
                println!();
            }
            println!("── {} ({})\n", workflow.title, workflow.config_path);
            print!("{}", render_terminal(&workflow.workflow, color)?);
        }
        return Ok(());
    };
    let workflow = load_one(target)?;
    print!("{}", render_terminal(&workflow.workflow, color)?);
    Ok(())
}

#[derive(clap::Args)]
struct ExportArgs {
    /// An agent config file, or an agent ID or name from agents.json.
    #[arg(required_unless_present = "all")]
    target: Option<String>,
    /// Export every agent (and registered branch) in agents.json.
    #[arg(long, conflicts_with = "target")]
    all: bool,
    /// Wrap each diagram in a ```mermaid fence, ready to paste into Markdown.
    #[arg(long)]
    markdown: bool,
}

fn handle_export(args: ExportArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("agents.workflow.export");
    let Some(target) = &args.target else {
        return export_all(args.markdown);
    };
    let workflow = load_one(target)?;
    let diagram = render_mermaid(&workflow.workflow);
    if args.markdown {
        print!("{}", fence(&diagram));
    } else {
        print!("{diagram}");
    }
    Ok(())
}

fn export_all(markdown: bool) -> Result<(), CliError> {
    for (i, workflow) in load_all()?.iter().enumerate() {
        let diagram = render_mermaid(&workflow.workflow);
        if i > 0 {
            println!();
        }
        if markdown {
            println!("### {}\n", workflow.title);
            print!("{}", fence(&diagram));
        } else {
            // Several flowcharts in one stream are not one Mermaid document;
            // the comment marks where each starts. --markdown is the form
            // meant for this.
            println!("%% {} ({})", workflow.title, workflow.config_path);
            print!("{diagram}");
        }
    }
    Ok(())
}

// ── Loading ─────────────────────────────────────────────────────────

/// An agent's workflow, read from its config file.
struct LoadedWorkflow {
    /// The agent's display name, plus the branch for a branch config.
    title: String,
    config_path: String,
    workflow: Value,
}

/// Load one workflow from a config file path, or from an agent ID or name in
/// agents.json. Errors when the config has no workflow.
fn load_one(target: &str) -> Result<LoadedWorkflow, CliError> {
    // A path the user typed is read as-is; one taken from agents.json goes
    // through the project containment checks like every other index entry.
    let (config, config_path) = if Path::new(target).is_file() {
        (project::read_value(Path::new(target))?, target.to_string())
    } else {
        let registry = require_agents()?;
        let agent = find_agent(&registry, target).ok_or_else(|| {
            CliError::Validation(format!(
                "'{target}' is not a config file, or an agent ID or name in agents.json"
            ))
        })?;
        (
            project::read_value_in_project(&agent.config)?,
            agent.config.clone(),
        )
    };
    let workflow = workflow_of(&config).cloned().ok_or_else(|| {
        CliError::Validation(format!(
            "{config_path} has no workflow to draw (no workflow.nodes)"
        ))
    })?;
    let title = config
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(target)
        .to_string();
    Ok(LoadedWorkflow {
        title,
        config_path,
        workflow,
    })
}

/// Load every agent's (and registered branch's) workflow from agents.json,
/// skipping, with a note on stderr, configs that are unreadable or have none.
fn load_all() -> Result<Vec<LoadedWorkflow>, CliError> {
    let registry = require_agents()?;
    // agents.json order, then each agent's branches by name: both stable.
    let mut entries: Vec<(String, String)> = Vec::new();
    for agent in &registry.agents {
        let name = agent_display_name(&agent.config);
        entries.push((name.clone(), agent.config.clone()));
        for (branch, def) in agent.branches.iter().flatten() {
            entries.push((format!("{name} (branch '{branch}')"), def.config.clone()));
        }
    }

    let mut loaded = Vec::new();
    for (title, config_path) in entries {
        let config = match project::read_value_in_project(&config_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Skipping '{title}': {e}");
                continue;
            }
        };
        let Some(workflow) = workflow_of(&config).cloned() else {
            eprintln!("Skipping '{title}': no workflow ({config_path})");
            continue;
        };
        loaded.push(LoadedWorkflow {
            title,
            config_path,
            workflow,
        });
    }
    if loaded.is_empty() {
        eprintln!("No agents in agents.json have a workflow to draw.");
    }
    Ok(loaded)
}

/// The config's workflow, if it has any nodes to draw.
fn workflow_of(config: &Value) -> Option<&Value> {
    let workflow = config.get("workflow")?;
    let has_nodes = workflow
        .get("nodes")
        .and_then(Value::as_object)
        .is_some_and(|nodes| !nodes.is_empty());
    has_nodes.then_some(workflow)
}

fn fence(diagram: &str) -> String {
    format!("```mermaid\n{diagram}```\n")
}

// ── Rendering ───────────────────────────────────────────────────────

/// Longest node label, in characters, before it is cut with an ellipsis.
const NODE_LABEL_MAX: usize = 60;
/// Longest edge label. LLM conditions are full sentences; the diagram only
/// needs enough to tell edges apart.
const EDGE_LABEL_MAX: usize = 40;
/// Line width node labels are wrapped to in the terminal. Side-by-side nodes
/// are what make a diagram wide, so they are kept narrow.
const TERMINAL_WRAP: usize = 24;

/// Who the Mermaid source is for.
#[derive(Clone, Copy, PartialEq)]
enum Target {
    /// mermaid.js (GitHub, docs): one-line labels, entity-escaped, with edge
    /// conditions inline.
    Export,
    /// mmdflux, drawing in a terminal: plain-text labels wrapped onto lines,
    /// and edge conditions replaced by numbered keys explained in a legend.
    /// Inline conditions are what make a terminal diagram too wide to read.
    Terminal,
}

/// Mermaid source, plus the legend lines for [`Target::Terminal`].
struct Built {
    source: String,
    legend: Vec<String>,
}

/// How each node type is drawn: Mermaid shape delimiters and its class.
/// Class names are prefixed because `end` is a Mermaid keyword.
struct Style {
    open: &'static str,
    close: &'static str,
    class: &'static str,
}

fn style_for(node_type: &str, target: Target) -> Style {
    let (open, close, class) = match node_type {
        "start" => ("([", "])", "t_start"),
        "end" => ("([", "])", "t_end"),
        "say" => ("(", ")", "t_say"),
        "tool" => ("[[", "]]", "t_tool"),
        // mmdflux has no parallelogram and prints its delimiters as text; a
        // cylinder (a data store) suits a state update just as well.
        "update_state" if target == Target::Terminal => ("[(", ")]", "t_state"),
        "update_state" => ("[/", "/]", "t_state"),
        "override_agent" => ("[", "]", "t_subagent"),
        "standalone_agent" | "phone_number" => ("{{", "}}", "t_transfer"),
        _ => ("[", "]", "t_unknown"),
    };
    Style { open, close, class }
}

/// `classDef` lines in emission order. Light fills with dark text read on
/// both GitHub's light and dark themes.
const CLASS_DEFS: &[(&str, &str)] = &[
    ("t_start", "fill:#d1fae5,stroke:#059669,color:#064e3b"),
    ("t_end", "fill:#fee2e2,stroke:#dc2626,color:#7f1d1d"),
    ("t_say", "fill:#e0f2fe,stroke:#0284c7,color:#0c4a6e"),
    ("t_tool", "fill:#fef3c7,stroke:#d97706,color:#78350f"),
    ("t_state", "fill:#ede9fe,stroke:#7c3aed,color:#4c1d95"),
    ("t_subagent", "fill:#f1f5f9,stroke:#475569,color:#0f172a"),
    ("t_transfer", "fill:#fce7f3,stroke:#db2777,color:#831843"),
    (
        "t_unknown",
        "fill:#f3f4f6,stroke:#6b7280,stroke-dasharray:4 2,color:#111827",
    ),
    // Terminal only: the numbered condition keys.
    ("t_key", "fill:#f3f4f6,stroke:#6b7280,color:#111827"),
];

/// Render a workflow (`{"nodes": {...}, "edges": {...}}`) as a Mermaid
/// flowchart. Always ends with a newline.
pub(super) fn render_mermaid(workflow: &Value) -> String {
    build(workflow, Target::Export).source
}

/// Draw a workflow as box-drawing text for a terminal, followed by a legend
/// of the edge conditions. `color` adds ANSI colour from the node classes.
fn render_terminal(workflow: &Value, color: bool) -> Result<String, CliError> {
    let built = build(workflow, Target::Terminal);
    let mut config = mmdflux::RenderConfig::default();
    if color {
        config.text_color_mode = mmdflux::TextColorMode::Ansi;
    }
    let mut out = mmdflux::render_diagram(&built.source, mmdflux::OutputFormat::Text, &config)
        .map_err(|e| {
            CliError::Other(anyhow::anyhow!(
                "Could not draw this workflow in the terminal ({e}). \
                 'elevenlabs agents workflow export' prints it as Mermaid instead."
            ))
        })?;
    if !out.ends_with('\n') {
        out.push('\n');
    }
    if !built.legend.is_empty() {
        out.push('\n');
        for line in &built.legend {
            out.push_str(line);
            out.push('\n');
        }
    }
    Ok(out)
}

fn build(workflow: &Value, target: Target) -> Built {
    let empty = serde_json::Map::new();
    let nodes = workflow
        .get("nodes")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let edges = workflow
        .get("edges")
        .and_then(Value::as_object)
        .unwrap_or(&empty);
    let nodes: BTreeMap<&str, &Value> = nodes.iter().map(|(k, v)| (k.as_str(), v)).collect();
    let edges: BTreeMap<&str, &Value> = edges.iter().map(|(k, v)| (k.as_str(), v)).collect();

    // Endpoints an edge names but no node defines. Drawn as unknown nodes so
    // the dangling edge is visible rather than silently dropped.
    let missing: BTreeSet<&str> = edges
        .values()
        .flat_map(|e| [str_field(e, "source"), str_field(e, "target")])
        .flatten()
        .filter(|id| !nodes.contains_key(id))
        .collect();

    let ids = MermaidIds::new(nodes.keys().copied().chain(missing.iter().copied()));
    let mut out = String::from("flowchart TD\n");
    let mut classes: BTreeMap<&'static str, Vec<&str>> = BTreeMap::new();

    for (id, node) in &nodes {
        let node_type = str_field(node, "type").unwrap_or("unknown");
        let style = style_for(node_type, target);
        let label = node_label(id, node_type, node).render(target);
        out.push_str(&node_line(
            ids.get(id),
            style.open,
            &label,
            style.close,
            target,
        ));
        classes.entry(style.class).or_default().push(ids.get(id));
    }
    for id in &missing {
        out.push_str(&format!("    %% no node defines '{}'\n", comment_safe(id)));
        let label = NodeLabel {
            kind: None,
            text: format!("missing: {id}"),
        };
        out.push_str(&format!("    {}[{}]\n", ids.get(id), label.render(target)));
        classes.entry("t_unknown").or_default().push(ids.get(id));
    }

    let mut legend = Vec::new();
    let mut keys = Vec::new();
    // One link with its condition. For export the (truncated) condition is
    // the edge label. In the terminal it becomes a numbered key node sitting
    // on the edge, explained in full in the legend: as an edge label,
    // mmdflux can place a key beside the wrong edge of a fan-out, while a
    // node has to sit on its own edge's path.
    let mut link = |from: &str, to: &str, label: Option<String>, backward: bool| {
        let (line, head) = if backward {
            ("-.-", "-.->")
        } else {
            ("---", "-->")
        };
        match (label, target) {
            (None, _) => out.push_str(&format!("    {from} {head} {to}\n")),
            (Some(label), Target::Export) => out.push_str(&format!(
                "    {from} {head}|{}| {to}\n",
                quote(&truncate(&label, EDGE_LABEL_MAX))
            )),
            (Some(label), Target::Terminal) => {
                let n = legend.len() + 1;
                let text = label.split_whitespace().collect::<Vec<_>>().join(" ");
                let direction = if backward { " (going back)" } else { "" };
                legend.push(format!("{n:>3}  {text}{direction}"));
                // `k_` cannot collide with the `n_` node IDs.
                let key = format!("k_{n}");
                out.push_str(&format!("    {from} {line} {key}([\"{n}\"]) {head} {to}\n"));
                keys.push(key);
            }
        }
    };
    // Export is sorted by edge ID, so it diffs cleanly. The terminal follows
    // the flow, so keys are numbered roughly top to bottom.
    let ordered: Vec<&Value> = match target {
        Target::Export => edges.values().copied().collect(),
        Target::Terminal => flow_order(&nodes, &edges),
    };
    for edge in ordered {
        let (Some(from), Some(to)) = (str_field(edge, "source"), str_field(edge, "target")) else {
            continue;
        };
        let (from, to) = (ids.get(from), ids.get(to));
        let forward = condition(edge, "forward_condition");
        let backward = condition(edge, "backward_condition");
        // An edge with neither condition is still a connection; draw it
        // forward. One with only a backward condition is drawn backward only.
        if forward.is_some() || backward.is_none() {
            link(from, to, forward.and_then(condition_label), false);
        }
        if let Some(cond) = backward {
            link(to, from, condition_label(cond), true);
        }
    }
    let key_refs: Vec<&str> = keys.iter().map(String::as_str).collect();
    if !key_refs.is_empty() {
        classes.insert("t_key", key_refs);
    }

    for (class, style) in CLASS_DEFS {
        if let Some(members) = classes.get(class) {
            out.push_str(&format!("    classDef {class} {style}\n"));
            let mut members = members.clone();
            members.sort_unstable();
            out.push_str(&format!("    class {} {class}\n", members.join(",")));
        }
    }
    Built {
        source: out,
        legend,
    }
}

/// Edges in the order the conversation reaches them: by the breadth-first
/// depth of their source from the start node(s), then by edge ID. Edges from
/// unreachable nodes come last. Deterministic, like the ID order.
fn flow_order<'a>(
    nodes: &BTreeMap<&str, &Value>,
    edges: &BTreeMap<&'a str, &'a Value>,
) -> Vec<&'a Value> {
    let mut depth: BTreeMap<&str, usize> = BTreeMap::new();
    let mut queue = std::collections::VecDeque::new();
    for (id, node) in nodes {
        if str_field(node, "type") == Some("start") {
            depth.insert(id, 0);
            queue.push_back(*id);
        }
    }
    while let Some(id) = queue.pop_front() {
        let next = depth[id] + 1;
        for edge in edges.values() {
            if str_field(edge, "source") != Some(id) {
                continue;
            }
            if let Some(to) = str_field(edge, "target") {
                if !depth.contains_key(to) {
                    depth.insert(to, next);
                    queue.push_back(to);
                }
            }
        }
    }
    let mut ordered: Vec<(&str, &Value)> = edges.iter().map(|(id, e)| (*id, *e)).collect();
    ordered.sort_by_key(|(id, edge)| {
        let source = str_field(edge, "source").and_then(|s| depth.get(s));
        (source.copied().unwrap_or(usize::MAX), *id)
    });
    ordered.into_iter().map(|(_, edge)| edge).collect()
}

/// One node's line: `id`, then `label` (already quoted) inside the shape's
/// delimiters.
///
/// mmdflux ends a round `( )` node at the first `)`, even inside the quoted
/// label, and then fails to parse the rest ("Say · Confirm (booking)"). Other
/// shapes are unaffected, and so is mermaid.js. So in the terminal, a round
/// node's brackets become same-width lookalikes.
fn node_line(id: &str, open: &str, label: &str, close: &str, target: Target) -> String {
    let label = if target == Target::Terminal && open == "(" {
        label.replace('(', "❨").replace(')', "❩")
    } else {
        label.to_string()
    };
    format!("    {id}{open}{label}{close}\n")
}

/// A node's label: the kind of node (absent for start/end, which say it
/// themselves) and the text that identifies this one.
struct NodeLabel {
    kind: Option<String>,
    text: String,
}

impl NodeLabel {
    /// The quoted label for `target`. Export puts it on one line, `kind ·
    /// text`; the terminal stacks the kind over the text, wrapped narrow.
    fn render(&self, target: Target) -> String {
        match target {
            Target::Export => {
                let line = match &self.kind {
                    Some(kind) => format!("{kind} · {}", self.text),
                    None => self.text.clone(),
                };
                quote(&truncate(&line, NODE_LABEL_MAX))
            }
            Target::Terminal => {
                let mut lines: Vec<String> = self.kind.iter().cloned().collect();
                lines.extend(wrap(&truncate(&self.text, NODE_LABEL_MAX), TERMINAL_WRAP));
                let lines: Vec<String> = lines.iter().map(|l| plain(l)).collect();
                format!("\"{}\"", lines.join("<br/>"))
            }
        }
    }
}

/// A node's label: its own `label` when it has one, otherwise a summary
/// derived from its type. Tagged with the kind, so types are tellable apart
/// without colour.
fn node_label(id: &str, node_type: &str, node: &Value) -> NodeLabel {
    let own = str_field(node, "label").filter(|l| !l.trim().is_empty());
    let (kind, text) = match node_type {
        "start" => (None, "Start".to_string()),
        "end" => match str_field(node, "outcome") {
            Some(outcome) => (None, format!("End ({outcome})")),
            None => (None, "End".to_string()),
        },
        "say" => {
            let summary = own.or_else(|| {
                let message = node.get("message")?;
                str_field(message, "text").or_else(|| str_field(message, "prompt"))
            });
            (Some("Say"), summary.unwrap_or(id).to_string())
        }
        "tool" => {
            let tools: Vec<&str> = node
                .get("tools")
                .and_then(Value::as_array)
                .map(|ts| ts.iter().filter_map(|t| str_field(t, "tool_id")).collect())
                .unwrap_or_default();
            let text = if tools.is_empty() {
                id.to_string()
            } else {
                tools.join(", ")
            };
            (Some("Tool"), text)
        }
        "update_state" => (Some("Update state"), own.unwrap_or(id).to_string()),
        "override_agent" => (Some("Subagent"), own.unwrap_or(id).to_string()),
        "standalone_agent" => {
            let agent = str_field(node, "agent_id").unwrap_or(id);
            let text = match str_field(node, "branch_id") {
                Some(branch) => format!("{agent} ({branch})"),
                None => agent.to_string(),
            };
            (Some("Agent transfer"), text)
        }
        "phone_number" => {
            let destination = node
                .get("transfer_destination")
                .and_then(|d| str_field(d, "phone_number").or_else(|| str_field(d, "sip_uri")));
            (
                Some("Phone transfer"),
                destination.unwrap_or(id).to_string(),
            )
        }
        other => (Some(other), own.unwrap_or(id).to_string()),
    };
    NodeLabel {
        kind: kind.map(String::from),
        text,
    }
}

/// A present, non-null condition object.
fn condition<'a>(edge: &'a Value, key: &str) -> Option<&'a Value> {
    edge.get(key).filter(|c| c.is_object())
}

/// A short summary of an edge condition, or `None` when there is nothing
/// worth printing (an unlabelled unconditional edge).
fn condition_label(cond: &Value) -> Option<String> {
    if let Some(label) = str_field(cond, "label").filter(|l| !l.trim().is_empty()) {
        return Some(label.to_string());
    }
    match str_field(cond, "type") {
        Some("llm") => str_field(cond, "condition")
            .filter(|c| !c.trim().is_empty())
            .map(String::from),
        Some("result") => Some(
            if cond.get("successful").and_then(Value::as_bool) == Some(true) {
                "success"
            } else {
                "failure"
            }
            .to_string(),
        ),
        Some("expression") => Some("expression".to_string()),
        Some("unconditional") | None => None,
        Some(other) => Some(other.to_string()),
    }
}

fn str_field<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}

/// Collapse whitespace and cut to `max` characters, ending in `…` when cut.
fn truncate(text: &str, max: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= max {
        return collapsed;
    }
    let cut: String = collapsed.chars().take(max - 1).collect();
    format!("{}…", cut.trim_end())
}

/// Quote label text for Mermaid, entity-encoding everything that would end
/// the string or be read as syntax. `#` goes first: it starts every entity.
fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '#' => out.push_str("#35;"),
            '"' => out.push_str("#quot;"),
            '<' => out.push_str("#lt;"),
            '>' => out.push_str("#gt;"),
            '|' => out.push_str("#124;"),
            '`' => out.push_str("#96;"),
            '\n' | '\r' => out.push(' '),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Label text for mmdflux, which prints quoted text literally but does not
/// decode Mermaid's `#…;` entities. Only a quote (ends the string) and angle
/// brackets (read as tags, like the `<br/>` that wraps lines) need replacing,
/// and lookalikes keep the text readable.
fn plain(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '"' => '\'',
            '<' => '‹',
            '>' => '›',
            '\n' | '\r' => ' ',
            c => c,
        })
        .collect()
}

/// Greedy word wrap to `width` characters. A word longer than the width
/// gets a line of its own rather than being split.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

/// A node ID for a `%%` comment: one line, nothing else to escape.
fn comment_safe(id: &str) -> String {
    id.replace(['\n', '\r'], " ")
}

/// Mermaid-safe node IDs for workflow node IDs.
///
/// Workflow IDs are arbitrary strings; Mermaid IDs must be plain words and
/// must not be keywords (`end`) or start with `o`/`x` right after a link
/// (`A---oB` is a circle edge). Prefixing `n_` and replacing everything else
/// with `_` covers all of it; a numeric suffix separates IDs that collapse to
/// the same word. IDs are assigned in sorted order so they are stable.
struct MermaidIds<'a> {
    map: BTreeMap<&'a str, String>,
}

impl<'a> MermaidIds<'a> {
    fn new(ids: impl Iterator<Item = &'a str>) -> Self {
        let sorted: BTreeSet<&str> = ids.collect();
        let mut map = BTreeMap::new();
        let mut taken = BTreeSet::new();
        for id in sorted {
            let base: String = format!("n_{id}")
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            let mut candidate = base.clone();
            let mut n = 2;
            while !taken.insert(candidate.clone()) {
                candidate = format!("{base}_{n}");
                n += 1;
            }
            map.insert(id, candidate);
        }
        MermaidIds { map }
    }

    fn get(&self, id: &str) -> &str {
        // Every id rendered was passed to `new`.
        self.map.get(id).map(String::as_str).unwrap_or("n_")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn render(workflow: Value) -> String {
        render_mermaid(&workflow)
    }

    #[test]
    fn every_known_node_type_gets_its_own_shape_and_class() {
        let out = render(json!({
            "nodes": {
                "a_start": { "type": "start" },
                "b_say": { "type": "say", "label": "Greet", "message": { "type": "literal", "text": "Hi" } },
                "c_tool": { "type": "tool", "tools": [{ "tool_id": "tool_1" }, { "tool_id": "tool_2" }] },
                "d_state": { "type": "update_state", "label": "Save name", "updates": [] },
                "e_sub": { "type": "override_agent", "label": "Billing" },
                "f_agent": { "type": "standalone_agent", "agent_id": "agent_9", "branch_id": "agtbrch_1" },
                "g_phone": { "type": "phone_number", "transfer_destination": { "type": "phone", "phone_number": "+15550100" } },
                "h_end": { "type": "end", "outcome": "success" }
            },
            "edges": {}
        }));
        assert!(out.contains(r#"n_a_start(["Start"])"#), "{out}");
        assert!(out.contains(r#"n_b_say("Say · Greet")"#), "{out}");
        assert!(
            out.contains(r#"n_c_tool[["Tool · tool_1, tool_2"]]"#),
            "{out}"
        );
        assert!(
            out.contains(r#"n_d_state[/"Update state · Save name"/]"#),
            "{out}"
        );
        assert!(out.contains(r#"n_e_sub["Subagent · Billing"]"#), "{out}");
        assert!(
            out.contains(r#"n_f_agent{{"Agent transfer · agent_9 (agtbrch_1)"}}"#),
            "{out}"
        );
        assert!(
            out.contains(r#"n_g_phone{{"Phone transfer · +15550100"}}"#),
            "{out}"
        );
        assert!(out.contains(r#"n_h_end(["End (success)"])"#), "{out}");
        for class in [
            "t_start",
            "t_end",
            "t_say",
            "t_tool",
            "t_state",
            "t_subagent",
            "t_transfer",
        ] {
            assert!(
                out.contains(&format!("classDef {class} ")),
                "{class}: {out}"
            );
        }
        assert!(
            out.contains("class n_f_agent,n_g_phone t_transfer"),
            "{out}"
        );
        // Only classes in use are defined.
        assert!(!out.contains("t_unknown"), "{out}");
    }

    #[test]
    fn an_unknown_node_type_renders_generically() {
        let out = render(json!({
            "nodes": { "x": { "type": "future_thing", "label": "Later" } },
            "edges": {}
        }));
        assert!(out.contains(r#"n_x["future_thing · Later"]"#), "{out}");
        assert!(out.contains("class n_x t_unknown"), "{out}");
    }

    #[test]
    fn a_say_node_without_a_label_falls_back_to_its_message() {
        let out = render(json!({
            "nodes": { "s": { "type": "say", "label": "", "message": { "type": "prompt", "prompt": "Ask for the order number" } } },
            "edges": {}
        }));
        assert!(out.contains(r#""Say · Ask for the order number""#), "{out}");
    }

    #[test]
    fn edge_labels_summarize_each_condition_kind() {
        let out = render(json!({
            "nodes": {
                "a": { "type": "start" }, "b": { "type": "end" }, "c": { "type": "end" },
                "d": { "type": "end" }, "e": { "type": "end" }, "f": { "type": "end" }
            },
            "edges": {
                "e1": { "source": "a", "target": "b", "forward_condition": { "type": "llm", "condition": "User wants a refund" } },
                "e2": { "source": "a", "target": "c", "forward_condition": { "type": "result", "successful": false } },
                "e3": { "source": "a", "target": "d", "forward_condition": { "type": "expression", "expression": { "type": "and_operator", "children": [] } } },
                "e4": { "source": "a", "target": "e", "forward_condition": { "type": "unconditional" } },
                "e5": { "source": "a", "target": "f", "forward_condition": { "type": "llm", "label": "Refund", "condition": "long text" } }
            }
        }));
        assert!(
            out.contains(r#"n_a -->|"User wants a refund"| n_b"#),
            "{out}"
        );
        assert!(out.contains(r#"n_a -->|"failure"| n_c"#), "{out}");
        assert!(out.contains(r#"n_a -->|"expression"| n_d"#), "{out}");
        assert!(out.contains("n_a --> n_e\n"), "{out}");
        // The human label wins over the raw condition.
        assert!(out.contains(r#"n_a -->|"Refund"| n_f"#), "{out}");
    }

    #[test]
    fn a_backward_condition_draws_a_reverse_dotted_arrow() {
        let out = render(json!({
            "nodes": { "a": { "type": "say", "label": "A" }, "b": { "type": "say", "label": "B" } },
            "edges": {
                "e": {
                    "source": "a", "target": "b",
                    "forward_condition": { "type": "llm", "condition": "go on" },
                    "backward_condition": { "type": "llm", "condition": "go back" }
                }
            }
        }));
        assert!(out.contains(r#"n_a -->|"go on"| n_b"#), "{out}");
        assert!(out.contains(r#"n_b -.->|"go back"| n_a"#), "{out}");
    }

    #[test]
    fn a_null_backward_condition_draws_nothing_backward() {
        let out = render(json!({
            "nodes": { "a": { "type": "start" }, "b": { "type": "end" } },
            "edges": { "e": { "source": "a", "target": "b", "forward_condition": { "type": "unconditional" }, "backward_condition": null } }
        }));
        assert!(!out.contains("-.->"), "{out}");
    }

    #[test]
    fn label_text_cannot_break_out_of_the_mermaid_string() {
        let out = render(json!({
            "nodes": {
                "a": { "type": "say", "label": "Say \"hi\" | <b>#1</b>\nthen `wait`" },
                "b": { "type": "end" }
            },
            "edges": { "e": { "source": "a", "target": "b", "forward_condition": { "type": "llm", "condition": "x|y \"z\"" } } }
        }));
        assert!(
            out.contains(r#"n_a("Say · Say #quot;hi#quot; #124; #lt;b#gt;#35;1#lt;/b#gt; then #96;wait#96;")"#),
            "{out}"
        );
        assert!(
            out.contains(r#"n_a -->|"x#124;y #quot;z#quot;"| n_b"#),
            "{out}"
        );
    }

    #[test]
    fn long_conditions_are_truncated_with_an_ellipsis() {
        let long = "The user has clearly said that they would like to speak to a human agent now";
        let out = render(json!({
            "nodes": { "a": { "type": "start" }, "b": { "type": "end" } },
            "edges": { "e": { "source": "a", "target": "b", "forward_condition": { "type": "llm", "condition": long } } }
        }));
        let label = out
            .lines()
            .find(|l| l.contains("-->"))
            .and_then(|l| l.split('"').nth(1))
            .unwrap();
        assert!(label.ends_with('…'), "{label}");
        assert_eq!(label.chars().count(), EDGE_LABEL_MAX, "{label}");
    }

    #[test]
    fn node_ids_are_made_mermaid_safe_and_kept_distinct() {
        let out = render(json!({
            "nodes": {
                "end": { "type": "end" },
                "node-1": { "type": "say", "label": "A" },
                "node_1": { "type": "say", "label": "B" },
                "o": { "type": "start" }
            },
            "edges": { "e": { "source": "o", "target": "end" } }
        }));
        assert!(out.contains(r#"n_end(["End"])"#), "{out}");
        assert!(out.contains(r#"n_node_1("Say · A")"#), "{out}");
        assert!(out.contains(r#"n_node_1_2("Say · B")"#), "{out}");
        assert!(out.contains("n_o --> n_end\n"), "{out}");
    }

    #[test]
    fn an_edge_to_a_missing_node_is_drawn_not_dropped() {
        let out = render(json!({
            "nodes": { "a": { "type": "start" } },
            "edges": { "e": { "source": "a", "target": "ghost" } }
        }));
        assert!(out.contains("%% no node defines 'ghost'"), "{out}");
        assert!(out.contains(r#"n_ghost["missing: ghost"]"#), "{out}");
        assert!(out.contains("n_a --> n_ghost"), "{out}");
    }

    #[test]
    fn output_does_not_depend_on_map_order() {
        let forward: Value = serde_json::from_str(
            r#"{"nodes":{"a":{"type":"start"},"b":{"type":"say","label":"B"},"c":{"type":"end"}},
                "edges":{"e1":{"source":"a","target":"b"},"e2":{"source":"b","target":"c"}}}"#,
        )
        .unwrap();
        let reversed: Value = serde_json::from_str(
            r#"{"edges":{"e2":{"source":"b","target":"c"},"e1":{"source":"a","target":"b"}},
                "nodes":{"c":{"type":"end"},"b":{"type":"say","label":"B"},"a":{"type":"start"}}}"#,
        )
        .unwrap();
        assert_eq!(render_mermaid(&forward), render_mermaid(&reversed));
    }

    #[test]
    fn a_workflow_without_nodes_has_nothing_to_draw() {
        assert!(workflow_of(&json!({})).is_none());
        assert!(workflow_of(&json!({ "workflow": null })).is_none());
        assert!(workflow_of(&json!({ "workflow": { "nodes": {}, "edges": {} } })).is_none());
        assert!(
            workflow_of(&json!({ "workflow": { "nodes": { "a": { "type": "start" } } } }))
                .is_some()
        );
    }

    #[test]
    fn markdown_output_is_a_mermaid_fence() {
        assert_eq!(fence("flowchart TD\n"), "```mermaid\nflowchart TD\n```\n");
    }

    fn terminal(workflow: Value) -> Built {
        build(&workflow, Target::Terminal)
    }

    #[test]
    fn terminal_conditions_become_numbered_keys_with_a_legend() {
        let built = terminal(json!({
            "nodes": { "a": { "type": "start" }, "b": { "type": "end" }, "c": { "type": "end" } },
            "edges": {
                "e1": { "source": "a", "target": "b", "forward_condition": { "type": "llm", "condition": "The caller   wants a refund for their whole order, not just part of it" } },
                "e2": { "source": "a", "target": "c", "forward_condition": { "type": "unconditional" } }
            }
        }));
        assert!(
            built.source.contains(r#"n_a --- k_1(["1"]) --> n_b"#),
            "{}",
            built.source
        );
        assert!(built.source.contains("n_a --> n_c\n"), "{}", built.source);
        assert!(built.source.contains("class k_1 t_key"), "{}", built.source);
        // The legend has the whole condition, not the export's truncation.
        assert_eq!(
            built.legend,
            ["  1  The caller wants a refund for their whole order, not just part of it"]
        );
    }

    #[test]
    fn terminal_backward_keys_are_dotted_and_say_so() {
        let built = terminal(json!({
            "nodes": { "a": { "type": "say", "label": "A" }, "b": { "type": "say", "label": "B" } },
            "edges": { "e": {
                "source": "a", "target": "b",
                "forward_condition": { "type": "llm", "condition": "on" },
                "backward_condition": { "type": "llm", "condition": "back" }
            } }
        }));
        assert!(
            built.source.contains(r#"n_b -.- k_2(["2"]) -.-> n_a"#),
            "{}",
            built.source
        );
        assert_eq!(built.legend[1], "  2  back (going back)");
    }

    #[test]
    fn terminal_labels_are_plain_text_wrapped_under_their_kind() {
        let built = terminal(json!({
            "nodes": {
                "s": { "type": "say", "label": "Read back the \"booking\" <details> to the caller" },
                "u": { "type": "update_state", "label": "Save", "updates": [] }
            },
            "edges": {}
        }));
        // mmdflux does not decode entities, so none may appear.
        assert!(!built.source.contains("#quot;"), "{}", built.source);
        assert!(
            built
                .source
                .contains(r#"n_s("Say<br/>Read back the 'booking'<br/>‹details› to the caller")"#),
            "{}",
            built.source
        );
        // No parallelogram in mmdflux; a cylinder instead.
        assert!(
            built.source.contains(r#"n_u[("Update state<br/>Save")]"#),
            "{}",
            built.source
        );
    }

    #[test]
    fn terminal_keys_are_numbered_in_flow_order() {
        // By edge ID the start edge ("z_first") would be numbered last.
        let built = terminal(json!({
            "nodes": { "s": { "type": "start" }, "t": { "type": "tool" }, "a": { "type": "end" }, "b": { "type": "end" } },
            "edges": {
                "a_fail": { "source": "t", "target": "a", "forward_condition": { "type": "result", "successful": false } },
                "b_ok": { "source": "t", "target": "b", "forward_condition": { "type": "result", "successful": true } },
                "z_first": { "source": "s", "target": "t", "forward_condition": { "type": "llm", "condition": "go" } }
            }
        }));
        assert_eq!(built.legend, ["  1  go", "  2  failure", "  3  success"]);
    }

    #[test]
    fn brackets_in_a_say_node_still_draw_in_the_terminal() {
        let workflow = json!({
            "nodes": {
                "a": { "type": "start" },
                "b": { "type": "say", "label": "Confirm (booking) details" },
                "c": { "type": "say", "label": "a ) b" }
            },
            "edges": { "e": { "source": "a", "target": "b" } }
        });
        let built = build(&workflow, Target::Terminal);
        assert!(
            built.source.contains("Confirm ❨booking❩"),
            "{}",
            built.source
        );
        render_terminal(&workflow, false).expect("draws");
        // mermaid.js handles brackets, so export keeps them.
        assert!(render_mermaid(&workflow).contains("Confirm (booking)"));
    }

    #[test]
    fn wrap_keeps_long_words_whole() {
        assert_eq!(wrap("a bb ccc", 4), ["a bb", "ccc"]);
        assert_eq!(
            wrap("system__is_text_only x", 8),
            ["system__is_text_only", "x"]
        );
        assert!(wrap("   ", 8).is_empty());
    }

    /// Every fixture must draw: an mmdflux parse error would only show up as
    /// a failed command.
    #[test]
    fn every_fixture_draws_in_the_terminal() {
        for input in [
            include_str!("fixtures/diagram/linear.json"),
            include_str!("fixtures/diagram/tool_branches.json"),
            include_str!("fixtures/diagram/transfers.json"),
            include_str!("fixtures/diagram/llm_loop.json"),
        ] {
            let config: Value = serde_json::from_str(input).unwrap();
            let workflow = workflow_of(&config).unwrap();
            let drawn = render_terminal(workflow, false).expect("draws");
            assert!(drawn.contains("Start"), "{drawn}");
            assert!(
                !drawn.contains('\u{1b}'),
                "no colour was asked for: {drawn}"
            );
            let legend = build(workflow, Target::Terminal).legend;
            for line in &legend {
                assert!(drawn.contains(line.as_str()), "legend line missing: {line}");
            }
        }
    }

    /// Representative workflows, rendered in full. A change here is a change
    /// to every diagram users have committed, so it should be deliberate.
    #[test]
    fn fixtures_render_to_their_snapshots() {
        const FIXTURES: &[(&str, &str, &str)] = &[
            (
                "linear",
                include_str!("fixtures/diagram/linear.json"),
                include_str!("fixtures/diagram/linear.mmd"),
            ),
            (
                "tool_branches",
                include_str!("fixtures/diagram/tool_branches.json"),
                include_str!("fixtures/diagram/tool_branches.mmd"),
            ),
            (
                "transfers",
                include_str!("fixtures/diagram/transfers.json"),
                include_str!("fixtures/diagram/transfers.mmd"),
            ),
            (
                "llm_loop",
                include_str!("fixtures/diagram/llm_loop.json"),
                include_str!("fixtures/diagram/llm_loop.mmd"),
            ),
        ];
        for (name, input, expected) in FIXTURES {
            let config: Value = serde_json::from_str(input).expect(name);
            let workflow = workflow_of(&config).expect(name);
            assert_eq!(render_mermaid(workflow), *expected, "fixture {name}");
        }
    }
}
