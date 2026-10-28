//! `--format` on agents-as-code push/pull commands, run through the real binary.
//!
//! `--format` is a global flag declared in `src/app.rs`, so clap accepts it on
//! every command. Push/pull read and write local JSON configs and never print
//! an API response, so they declare `ignores_format` at registration and
//! `CliApp` dispatch warns on stderr. These tests drive the built binary — the
//! same command tree `main` builds — so a change to how the global flag is
//! attached, or a dropped declaration, fails here.
//!
//! No network: `ELEVENLABS_BASE_URL` points at a closed localhost port, so a
//! pull that gets as far as the API fails with a connection error, and each
//! run happens in an empty temp directory. The warning prints before the
//! handler runs, so the handler's own outcome does not matter here.

use std::path::Path;
use std::process::{Command, Output};

fn cli(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .current_dir(dir)
        .env("HOME", dir)
        .env("ELEVENLABS_API_KEY", "test-key")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("ELEVENLABS_BASE_URL", "http://127.0.0.1:9")
        .env_remove("ELEVENLABS_VIA")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

fn stderr_of(args: &[&str]) -> String {
    let dir = tempfile::tempdir().expect("tempdir");
    String::from_utf8_lossy(&cli(dir.path(), args).stderr).to_string()
}

fn warning(command: &str, noun: &str, verb: &str) -> String {
    format!(
        "--format controls how API responses are printed and has no effect on {command}; {noun} are always {verb} as JSON.\n"
    )
}

#[test]
fn agents_pull_warns_with_format_after_the_command() {
    let err = stderr_of(&["agents", "pull", "--format", "yaml"]);
    assert!(
        err.starts_with(&warning("agents pull", "agent configs", "written")),
        "{err}"
    );
}

#[test]
fn agents_pull_warns_with_format_before_the_command() {
    let err = stderr_of(&["--format", "yaml", "agents", "pull"]);
    assert!(
        err.starts_with(&warning("agents pull", "agent configs", "written")),
        "{err}"
    );
}

#[test]
fn tools_push_warns_with_its_own_noun() {
    let err = stderr_of(&["tools", "push", "--format", "json"]);
    assert!(
        err.starts_with(&warning("tools push", "tool configs", "read")),
        "{err}"
    );
}

#[test]
fn tests_pull_warns_with_its_own_noun() {
    let err = stderr_of(&["tests", "pull", "--format", "table"]);
    assert!(
        err.starts_with(&warning("tests pull", "test configs", "written")),
        "{err}"
    );
}

#[test]
fn no_warning_without_format() {
    let err = stderr_of(&["agents", "pull"]);
    assert!(!err.contains("--format"), "{err}");
}
