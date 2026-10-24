//! The `agents workflow diagram` command: render the `workflow` of a local
//! agent config as a Mermaid flowchart on stdout.
//!
//! Issue #170: `workflow.nodes` / `workflow.edges` are stored as raw JSON,
//! which is hard to review in a diff. Mermaid renders natively on GitHub and
//! in most docs tooling. The command reads the file as untyped JSON so node
//! types the CLI does not know yet still render. It needs no API key and no
//! network.
//!
//! Related: [`super::agents`] (the `agents` group this nests under),
//! [`super::project`] (`read_value`), `README.md` ("Agents as Code").

use std::path::Path;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use serde_json::Value;

use super::project;

/// Edge labels longer than this many characters are cut and end in `…`.
const MAX_LABEL_CHARS: usize = 40;

/// Node types with their own `classDef`. Any other type renders as `other`.
/// Emitted class names carry the `wf_` prefix because `end` is a Mermaid
/// flowchart keyword and as a class name it is unsafe across versions.
const KNOWN_TYPES: &[(&str, &str)] = &[
    ("start", "#d4edda,stroke:#28a745"),
    ("end", "#f8d7da,stroke:#dc3545"),
    ("say", "#d1ecf1,stroke:#17a2b8"),
    ("tool", "#fff3cd,stroke:#ffc107"),
    ("update_state", "#e2d9f3,stroke:#6f42c1"),
    ("override_agent", "#cce5ff,stroke:#007bff"),
    ("standalone_agent", "#d6d8db,stroke:#6c757d"),
    ("phone_number", "#ffe5d0,stroke:#fd7e14"),
];
const OTHER_STYLE: &str = "#ffffff,stroke:#999999,stroke-dasharray:4 2";

/// Register `agents workflow diagram <config-path>`.
pub fn register(app: CliApp) -> CliApp {
    // Nested under `agents workflow` so the verb stays clear of the
    // API-owned names in the `agents` group (see README "Two surfaces").
    app.command_under(
        &["agents", "workflow"],
        clap::Command::new("diagram")
            .about("Print an agent config's workflow as a Mermaid flowchart")
            .arg(
                clap::Arg::new("config-path")
                    .required(true)
                    .help("Path to an agent config JSON file"),
            ),
        Box::new(|matches, _ctx| {
            let path = matches
                .get_one::<String>("config-path")
                .expect("config-path is required");
            let config = project::read_value(Path::new(path))?;
            print!("{}", render(&config)?);
            Ok(())
        }),
    )
}

/// Render the config's `workflow` as Mermaid text.
fn render(config: &Value) -> Result<String, CliError> {
    let workflow = config
        .get("workflow")
        .filter(|w| w.is_object())
        .ok_or_else(|| CliError::Validation("This config has no `workflow` to draw".to_string()))?;
    let nodes = sorted_entries(workflow.get("nodes"));
    let edges = sorted_entries(workflow.get("edges"));

    let mut out = String::from("flowchart TD\n");
    let mut used: Vec<&str> = Vec::new();
    for (id, node) in &nodes {
        let kind = node.get("type").and_then(Value::as_str).unwrap_or("");
        let class = KNOWN_TYPES
            .iter()
            .map(|(k, _)| *k)
            .find(|k| *k == kind)
            .unwrap_or("other");
        if !used.contains(&class) {
            used.push(class);
        }
        let label = node.get("label").and_then(Value::as_str).unwrap_or(id);
        out.push_str(&format!(
            "    {}[\"{}\"]:::wf_{class}\n",
            node_id(id),
            escape(label)
        ));
    }
    for (_, edge) in &edges {
        let source = edge.get("source").and_then(Value::as_str).unwrap_or("");
        let target = edge.get("target").and_then(Value::as_str).unwrap_or("");
        let arrow = match edge_label(edge) {
            Some(label) => format!("-->|\"{}\"|", escape(&label)),
            None => "-->".to_string(),
        };
        out.push_str(&format!(
            "    {} {arrow} {}\n",
            node_id(source),
            node_id(target)
        ));
    }
    for (kind, style) in KNOWN_TYPES.iter().chain([&("other", OTHER_STYLE)]) {
        if used.contains(kind) {
            out.push_str(&format!("    classDef wf_{kind} fill:{style}\n"));
        }
    }
    Ok(out)
}

/// `(key, value)` pairs of a JSON object, sorted by key; empty if absent.
fn sorted_entries(map: Option<&Value>) -> Vec<(&String, &Value)> {
    let mut entries: Vec<_> = map
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .collect();
    entries.sort_by_key(|(k, _)| *k);
    entries
}

/// Mermaid id: `n_` plus the node id with non-word characters replaced.
/// The prefix keeps ids such as `end` clear of Mermaid's reserved words.
fn node_id(id: &str) -> String {
    let safe: String = id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("n_{safe}")
}

/// Text for an edge label: the forward condition's `label`, else its
/// `condition` text. `None` for unconditional or unlabelled edges.
fn edge_label(edge: &Value) -> Option<String> {
    let cond = edge.get("forward_condition")?;
    let text = ["label", "condition"]
        .iter()
        .filter_map(|k| cond.get(*k).and_then(Value::as_str))
        .find(|s| !s.trim().is_empty())?;
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() > MAX_LABEL_CHARS {
        let cut: String = text.chars().take(MAX_LABEL_CHARS).collect();
        Some(format!("{}…", cut.trim_end()))
    } else {
        Some(text)
    }
}

/// Escape a label for use inside a double-quoted Mermaid string.
fn escape(text: &str) -> String {
    text.replace('"', "#quot;")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn renders_a_linear_workflow() {
        let config = json!({"workflow": {
            "nodes": {
                "start_node": {"type": "start", "label": "Start"},
                "greet": {"type": "say", "label": "Greet"},
                "end": {"type": "end", "label": "End"}
            },
            "edges": {
                "e2": {"source": "greet", "target": "end", "forward_condition": {"type": "unconditional"}},
                "e1": {"source": "start_node", "target": "greet"}
            }
        }});
        assert_eq!(
            render(&config).unwrap(),
            "flowchart TD\n\
             \x20   n_end[\"End\"]:::wf_end\n\
             \x20   n_greet[\"Greet\"]:::wf_say\n\
             \x20   n_start_node[\"Start\"]:::wf_start\n\
             \x20   n_start_node --> n_greet\n\
             \x20   n_greet --> n_end\n\
             \x20   classDef wf_start fill:#d4edda,stroke:#28a745\n\
             \x20   classDef wf_end fill:#f8d7da,stroke:#dc3545\n\
             \x20   classDef wf_say fill:#d1ecf1,stroke:#17a2b8\n"
        );
    }

    #[test]
    fn renders_branches_unknown_types_and_cuts_long_labels() {
        let config = json!({"workflow": {
            "nodes": {
                "a": {"type": "start", "label": "Say \"hi\""},
                "b": {"type": "hologram"},
                "c": {"type": "tool", "label": "Lookup"}
            },
            "edges": {
                "e1": {"source": "a", "target": "b", "forward_condition": {
                    "type": "llm",
                    "condition": "The caller asks about billing, refunds or an invoice they did not recognise"
                }},
                "e2": {"source": "a", "target": "c", "forward_condition": {
                    "type": "llm", "label": "Wants lookup", "condition": "ignored"
                }}
            }
        }});
        assert_eq!(
            render(&config).unwrap(),
            "flowchart TD\n\
             \x20   n_a[\"Say #quot;hi#quot;\"]:::wf_start\n\
             \x20   n_b[\"b\"]:::wf_other\n\
             \x20   n_c[\"Lookup\"]:::wf_tool\n\
             \x20   n_a -->|\"The caller asks about billing, refunds o…\"| n_b\n\
             \x20   n_a -->|\"Wants lookup\"| n_c\n\
             \x20   classDef wf_start fill:#d4edda,stroke:#28a745\n\
             \x20   classDef wf_tool fill:#fff3cd,stroke:#ffc107\n\
             \x20   classDef wf_other fill:#ffffff,stroke:#999999,stroke-dasharray:4 2\n"
        );
    }

    #[test]
    fn config_without_workflow_is_a_validation_error() {
        assert!(matches!(
            render(&json!({"name": "x"})),
            Err(CliError::Validation(_))
        ));
    }
}
