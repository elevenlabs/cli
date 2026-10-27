//! Integration tests for `elevenlabs agents workflow diagram` and `export`.
//!
//! The renderer itself is unit-tested in `cli/elevenlabs/workflow/diagram.rs`;
//! these cover what only the binary can: resolving an agent through
//! `agents.json`, `--all` / `--markdown` output, and that the command runs
//! with no API key at all (it is meant for CI on forks, where there is none).

use std::path::Path;
use std::process::{Command, Output};

/// Run the CLI in `project` with no credentials, and `HOME` redirected so a
/// developer's own config cannot supply any.
fn cli(project: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .current_dir(project)
        .env("HOME", project)
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env_remove("ELEVENLABS_API_KEY")
        .env_remove("ELEVENLABS_BASE_URL")
        .env_remove("ELEVENLABS_VIA")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A project with one agent that has a workflow, one branch of it, and one
/// agent that has none.
fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir(root.join("agent_configs")).unwrap();
    let workflow = serde_json::json!({
        "nodes": {
            "start_node": { "type": "start" },
            "greet": { "type": "say", "label": "Greet", "message": { "type": "literal", "text": "Hi" } }
        },
        "edges": {
            "e": { "source": "start_node", "target": "greet", "forward_condition": { "type": "unconditional" } }
        }
    });
    let write = |path: &str, value: serde_json::Value| {
        std::fs::write(
            root.join(path),
            serde_json::to_string_pretty(&value).unwrap(),
        )
        .unwrap();
    };
    write(
        "agent_configs/support.json",
        serde_json::json!({ "name": "Support", "workflow": workflow }),
    );
    write(
        "agent_configs/support.beta.json",
        serde_json::json!({ "name": "Support", "workflow": workflow }),
    );
    write(
        "agent_configs/plain.json",
        serde_json::json!({ "name": "Plain", "conversation_config": {} }),
    );
    write(
        "agents.json",
        serde_json::json!({ "agents": [
            {
                "config": "agent_configs/support.json",
                "id": "agent_support",
                "branches": { "beta": { "config": "agent_configs/support.beta.json", "branch_id": "agtbrch_1" } }
            },
            { "config": "agent_configs/plain.json", "id": "agent_plain" }
        ]}),
    );
    dir
}

#[test]
fn a_file_an_id_and_a_name_render_the_same_diagram() {
    let dir = project();
    let by_file = cli(
        dir.path(),
        &["agents", "workflow", "export", "agent_configs/support.json"],
    );
    assert!(by_file.status.success(), "{by_file:?}");
    let diagram = stdout(&by_file);
    assert!(diagram.starts_with("flowchart TD\n"), "{diagram}");
    assert!(diagram.contains(r#"n_greet("Say · Greet")"#), "{diagram}");
    assert!(diagram.contains("n_start_node --> n_greet"), "{diagram}");

    for target in ["agent_support", "Support"] {
        let out = cli(dir.path(), &["agents", "workflow", "export", target]);
        assert!(out.status.success(), "{target}: {out:?}");
        assert_eq!(stdout(&out), diagram, "{target}");
    }
}

#[test]
fn markdown_wraps_the_diagram_in_a_mermaid_fence() {
    let dir = project();
    let out = cli(
        dir.path(),
        &["agents", "workflow", "export", "Support", "--markdown"],
    );
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    assert!(text.starts_with("```mermaid\nflowchart TD\n"), "{text}");
    assert!(text.ends_with("```\n"), "{text}");
}

#[test]
fn all_renders_every_agent_and_branch_with_a_workflow() {
    let dir = project();
    let out = cli(
        dir.path(),
        &["agents", "workflow", "export", "--all", "--markdown"],
    );
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    assert!(text.contains("### Support\n"), "{text}");
    assert!(text.contains("### Support (branch 'beta')\n"), "{text}");
    assert_eq!(text.matches("```mermaid").count(), 2, "{text}");
    // The agent without a workflow is skipped, and says so on stderr.
    assert!(!text.contains("Plain"), "{text}");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("Skipping 'Plain'"), "{err}");
}

#[test]
fn an_agent_without_a_workflow_is_an_error() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "workflow", "export", "Plain"]);
    assert!(!out.status.success(), "{out:?}");
    // Errors are reported as JSON on stdout, like every other command's.
    let err = stdout(&out);
    assert!(err.contains("has no workflow"), "{err}");
}

#[test]
fn an_unknown_target_is_an_error() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "workflow", "export", "nobody"]);
    assert!(!out.status.success(), "{out:?}");
    // Errors are reported as JSON on stdout, like every other command's.
    let err = stdout(&out);
    assert!(err.contains("'nobody'"), "{err}");
}

#[test]
fn a_target_or_all_is_required() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "workflow", "export"]);
    assert!(!out.status.success(), "{out:?}");
}

#[test]
fn diagram_draws_the_workflow_as_text() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "workflow", "diagram", "Support"]);
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    // Box-drawing, not Mermaid source.
    assert!(!text.contains("flowchart"), "{text}");
    assert!(text.contains("│ Start │"), "{text}");
    assert!(text.contains("Greet"), "{text}");
    // Not a terminal (and NO_COLOR is set), so no ANSI escapes.
    assert!(!text.contains('\u{1b}'), "{text}");
}

#[test]
fn diagram_all_draws_each_agent_under_a_heading() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "workflow", "diagram", "--all"]);
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    assert!(
        text.contains("── Support (agent_configs/support.json)"),
        "{text}"
    );
    assert!(
        text.contains("── Support (branch 'beta') (agent_configs/support.beta.json)"),
        "{text}"
    );
}
