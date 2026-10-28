//! `agents graph`: how the agents in a project connect to their tools, tests,
//! knowledge-base documents and to each other.
//!
//! Agent configs refer to everything else by ID only, and nothing else in the
//! CLI follows those references. This reads every config in `agents.json`
//! (branches included), resolves the IDs against `tools.json` / `tests.json` /
//! `agents.json`, and draws the result in the terminal, prints it as Mermaid,
//! or answers the reverse question with `--uses`: which agents depend on this?
//!
//! Local files only, like `agents workflow diagram`. Knowledge-base documents
//! have no local registry, so they are shown by the ID and name the agent
//! config carries. An ID that isn't in the project (a tool made in the
//! dashboard, a transfer to an agent outside the repo) becomes an external
//! node rather than being dropped.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::agents::{config_entries, find_agent, require_agents, AgentConfigEntry};
use super::mermaid::{
    abbreviate, draw_text, fence, node_line, plain, push_classes, quote, str_field, tangles,
    terminal_width, text_width, truncate, use_color, wrap, Format, MermaidIds, NodeLabel, Target,
    EDGE_LABEL_MAX, NODE_LABEL_MAX, SHORT_ID_MIN, TERMINAL_WRAP,
};
use super::{api, project};

/// Register `agents graph`.
pub fn register(app: CliApp) -> CliApp {
    app.command_under_typed_with(
        &["agents"],
        clap::Command::new("graph").about(
            "Show how agents connect to their tools, tests, documents and each other (local files only)",
        ),
        handle_graph,
    )
}

#[derive(clap::Args)]
struct GraphArgs {
    /// Show only this agent and what it connects to: a config file, or an
    /// agent ID or name from agents.json.
    agent: Option<String>,
    /// List the agents that use this tool, test, agent or document (an ID, or
    /// a tool's or test's name), one per line, tab-separated: agent ID, agent
    /// name, where the reference is.
    #[arg(long, value_name = "ID_OR_NAME", conflicts_with_all = ["agent", "format"])]
    uses: Option<String>,
    /// What to print: one drawing per agent for the terminal, or the whole
    /// project as Mermaid source, bare or in a ```mermaid fence.
    #[arg(long, value_enum, default_value_t = Format::Ascii)]
    format: Format,
}

fn handle_graph(args: GraphArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let _scope = api::command_scope("agents.graph");
    let registry = require_agents()?;
    let entries = config_entries(&registry);
    let mut agents = Vec::new();
    for entry in entries {
        match project::read_value_in_project(&entry.config_path) {
            Ok(config) => agents.push(LoadedAgent { entry, config }),
            Err(e) => eprintln!("Skipping '{}': {e}", entry.title),
        }
    }
    if agents.is_empty() {
        eprintln!("No agents in agents.json to graph.");
        return Ok(());
    }
    let graph = collect(&agents, &tool_names()?, &test_names()?);

    if let Some(query) = &args.uses {
        let lines = uses(&graph, query);
        if lines.is_empty() {
            eprintln!("No agent in agents.json uses '{query}'.");
        }
        for line in lines {
            println!("{line}");
        }
        return Ok(());
    }

    let scope = match &args.agent {
        Some(target) => Some(resolve_agent(&registry, &agents, target)?),
        None => None,
    };
    if args.format != Format::Ascii {
        let graph = match &scope {
            Some(key) => graph.scoped_to(key),
            None => graph,
        };
        let source = render(&graph, Target::Export);
        if args.format == Format::MermaidMd {
            print!("{}", fence(&source));
        } else {
            print!("{source}");
        }
        return Ok(());
    }

    // In the terminal the whole project is one drawing per agent, in
    // agents.json order: drawn as one graph, edges cross and merge until it is
    // no longer clear what connects to what. Each agent on its own is a star,
    // which draws cleanly. --format mermaid keeps the single graph, which mermaid.js
    // lays out well.
    let color = use_color();
    let width = terminal_width();
    let draw = |key: &Key| draw_fitted(&graph.star(key), key, width, color);
    match &scope {
        Some(key) => print!("{}", draw(key)?),
        None => {
            for (i, agent) in agents.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                println!("── {} ({})\n", agent.entry.title, agent.entry.config_path);
                print!("{}", draw(&agent_key(&agent.entry))?);
            }
        }
    }
    Ok(())
}

/// Draw one agent's [`Graph::star`] for the terminal (see [`render_star`]).
/// When the drawing is wider than `width`,
/// redraw with long IDs shortened; if even that does not fit, say so on
/// stderr and print it anyway.
fn draw_fitted(
    star: &Graph,
    focus: &Key,
    width: Option<usize>,
    color: bool,
) -> Result<String, CliError> {
    const FALLBACK: &str = "elevenlabs agents graph --format mermaid";
    // Measure without colour: ANSI codes are not columns.
    let (mut order, mut plain) = best_order(star, focus, FALLBACK)?;
    let mut graph = star.clone();
    if let Some(width) = width.filter(|w| text_width(&plain) > *w) {
        graph = shorten_ids(star);
        (order, plain) = best_order(&graph, focus, FALLBACK)?;
        if text_width(&plain) > width {
            eprintln!(
                "This drawing is {} columns wide, more than the {width} available. \
                 '{FALLBACK}' prints it as Mermaid instead.",
                text_width(&plain)
            );
        }
    }
    if color {
        draw_text(&render_star(&graph, focus, &order), true, FALLBACK)
    } else {
        Ok(plain)
    }
}

/// A copy of `graph` with every node named by a long ID shown as the ID's
/// start and end, `tool_01jx…sfb67`. Real names are left alone. Shortened
/// names are kept distinct by lengthening the ones that collide.
fn shorten_ids(graph: &Graph) -> Graph {
    let mut short = graph.clone();
    let long: Vec<Key> = graph
        .nodes
        .iter()
        .filter(|(_, n)| n.name == n.id && n.id.chars().count() > SHORT_ID_MIN)
        .map(|(k, _)| k.clone())
        .collect();
    for key in &long {
        let id = &graph.nodes[key].id;
        let chars: Vec<char> = id.chars().collect();
        // Start with 8 + 5 characters; grow the tail until no other long ID
        // in the same graph abbreviates the same way.
        let mut tail = 5;
        let name = loop {
            let candidate = abbreviate(&chars, 8, tail);
            let clash = long.iter().any(|other| {
                other != key && {
                    let o: Vec<char> = graph.nodes[other].id.chars().collect();
                    abbreviate(&o, 8, tail) == candidate
                }
            });
            if !clash || 8 + tail >= chars.len() {
                break if clash { id.clone() } else { candidate };
            }
            tail += 1;
        };
        short.nodes.get_mut(key).expect("key from graph").name = name;
    }
    short
}

/// An agents.json entry and its parsed config.
struct LoadedAgent {
    entry: AgentConfigEntry,
    config: Value,
}

/// The graph key of the agent `target` names: a config file path, or an
/// agent ID or name (which means its main config).
fn resolve_agent(
    registry: &project::AgentsConfig,
    agents: &[LoadedAgent],
    target: &str,
) -> Result<Key, CliError> {
    let not_found = || {
        CliError::Validation(format!(
            "'{target}' is not a config file in agents.json, or an agent ID or name in it"
        ))
    };
    let config_path = if Path::new(target).is_file() {
        let wanted = Path::new(target).canonicalize().ok();
        agents
            .iter()
            .find(|a| Path::new(&a.entry.config_path).canonicalize().ok() == wanted)
            .map(|a| a.entry.config_path.clone())
            .ok_or_else(not_found)?
    } else {
        find_agent(registry, target)
            .ok_or_else(not_found)?
            .config
            .clone()
    };
    agents
        .iter()
        .find(|a| a.entry.config_path == config_path)
        .map(|a| agent_key(&a.entry))
        .ok_or_else(not_found)
}

/// Tool ID → local name, from tools.json. A project without one has no
/// local tools, so every tool reference is external.
fn tool_names() -> Result<BTreeMap<String, String>, CliError> {
    if !Path::new(project::TOOLS_FILE).exists() {
        return Ok(BTreeMap::new());
    }
    Ok(project::load_tools()?
        .tools
        .iter()
        .filter_map(|t| Some((t.id.clone()?, registry_name(&t.config))))
        .collect())
}

/// Test ID → local name, from tests.json. Missing means none, as for tools.
fn test_names() -> Result<BTreeMap<String, String>, CliError> {
    if !Path::new(project::TESTS_FILE).exists() {
        return Ok(BTreeMap::new());
    }
    Ok(project::load_tests()?
        .tests
        .iter()
        .filter_map(|t| Some((t.id.clone()?, registry_name(&t.config))))
        .collect())
}

/// A registered config's name, falling back to its path.
fn registry_name(config_path: &str) -> String {
    project::config_name(config_path).unwrap_or_else(|| config_path.to_string())
}

// ── Model ───────────────────────────────────────────────────────────

/// What a node is. The order is the order nodes are emitted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Agent,
    Tool,
    Test,
    Document,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Agent => "Agent",
            Kind::Tool => "Tool",
            Kind::Test => "Test",
            Kind::Document => "Document",
        }
    }
}

/// A node's identity: its kind and an ID unique within that kind. Agents are
/// keyed by agent ID plus `#branch_id` for a registered branch; an agent not
/// pushed yet (no ID) by its config path.
type Key = (Kind, String);

#[derive(Clone)]
struct Node {
    /// The name shown: the local name, or the ID when there is none.
    name: String,
    /// The resource ID `--uses` prints (an agent's ID without the branch).
    id: String,
    /// Referenced, but not in this project.
    external: bool,
}

#[derive(Clone, Default)]
struct Graph {
    nodes: BTreeMap<Key, Node>,
    /// (from, to) → where in the source config the reference is. Several
    /// references between the same two nodes are one edge.
    edges: BTreeMap<(Key, Key), BTreeSet<String>>,
}

impl Graph {
    /// One agent, everything it references, and every agent that hands off
    /// to it.
    fn scoped_to(&self, agent: &Key) -> Graph {
        let edges: BTreeMap<(Key, Key), BTreeSet<String>> = self
            .edges
            .iter()
            .filter(|((from, to), _)| from == agent || to == agent)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let mut keep: BTreeSet<&Key> = BTreeSet::from([agent]);
        for (from, to) in edges.keys() {
            keep.insert(from);
            keep.insert(to);
        }
        let nodes = self
            .nodes
            .iter()
            .filter(|(k, _)| keep.contains(k))
            .map(|(k, n)| (k.clone(), n.clone()))
            .collect();
        Graph { nodes, edges }
    }
}

impl Graph {
    /// [`Graph::scoped_to`], laid out for drawing as text: the agents that
    /// hand off to `agent` become separate "caller" nodes, so every edge runs
    /// left to right (callers → agent → what it uses).
    ///
    /// With the real nodes, an agent that `agent` also hands off to (A ⇄ B)
    /// makes a loop, and mmdflux routes the way back through the column of
    /// tools and tests, as if they connected to each other. Mermaid export
    /// keeps the real graph: mermaid.js lays loops out well.
    fn star(&self, agent: &Key) -> Graph {
        let mut star = Graph::default();
        if let Some(node) = self.nodes.get(agent) {
            star.nodes.insert(agent.clone(), node.clone());
        }
        for ((from, to), vias) in &self.edges {
            if from == agent {
                star.nodes.insert(to.clone(), self.nodes[to].clone());
                star.edges.insert((from.clone(), to.clone()), vias.clone());
            } else if to == agent {
                let caller = (Kind::Agent, format!("caller:{}", from.1));
                star.nodes.insert(caller.clone(), self.nodes[from].clone());
                star.edges.insert((caller, to.clone()), vias.clone());
            }
        }
        star
    }
}

fn agent_key(entry: &AgentConfigEntry) -> Key {
    let id = match (&entry.agent_id, &entry.branch_name, &entry.branch_id) {
        (Some(id), Some(_), Some(branch)) => format!("{id}#{branch}"),
        (Some(id), _, _) => id.clone(),
        (None, _, _) => entry.config_path.clone(),
    };
    (Kind::Agent, id)
}

/// One reference found in an agent config.
struct Ref {
    kind: Kind,
    id: String,
    /// Transfers only: the target branch, if the transfer names one.
    branch_id: Option<String>,
    /// Documents only: the name the locator carries.
    name: Option<String>,
    /// Where in the config, for `--uses` and export edge labels.
    via: String,
}

/// Every reference in one agent config, in a stable order.
fn refs_in(config: &Value) -> Vec<Ref> {
    let mut refs = Vec::new();
    if let Some(prompt) = config.pointer("/conversation_config/agent/prompt") {
        refs_in_prompt(prompt, "prompt", &mut refs);
    }
    for test in array(config.pointer("/platform_settings/testing/attached_tests")) {
        if let Some(id) = str_field(test, "test_id") {
            refs.push(simple(
                Kind::Test,
                id,
                "platform_settings.testing.attached_tests",
            ));
        }
    }
    let nodes = config
        .pointer("/workflow/nodes")
        .and_then(Value::as_object)
        .map(|n| n.iter().collect::<BTreeMap<_, _>>())
        .unwrap_or_default();
    for (node_id, node) in nodes {
        let via = format!("workflow node '{node_id}'");
        for tool in array(node.get("tools")) {
            if let Some(id) = str_field(tool, "tool_id") {
                refs.push(simple(Kind::Tool, id, &via));
            }
        }
        for id in array(node.get("additional_tool_ids")).filter_map(Value::as_str) {
            refs.push(simple(Kind::Tool, id, &via));
        }
        for doc in array(node.get("additional_knowledge_base")) {
            push_document(doc, &via, &mut refs);
        }
        if str_field(node, "type") == Some("standalone_agent") {
            if let Some(id) = str_field(node, "agent_id") {
                refs.push(Ref {
                    kind: Kind::Agent,
                    id: id.to_string(),
                    branch_id: str_field(node, "branch_id").map(String::from),
                    name: None,
                    via: via.clone(),
                });
            }
        }
        if let Some(prompt) = node.pointer("/conversation_config/agent/prompt") {
            refs_in_prompt(prompt, &via, &mut refs);
        }
    }
    refs
}

/// The references a `prompt` object can hold, at the root or in a workflow
/// node's override.
fn refs_in_prompt(prompt: &Value, via: &str, refs: &mut Vec<Ref>) {
    let at = |field: &str| {
        if via == "prompt" {
            format!("prompt.{field}")
        } else {
            via.to_string()
        }
    };
    for id in array(prompt.get("tool_ids")).filter_map(Value::as_str) {
        refs.push(simple(Kind::Tool, id, &at("tool_ids")));
    }
    for doc in array(prompt.get("knowledge_base")) {
        push_document(doc, &at("knowledge_base"), refs);
    }
    let transfers = prompt.pointer("/built_in_tools/transfer_to_agent/params/transfers");
    for transfer in array(transfers) {
        if let Some(id) = str_field(transfer, "agent_id") {
            refs.push(Ref {
                kind: Kind::Agent,
                id: id.to_string(),
                branch_id: str_field(transfer, "branch_id").map(String::from),
                name: None,
                via: at("built_in_tools.transfer_to_agent"),
            });
        }
    }
}

fn push_document(doc: &Value, via: &str, refs: &mut Vec<Ref>) {
    if let Some(id) = str_field(doc, "id") {
        refs.push(Ref {
            kind: Kind::Document,
            id: id.to_string(),
            branch_id: None,
            name: str_field(doc, "name")
                .filter(|n| !n.trim().is_empty())
                .map(String::from),
            via: via.to_string(),
        });
    }
}

fn simple(kind: Kind, id: &str, via: &str) -> Ref {
    Ref {
        kind,
        id: id.to_string(),
        branch_id: None,
        name: None,
        via: via.to_string(),
    }
}

fn array(value: Option<&Value>) -> impl Iterator<Item = &Value> {
    value.and_then(Value::as_array).into_iter().flatten()
}

/// Build the graph from the loaded agent configs and the tool and test
/// registries (ID → local name). Pure, so it is testable without files.
fn collect(
    agents: &[LoadedAgent],
    tools: &BTreeMap<String, String>,
    tests: &BTreeMap<String, String>,
) -> Graph {
    let mut graph = Graph::default();
    // (agent ID, branch ID) → key. `None` is the main config, which a
    // transfer naming the main branch's ID also reaches.
    let mut agent_keys: BTreeMap<(String, Option<String>), Key> = BTreeMap::new();
    for agent in agents {
        let key = agent_key(&agent.entry);
        graph.nodes.insert(
            key.clone(),
            Node {
                name: agent.entry.title.clone(),
                id: agent.entry.agent_id.clone().unwrap_or_default(),
                external: false,
            },
        );
        if let Some(id) = &agent.entry.agent_id {
            if agent.entry.branch_name.is_none() {
                agent_keys.insert((id.clone(), None), key.clone());
            }
            if let Some(branch) = &agent.entry.branch_id {
                agent_keys
                    .entry((id.clone(), Some(branch.clone())))
                    .or_insert(key);
            }
        }
    }

    for agent in agents {
        let from = agent_key(&agent.entry);
        for r in refs_in(&agent.config) {
            let to = match r.kind {
                Kind::Agent => {
                    let local = r
                        .branch_id
                        .as_ref()
                        .and_then(|b| agent_keys.get(&(r.id.clone(), Some(b.clone()))))
                        .or_else(|| agent_keys.get(&(r.id.clone(), None)));
                    match local {
                        Some(key) => key.clone(),
                        None => external(&mut graph, Kind::Agent, &r.id),
                    }
                }
                Kind::Tool | Kind::Test => {
                    let names = if r.kind == Kind::Tool { tools } else { tests };
                    match names.get(&r.id) {
                        Some(name) => {
                            let key = (r.kind, r.id.clone());
                            graph.nodes.entry(key.clone()).or_insert_with(|| Node {
                                name: name.clone(),
                                id: r.id.clone(),
                                external: false,
                            });
                            key
                        }
                        None => external(&mut graph, r.kind, &r.id),
                    }
                }
                // No local registry: never "external", always named by the
                // locator (the first name seen wins).
                Kind::Document => {
                    let key = (Kind::Document, r.id.clone());
                    graph.nodes.entry(key.clone()).or_insert_with(|| Node {
                        name: r.name.clone().unwrap_or_else(|| r.id.clone()),
                        id: r.id.clone(),
                        external: false,
                    });
                    key
                }
            };
            graph
                .edges
                .entry((from.clone(), to))
                .or_default()
                .insert(r.via);
        }
    }
    graph
}

/// The key of an external node, adding it on first sight.
fn external(graph: &mut Graph, kind: Kind, id: &str) -> Key {
    let key = (kind, id.to_string());
    graph.nodes.entry(key.clone()).or_insert_with(|| Node {
        name: id.to_string(),
        id: id.to_string(),
        external: true,
    });
    key
}

/// `--uses`: one line per (agent, reference), tab-separated agent ID, agent
/// name and where the reference is. `query` matches a node's ID, or the
/// local name of a tool or test.
fn uses(graph: &Graph, query: &str) -> Vec<String> {
    let targets: BTreeSet<&Key> = graph
        .nodes
        .iter()
        .filter(|((kind, _), node)| {
            node.id == query || (matches!(kind, Kind::Tool | Kind::Test) && node.name == query)
        })
        .map(|(key, _)| key)
        .collect();
    let mut lines = Vec::new();
    for ((from, to), vias) in &graph.edges {
        if !targets.contains(to) {
            continue;
        }
        let agent = &graph.nodes[from];
        for via in vias {
            lines.push(format!("{}\t{}\t{via}", agent.id, agent.name));
        }
    }
    lines
}

// ── Rendering ───────────────────────────────────────────────────────

/// Mermaid source for the graph, left to right: a single agent's references
/// then stack downwards instead of spreading sideways. Agents hand off along
/// thick arrows; every other reference is a plain one. For export, a reference made by a
/// workflow node is labelled with the node.
fn render(graph: &Graph, target: Target) -> String {
    let mermaid_key = |(kind, id): &Key| format!("{}:{id}", kind.name());
    let keys: Vec<String> = graph.nodes.keys().map(mermaid_key).collect();
    let ids = MermaidIds::new(keys.iter().map(String::as_str));
    let mut out = String::from("flowchart LR\n");
    let mut classes: BTreeMap<&'static str, Vec<&str>> = BTreeMap::new();

    for (key, node) in &graph.nodes {
        let id = ids.get(&mermaid_key(key));
        let kind = if node.external {
            format!("{} (external)", key.0.name())
        } else {
            key.0.name().to_string()
        };
        let label = NodeLabel {
            kind: Some(kind),
            text: node.name.clone(),
        }
        .render(target);
        let (open, close, class) = match key.0 {
            Kind::Agent => ("[", "]", "t_subagent"),
            Kind::Tool => ("[[", "]]", "t_tool"),
            // A stadium rather than a round node: they draw alike in the
            // terminal, and a round node trips mmdflux on the `)` of
            // "(external)" (see `node_line`).
            Kind::Test => ("([", "])", "t_state"),
            Kind::Document => ("[(", ")]", "t_doc"),
        };
        let class = if node.external { "t_unknown" } else { class };
        out.push_str(&node_line(id, open, &label, close, target));
        classes.entry(class).or_default().push(id);
    }

    for ((from, to), vias) in &graph.edges {
        let link = if to.0 == Kind::Agent { "==>" } else { "-->" };
        let (from, to) = (ids.get(&mermaid_key(from)), ids.get(&mermaid_key(to)));
        let nodes: Vec<&str> = vias
            .iter()
            .filter_map(|v| v.strip_prefix("workflow node "))
            .collect();
        match target {
            Target::Export if !nodes.is_empty() => {
                let label = truncate(&format!("via {}", nodes.join(", ")), EDGE_LABEL_MAX);
                out.push_str(&format!("    {from} {link}|{}| {to}\n", quote(&label)));
            }
            _ => out.push_str(&format!("    {from} {link} {to}\n")),
        }
    }
    push_classes(&mut out, &classes);
    out
}

/// Longest item in a grouped box, in characters.
const GROUP_ITEM_MAX: usize = 36;

/// One box around the agent in [`render_star`].
struct Group<'a> {
    title: &'static str,
    /// How the agent's box counts this group: (one, several).
    count: (&'static str, &'static str),
    pick: Box<dyn Fn(&Key) -> bool + 'a>,
    shape: (&'static str, &'static str),
    class: &'static str,
    id: &'static str,
    /// Handoffs are thick, as in the Mermaid graph.
    link: &'static str,
    /// On the left, pointing at the agent, rather than on the right.
    left: bool,
}
/// The boxes around `focus` in [`render_star`]: callers first (always on the
/// left), then the right-hand boxes in their natural order.
fn star_groups(focus: &Key) -> [Group<'_>; 5] {
    [
        Group {
            title: "Handed off from",
            count: ("hands off to it", "hand off to it"),
            pick: Box::new(is_caller),
            shape: ("[", "]"),
            class: "t_subagent",
            id: "g_callers",
            link: "==>",
            left: true,
        },
        Group {
            title: "Hands off to",
            count: ("handoff", "handoffs"),
            pick: Box::new(move |k: &Key| k.0 == Kind::Agent && k != focus && !is_caller(k)),
            shape: ("[", "]"),
            class: "t_subagent",
            id: "g_handoffs",
            link: "==>",
            left: false,
        },
        Group {
            title: "Tools",
            count: ("tool", "tools"),
            pick: Box::new(|k: &Key| k.0 == Kind::Tool),
            shape: ("[[", "]]"),
            class: "t_tool",
            id: "g_tools",
            link: "-->",
            left: false,
        },
        Group {
            title: "Tests",
            count: ("test", "tests"),
            pick: Box::new(|k: &Key| k.0 == Kind::Test),
            shape: ("([", "])"),
            class: "t_state",
            id: "g_tests",
            link: "-->",
            left: false,
        },
        Group {
            title: "Documents",
            count: ("document", "documents"),
            pick: Box::new(|k: &Key| k.0 == Kind::Document),
            shape: ("[(", ")]"),
            class: "t_doc",
            id: "g_documents",
            link: "-->",
            left: false,
        },
    ]
}

fn is_caller((kind, id): &Key) -> bool {
    *kind == Kind::Agent && id.starts_with("caller:")
}

/// Indexes into [`star_groups`] of the right-hand boxes `star` has.
fn right_groups(star: &Graph, focus: &Key) -> Vec<usize> {
    let groups = star_groups(focus);
    (1..groups.len())
        .filter(|i| star.nodes.keys().any(|k| (groups[*i].pick)(k)))
        .collect()
}

/// Every ordering of `items`, starting with `items` as given.
fn permutations(items: &[usize]) -> Vec<Vec<usize>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut all = Vec::new();
    for (i, first) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, *first);
            all.push(tail);
        }
    }
    all
}

/// The right-hand box order whose drawing has the fewest tangled lines,
/// with that drawing (uncoloured). Ties keep the natural order: handoffs,
/// tools, tests, documents.
///
/// Which order is clearest depends on how many boxes there are and how tall
/// each is, which mmdflux decides, so it is found by drawing each order (at
/// most 24, and each drawing is small) rather than by a fixed rule.
fn best_order(star: &Graph, focus: &Key, fallback: &str) -> Result<(Vec<usize>, String), CliError> {
    let mut best: Option<(usize, Vec<usize>, String)> = None;
    for order in permutations(&right_groups(star, focus)) {
        let text = draw_text(&render_star(star, focus, &order), false, fallback)?;
        let count = tangles(&text);
        if best.as_ref().is_none_or(|(fewest, _, _)| count < *fewest) {
            let done = count == 0;
            best = Some((count, order, text));
            if done {
                break;
            }
        }
    }
    let (_, order, text) = best.expect("there is always at least one order");
    Ok((order, text))
}

/// Mermaid source for one agent's [`Graph::star`], for the terminal.
///
/// Everything around the agent is grouped into one box per kind, listing
/// them: the agents that hand off to it on the left, and on the right the
/// agents it hands off to, its tools, its tests and its documents. So a star
/// has at most one line in and four out.
///
/// With a box per neighbour, an agent with a dozen tools or a few handoffs
/// needs that many parallel lines, and mmdflux gives each its own lane: they
/// cross and run alongside each other until it is unclear which line ends
/// where. `--format mermaid` keeps one node per resource.
///
/// `order` is the top-to-bottom order of the right-hand boxes, as indexes
/// into [`star_groups`] (see [`best_order`]).
fn render_star(star: &Graph, focus: &Key, order: &[usize]) -> String {
    let target = Target::Terminal;
    let mut out = String::from("flowchart LR\n");
    let mut classes: BTreeMap<&'static str, Vec<&str>> = BTreeMap::new();

    // The agent's box is a summary card: its name, then a count for each box
    // around it. Besides being useful, the count lines make the box tall
    // enough that mmdflux gives every line leaving it a row of its own; on a
    // two-line box it attaches several lines to the same row, and they cross
    // or run together on the way out.
    let groups = star_groups(focus);
    let mut lines = vec!["Agent".to_string()];
    lines.extend(wrap(
        &truncate(&star.nodes[focus].name, NODE_LABEL_MAX),
        TERMINAL_WRAP,
    ));
    for group in &groups {
        let n = star.nodes.keys().filter(|k| (group.pick)(k)).count();
        if n > 0 {
            let (one, many) = group.count;
            lines.push(format!("{n} {}", if n == 1 { one } else { many }));
        }
    }
    let lines: Vec<String> = lines.iter().map(|l| plain(l)).collect();
    let label = format!("\"{}\"", lines.join("<br/>"));
    out.push_str(&node_line("n_focus", "[", &label, "]", target));
    classes.entry("t_subagent").or_default().push("n_focus");

    let picked = [0].iter().chain(order).map(|i| &groups[*i]);
    for group in picked {
        let Group {
            title,
            pick,
            shape: (open, close),
            class,
            id: group_id,
            link,
            left,
            ..
        } = group;
        let mut items: Vec<String> = star
            .nodes
            .iter()
            .filter(|(key, _)| pick(key))
            .map(|(_, n)| {
                let name = truncate(&n.name, GROUP_ITEM_MAX);
                if n.external {
                    format!("{name} (external)")
                } else {
                    name
                }
            })
            .collect();
        if items.is_empty() {
            continue;
        }
        items.sort();
        let mut lines = vec![format!("{title} ({})", items.len())];
        lines.extend(items);
        let lines: Vec<String> = lines.iter().map(|l| plain(l)).collect();
        let label = format!("\"{}\"", lines.join("<br/>"));
        out.push_str(&node_line(group_id, open, &label, close, target));
        if *left {
            out.push_str(&format!("    {group_id} {link} n_focus\n"));
        } else {
            out.push_str(&format!("    n_focus {link} {group_id}\n"));
        }
        classes.entry(class).or_default().push(group_id);
    }
    push_classes(&mut out, &classes);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn agent(title: &str, id: &str, config: Value) -> LoadedAgent {
        LoadedAgent {
            entry: AgentConfigEntry {
                title: title.to_string(),
                config_path: format!("agent_configs/{title}.json"),
                agent_id: Some(id.to_string()),
                branch_id: None,
                branch_name: None,
            },
            config,
        }
    }

    fn branch(title: &str, id: &str, name: &str, branch_id: &str, config: Value) -> LoadedAgent {
        LoadedAgent {
            entry: AgentConfigEntry {
                title: format!("{title} (branch '{name}')"),
                config_path: format!("agent_configs/{title}.{name}.json"),
                agent_id: Some(id.to_string()),
                branch_id: Some(branch_id.to_string()),
                branch_name: Some(name.to_string()),
            },
            config,
        }
    }

    fn names(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn edge(graph: &Graph, from: (Kind, &str), to: (Kind, &str)) -> Option<Vec<String>> {
        graph
            .edges
            .get(&((from.0, from.1.to_string()), (to.0, to.1.to_string())))
            .map(|v| v.iter().cloned().collect())
    }

    #[test]
    fn every_reference_path_becomes_an_edge() {
        let config = json!({
            "conversation_config": { "agent": { "prompt": {
                "tool_ids": ["tool_a"],
                "knowledge_base": [{ "type": "file", "id": "doc_1", "name": "Pricing", "usage_mode": "auto" }],
                "built_in_tools": { "transfer_to_agent": { "type": "system", "name": "transfer_to_agent",
                    "params": { "system_tool_type": "transfer_to_agent", "transfers": [{ "agent_id": "agent_b", "condition": "billing" }] } } }
            } } },
            "platform_settings": { "testing": { "attached_tests": [{ "test_id": "test_1" }] } },
            "workflow": { "nodes": {
                "lookup": { "type": "tool", "tools": [{ "tool_id": "tool_b" }] },
                "sub": { "type": "override_agent", "label": "Sub",
                    "additional_tool_ids": ["tool_c"],
                    "additional_knowledge_base": [{ "type": "url", "id": "doc_2", "name": "FAQ" }],
                    "conversation_config": { "agent": { "prompt": { "tool_ids": ["tool_d"] } } } },
                "handoff": { "type": "standalone_agent", "agent_id": "agent_c" }
            } }
        });
        let graph = collect(
            &[agent("A", "agent_a", config)],
            &names(&[("tool_a", "Lookup")]),
            &names(&[("test_1", "Greets")]),
        );
        let a = (Kind::Agent, "agent_a");
        assert_eq!(
            edge(&graph, a, (Kind::Tool, "tool_a")).unwrap(),
            ["prompt.tool_ids"]
        );
        assert_eq!(
            edge(&graph, a, (Kind::Document, "doc_1")).unwrap(),
            ["prompt.knowledge_base"]
        );
        assert_eq!(
            edge(&graph, a, (Kind::Agent, "agent_b")).unwrap(),
            ["prompt.built_in_tools.transfer_to_agent"]
        );
        assert_eq!(
            edge(&graph, a, (Kind::Test, "test_1")).unwrap(),
            ["platform_settings.testing.attached_tests"]
        );
        assert_eq!(
            edge(&graph, a, (Kind::Tool, "tool_b")).unwrap(),
            ["workflow node 'lookup'"]
        );
        for tool in ["tool_c", "tool_d"] {
            assert_eq!(
                edge(&graph, a, (Kind::Tool, tool)).unwrap(),
                ["workflow node 'sub'"],
                "{tool}"
            );
        }
        assert_eq!(
            edge(&graph, a, (Kind::Document, "doc_2")).unwrap(),
            ["workflow node 'sub'"]
        );
        assert_eq!(
            edge(&graph, a, (Kind::Agent, "agent_c")).unwrap(),
            ["workflow node 'handoff'"]
        );
        // Names come from the registries and the locators.
        assert_eq!(graph.nodes[&(Kind::Tool, "tool_a".into())].name, "Lookup");
        assert_eq!(graph.nodes[&(Kind::Test, "test_1".into())].name, "Greets");
        assert_eq!(
            graph.nodes[&(Kind::Document, "doc_1".into())].name,
            "Pricing"
        );
    }

    #[test]
    fn ids_outside_the_project_become_external_nodes() {
        let config = json!({ "conversation_config": { "agent": { "prompt": {
            "tool_ids": ["tool_local", "tool_dashboard"],
            "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [{ "agent_id": "agent_elsewhere" }] } } }
        } } } });
        let graph = collect(
            &[agent("A", "agent_a", config)],
            &names(&[("tool_local", "Local")]),
            &BTreeMap::new(),
        );
        assert!(!graph.nodes[&(Kind::Tool, "tool_local".into())].external);
        assert!(graph.nodes[&(Kind::Tool, "tool_dashboard".into())].external);
        assert!(graph.nodes[&(Kind::Agent, "agent_elsewhere".into())].external);
        let source = render(&graph, Target::Export);
        assert!(
            source.contains(r#"n_Tool_tool_dashboard[["Tool (external) · tool_dashboard"]]"#),
            "{source}"
        );
        assert!(
            source.contains("class n_Agent_agent_elsewhere,n_Tool_tool_dashboard t_unknown"),
            "{source}"
        );
    }

    #[test]
    fn a_transfer_naming_a_registered_branch_reaches_that_branch() {
        let caller = json!({ "conversation_config": { "agent": { "prompt": {
            "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [
                { "agent_id": "agent_b", "branch_id": "agtbrch_beta" },
                { "agent_id": "agent_b", "branch_id": "agtbrch_unregistered" }
            ] } } }
        } } } });
        let graph = collect(
            &[
                agent("A", "agent_a", caller),
                agent("B", "agent_b", json!({})),
                branch("B", "agent_b", "beta", "agtbrch_beta", json!({})),
            ],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        let a = (Kind::Agent, "agent_a");
        assert!(edge(&graph, a, (Kind::Agent, "agent_b#agtbrch_beta")).is_some());
        // A branch agents.json doesn't know falls back to the main config.
        assert!(edge(&graph, a, (Kind::Agent, "agent_b")).is_some());
        assert_eq!(
            graph.nodes[&(Kind::Agent, "agent_b#agtbrch_beta".into())].name,
            "B (branch 'beta')"
        );
    }

    #[test]
    fn repeated_references_merge_into_one_edge() {
        let config = json!({
            "conversation_config": { "agent": { "prompt": { "tool_ids": ["tool_a", "tool_a"] } } },
            "workflow": { "nodes": { "n": { "type": "tool", "tools": [{ "tool_id": "tool_a" }] } } }
        });
        let graph = collect(
            &[agent("A", "agent_a", config)],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(
            edge(&graph, (Kind::Agent, "agent_a"), (Kind::Tool, "tool_a")).unwrap(),
            ["prompt.tool_ids", "workflow node 'n'"]
        );
        let source = render(&graph, Target::Export);
        assert!(source.contains(r#"-->|"via 'n'"|"#), "{source}");
    }

    #[test]
    fn uses_matches_an_id_or_a_tool_or_test_name() {
        let shared =
            json!({ "conversation_config": { "agent": { "prompt": { "tool_ids": ["tool_a"] } } } });
        let graph = collect(
            &[
                agent("A", "agent_a", shared.clone()),
                agent("B", "agent_b", shared),
            ],
            &names(&[("tool_a", "Lookup")]),
            &BTreeMap::new(),
        );
        let expected = ["agent_a\tA\tprompt.tool_ids", "agent_b\tB\tprompt.tool_ids"];
        assert_eq!(uses(&graph, "tool_a"), expected);
        assert_eq!(uses(&graph, "Lookup"), expected);
        assert!(uses(&graph, "nothing").is_empty());
    }

    #[test]
    fn scoping_keeps_one_agent_its_references_and_who_calls_it() {
        let a = json!({ "conversation_config": { "agent": { "prompt": {
            "tool_ids": ["tool_a"],
            "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [{ "agent_id": "agent_b" }] } } }
        } } } });
        let c =
            json!({ "conversation_config": { "agent": { "prompt": { "tool_ids": ["tool_c"] } } } });
        let graph = collect(
            &[
                agent("A", "agent_a", a),
                agent("B", "agent_b", json!({})),
                agent("C", "agent_c", c),
            ],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        let scoped = graph.scoped_to(&(Kind::Agent, "agent_b".into()));
        let kept: Vec<&str> = scoped.nodes.keys().map(|(_, id)| id.as_str()).collect();
        assert_eq!(kept, ["agent_a", "agent_b"]);
        let scoped = graph.scoped_to(&(Kind::Agent, "agent_a".into()));
        let kept: Vec<&str> = scoped.nodes.keys().map(|(_, id)| id.as_str()).collect();
        assert_eq!(kept, ["agent_a", "agent_b", "tool_a"]);
    }

    fn long_ids_graph() -> Graph {
        let config = json!({ "conversation_config": { "agent": { "prompt": {
            "tool_ids": ["tool_01jxcrhpk0ehdvnxw3mw4sfb67", "tool_01jxcrhpk3er4ve8nxcfprz04x", "tool_local"],
            "knowledge_base": [{ "id": "F7wzgVuiIUIKYAH2JH5s", "name": "A very long document name indeed" }]
        } } } });
        collect(
            &[agent("A", "agent_a", config)],
            &names(&[("tool_local", "a_local_tool_with_a_long_name")]),
            &BTreeMap::new(),
        )
    }

    #[test]
    fn long_ids_are_shortened_and_kept_distinct() {
        let short = shorten_ids(&long_ids_graph());
        let name = |id: &str| short.nodes[&(Kind::Tool, id.to_string())].name.clone();
        assert_eq!(name("tool_01jxcrhpk0ehdvnxw3mw4sfb67"), "tool_01j…sfb67");
        assert_eq!(name("tool_01jxcrhpk3er4ve8nxcfprz04x"), "tool_01j…rz04x");
        // Real names stay, however long.
        assert_eq!(name("tool_local"), "a_local_tool_with_a_long_name");
        assert_eq!(
            short.nodes[&(Kind::Document, "F7wzgVuiIUIKYAH2JH5s".into())].name,
            "A very long document name indeed"
        );
    }

    #[test]
    fn shortened_ids_that_would_collide_grow_until_they_differ() {
        let config = json!({ "conversation_config": { "agent": { "prompt": {
            "tool_ids": ["tool_aaaaaaaa_same_end_xyz12", "tool_aaaaaaaa_diff_end_xyz12"]
        } } } });
        let graph = collect(
            &[agent("A", "agent_a", config)],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        let short = shorten_ids(&graph);
        let names: BTreeSet<String> = short
            .nodes
            .iter()
            .filter(|((kind, _), _)| *kind == Kind::Tool)
            .map(|(_, n)| n.name.clone())
            .collect();
        assert_eq!(names.len(), 2, "{names:?}");
    }

    #[test]
    fn drawings_are_shortened_only_when_too_wide() {
        let a = (Kind::Agent, "agent_a".to_string());
        let star = long_ids_graph().star(&a);
        let full = draw_fitted(&star, &a, None, false).unwrap();
        assert!(full.contains("tool_01jxcrhpk0ehdvnxw3mw4sfb67"), "{full}");
        let roomy = draw_fitted(&star, &a, Some(500), false).unwrap();
        assert_eq!(roomy, full);
        let narrow = draw_fitted(&star, &a, Some(40), false).unwrap();
        assert!(narrow.contains("tool_01j…sfb67"), "{narrow}");
        assert!(text_width(&narrow) < text_width(&full));
    }

    #[test]
    fn the_terminal_star_puts_callers_on_their_own_side() {
        // A ⇄ B: A hands off to B, and B back to A.
        let to = |id: &str| {
            json!({ "conversation_config": { "agent": { "prompt": {
                "tool_ids": ["tool_x"],
                "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [{ "agent_id": id }] } } }
            } } } })
        };
        let graph = collect(
            &[
                agent("A", "agent_a", to("agent_b")),
                agent("B", "agent_b", to("agent_a")),
            ],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        let star = graph.star(&(Kind::Agent, "agent_a".into()));
        let a = (Kind::Agent, "agent_a".to_string());
        let edges: Vec<_> = star.edges.keys().cloned().collect();
        assert_eq!(
            edges,
            [
                (a.clone(), (Kind::Agent, "agent_b".to_string())),
                (a.clone(), (Kind::Tool, "tool_x".to_string())),
                ((Kind::Agent, "caller:agent_b".to_string()), a.clone()),
            ]
        );
        // No edge leaves anything but A, and only callers point at A: no loops.
        assert!(star.edges.keys().all(|(from, to)| from == &a || to == &a));
        assert_eq!(
            star.nodes[&(Kind::Agent, "caller:agent_b".into())].name,
            "B"
        );
        draw_fitted(&star, &a, None, false).expect("draws");
    }

    #[test]
    fn the_terminal_groups_resources_into_one_box_per_kind() {
        let a = (Kind::Agent, "agent_a".to_string());
        let star = long_ids_graph().star(&a);
        let source = render_star(&star, &a, &right_groups(&star, &a));
        assert!(
            source.contains(
                r#"g_tools[["Tools (3)<br/>a_local_tool_with_a_long_name<br/>tool_01jxcrhpk0ehdvnxw3mw4sfb67 (external)<br/>tool_01jxcrhpk3er4ve8nxcfprz04x (external)"]]"#
            ),
            "{source}"
        );
        assert!(source.contains("n_focus --> g_tools"), "{source}");
        assert!(source.contains("g_documents[(\"Documents (1)"), "{source}");
        // No kind without references gets a box.
        assert!(!source.contains("g_tests"), "{source}");
    }

    #[test]
    fn the_terminal_groups_handoffs_on_each_side() {
        // A hands off to B and to an agent outside the project; B hands back.
        let a_config = json!({ "conversation_config": { "agent": { "prompt": {
            "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [
                { "agent_id": "agent_b" }, { "agent_id": "agent_elsewhere" }
            ] } } }
        } } } });
        let b_config = json!({ "conversation_config": { "agent": { "prompt": {
            "built_in_tools": { "transfer_to_agent": { "params": { "transfers": [{ "agent_id": "agent_a" }] } } }
        } } } });
        let graph = collect(
            &[
                agent("A", "agent_a", a_config),
                agent("B", "agent_b", b_config),
            ],
            &BTreeMap::new(),
            &BTreeMap::new(),
        );
        let a = (Kind::Agent, "agent_a".to_string());
        let star = graph.star(&a);
        let source = render_star(&star, &a, &right_groups(&star, &a));
        assert!(
            source
                .contains(r#"g_handoffs["Hands off to (2)<br/>B<br/>agent_elsewhere (external)"]"#),
            "{source}"
        );
        assert!(source.contains("n_focus ==> g_handoffs"), "{source}");
        assert!(
            source.contains(r#"g_callers["Handed off from (1)<br/>B"]"#),
            "{source}"
        );
        assert!(source.contains("g_callers ==> n_focus"), "{source}");
        // One line in, one line out: nothing for mmdflux to cross.
        assert_eq!(
            source.matches("==>").count() + source.matches("-->").count(),
            2
        );
    }

    #[test]
    fn output_does_not_depend_on_input_order() {
        let forward: Value = serde_json::from_str(
            r#"{"conversation_config":{"agent":{"prompt":{"tool_ids":["t1","t2"]}}},
                "workflow":{"nodes":{"x":{"type":"tool","tools":[{"tool_id":"t3"}]},"y":{"type":"tool","tools":[{"tool_id":"t4"}]}}}}"#,
        )
        .unwrap();
        let reversed: Value = serde_json::from_str(
            r#"{"workflow":{"nodes":{"y":{"type":"tool","tools":[{"tool_id":"t4"}]},"x":{"type":"tool","tools":[{"tool_id":"t3"}]}}},
                "conversation_config":{"agent":{"prompt":{"tool_ids":["t2","t1"]}}}}"#,
        )
        .unwrap();
        let render_one = |config: Value| {
            let graph = collect(
                &[agent("A", "agent_a", config)],
                &BTreeMap::new(),
                &BTreeMap::new(),
            );
            render(&graph, Target::Export)
        };
        assert_eq!(render_one(forward), render_one(reversed));
    }
}
