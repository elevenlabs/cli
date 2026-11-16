//! Integration tests for `elevenlabs agents graph`.
//!
//! The reference-following and rendering are unit-tested in
//! `cli/elevenlabs/workflow/graph.rs`; these run the binary against the
//! fixture project in `cli/elevenlabs/workflow/fixtures/graph/` (copied into
//! a temp dir), with no API key: the command reads local files only.

use std::path::Path;
use std::process::{Command, Output};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/elevenlabs/workflow/fixtures/graph"
);

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

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// A fresh copy of the fixture project.
fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    copy_dir(Path::new(FIXTURE), dir.path());
    dir
}

#[test]
fn mermaid_output_matches_the_snapshot() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "graph", "--format", "mermaid"]);
    assert!(out.status.success(), "{out:?}");
    let expected = std::fs::read_to_string(Path::new(FIXTURE).join("expected.mmd")).unwrap();
    assert_eq!(stdout(&out), expected);
}

#[test]
fn markdown_wraps_the_mermaid_in_a_fence() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "graph", "--format", "mermaid-md"]);
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    assert!(text.starts_with("```mermaid\nflowchart LR\n"), "{text}");
    assert!(text.ends_with("```\n"), "{text}");
}

#[test]
fn uses_does_not_take_a_format() {
    let dir = project();
    let out = cli(
        dir.path(),
        &[
            "agents",
            "graph",
            "--uses",
            "tool_refund",
            "--format",
            "mermaid",
        ],
    );
    assert!(!out.status.success(), "{out:?}");
}

#[test]
fn the_terminal_draws_one_section_per_agent() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "graph"]);
    assert!(out.status.success(), "{out:?}");
    let text = stdout(&out);
    assert!(!text.contains("flowchart"), "{text}");
    for heading in [
        "── Support (agent_configs/support.json)",
        "── Support (branch 'beta') (agent_configs/support.beta.json)",
        "── Billing (agent_configs/billing.json)",
    ] {
        assert!(text.contains(heading), "{heading}: {text}");
    }
    // Tools are listed in one box per agent, external ones marked.
    assert!(text.contains("Tools (2)"), "{text}");
    assert!(text.contains("lookup_order"), "{text}");
    assert!(text.contains("tool_from_dashboard (external)"), "{text}");
    assert!(!text.contains('\u{1b}'), "{text}");
    // No line crosses or runs into another, in any agent's drawing.
    let tangled: Vec<char> = text.chars().filter(|c| "┼╋├┤┬┴┣┫┳┻".contains(*c)).collect();
    assert!(
        tangled.is_empty(),
        "lines cross or merge ({tangled:?}): {text}"
    );
}

#[test]
fn one_agent_by_name_id_or_config_path() {
    let dir = project();
    let by_name = cli(
        dir.path(),
        &["agents", "graph", "Billing", "--format", "mermaid"],
    );
    assert!(by_name.status.success(), "{by_name:?}");
    let source = stdout(&by_name);
    // Billing's own references, and the two agents that hand off to it.
    assert!(source.contains("Tool · issue_refund"), "{source}");
    assert!(
        source.contains("Agent · Support (branch 'beta')"),
        "{source}"
    );
    assert!(!source.contains("lookup_order"), "{source}");

    for target in ["agent_billing", "agent_configs/billing.json"] {
        let out = cli(
            dir.path(),
            &["agents", "graph", target, "--format", "mermaid"],
        );
        assert!(out.status.success(), "{target}: {out:?}");
        assert_eq!(stdout(&out), source, "{target}");
    }
}

#[test]
fn uses_lists_dependents_tab_separated() {
    let dir = project();
    let out = cli(dir.path(), &["agents", "graph", "--uses", "tool_refund"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(
        stdout(&out),
        "agent_billing\tBilling\tprompt.tool_ids\n\
         agent_support\tSupport (branch 'beta')\tworkflow node 'refunds'\n"
    );
    // A tool's local name works too.
    let by_name = cli(dir.path(), &["agents", "graph", "--uses", "issue_refund"]);
    assert_eq!(stdout(&by_name), stdout(&out));
}

#[test]
fn uses_with_no_dependents_says_so_and_succeeds() {
    let dir = project();
    let out = cli(
        dir.path(),
        &["agents", "graph", "--uses", "tool_nobody_uses"],
    );
    assert!(out.status.success(), "{out:?}");
    assert!(stdout(&out).is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("No agent in agents.json uses"));
}

#[test]
fn a_project_without_tools_json_shows_its_tools_as_external() {
    let dir = project();
    std::fs::remove_file(dir.path().join("tools.json")).unwrap();
    let out = cli(
        dir.path(),
        &["agents", "graph", "Billing", "--format", "mermaid"],
    );
    assert!(out.status.success(), "{out:?}");
    let source = stdout(&out);
    assert!(source.contains("Tool (external) · tool_refund"), "{source}");
}
