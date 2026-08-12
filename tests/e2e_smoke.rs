//! End-to-end smoke test for the agents-as-code workflow, run against a real
//! ElevenLabs account.
//!
//! # ⚠ DESTRUCTIVE — use a dedicated, empty test account
//!
//! This test **creates and deletes agents**. Point it at a throwaway account,
//! never a production one. It is opt-in for exactly that reason: it does
//! nothing unless `ELEVENLABS_E2E_API_KEY` is set, so `cargo test` on a
//! developer machine or in CI skips it by default. Note it deliberately does
//! *not* read `ELEVENLABS_API_KEY` — otherwise a normal shell with credentials
//! already exported would start mutating a live workspace.
//!
//! ```bash
//! ELEVENLABS_E2E_API_KEY=xi-... cargo test --test e2e_smoke -- --nocapture
//! ```
//!
//! # What it proves
//!
//! The thing unit and wire tests can't: that a config survives a real
//! round-trip through the API. `push` then `pull` twice and compare — the
//! second and third snapshots must be byte-identical, which is the guarantee
//! the whole raw-JSON design exists to provide (see `workflow/api.rs`). It also
//! exercises init → add → push → pull → delete against live endpoints.

use std::path::Path;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const OPT_IN_VAR: &str = "ELEVENLABS_E2E_API_KEY";

/// Run the CLI in `dir` with the E2E credentials, returning the output.
///
/// `HOME` is redirected at the temp dir so the run can't pick up (or write) the
/// developer's `~/.elevenlabs` residency setting.
fn cli(dir: &Path, home: &Path, api_key: &str, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .current_dir(dir)
        .env("ELEVENLABS_API_KEY", api_key)
        .env("HOME", home)
        .env("NO_COLOR", "1")
        // Never let an ambient override point the test somewhere unexpected.
        .env_remove("ELEVENLABS_BASE_URL")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn assert_ok(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} failed (exit {:?})\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The agent id recorded in `agents.json` after `agents add`.
fn agent_id_from_registry(dir: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(dir.join("agents.json")).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    json.get("agents")?
        .as_array()?
        .first()?
        .get("id")?
        .as_str()
        .map(String::from)
}

fn config_path_from_registry(dir: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(dir.join("agents.json")).ok()?;
    let json: serde_json::Value = serde_json::from_str(&raw).ok()?;
    json.get("agents")?
        .as_array()?
        .first()?
        .get("config")?
        .as_str()
        .map(String::from)
}

#[test]
fn agents_as_code_round_trips_against_a_live_account() {
    let Ok(api_key) = std::env::var(OPT_IN_VAR) else {
        eprintln!("skipping: set {OPT_IN_VAR} to run the live E2E smoke test");
        return;
    };

    let workdir = tempfile::tempdir().expect("temp workdir");
    let homedir = tempfile::tempdir().expect("temp home");
    let dir = workdir.path();
    let home = homedir.path();

    // Unique name so parallel or repeated runs can't collide.
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs();
    let agent_name = format!("cli-e2e-{nonce}");

    // 1. Scaffold a project.
    assert_ok(&cli(dir, home, &api_key, &["agents", "init"]), "agents init");
    assert!(dir.join("agents.json").exists(), "init wrote agents.json");
    assert!(dir.join("agent_configs").is_dir(), "init made agent_configs/");

    // 2. Create the agent remotely and register it locally.
    let add = cli(
        dir,
        home,
        &api_key,
        &["agents", "add", &agent_name, "--template", "minimal"],
    );

    // From here on the agent exists remotely, so always clean up.
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_ok(&add, "agents add");

        let agent_id = agent_id_from_registry(dir)
            .unwrap_or_else(|| panic!("no agent id in agents.json:\n{}", stdout_of(&add)));
        let config_path = config_path_from_registry(dir).expect("config path in agents.json");
        let config_file = dir.join(&config_path);
        assert!(config_file.exists(), "add wrote {config_path}");

        // 3. Status should now see it as created.
        let status = cli(dir, home, &api_key, &["agents", "status"]);
        assert_ok(&status, "agents status");
        assert!(
            stdout_of(&status).contains(&agent_id),
            "status should report the agent id, got:\n{}",
            stdout_of(&status)
        );

        // 4. Push the local config, then pull it back — this is the snapshot the
        //    round-trip is measured from (the server fills defaults on first
        //    write, so the pre-push file is not a fair comparison point).
        assert_ok(&cli(dir, home, &api_key, &["agents", "push"]), "agents push");
        assert_ok(
            &cli(dir, home, &api_key, &["agents", "pull", "--agent", &agent_id, "--all"]),
            "agents pull (first)",
        );
        let first = std::fs::read_to_string(&config_file).expect("read config after first pull");

        // 5. Push that exact config back and pull again. A lossless round-trip
        //    means nothing changed.
        assert_ok(&cli(dir, home, &api_key, &["agents", "push"]), "agents push (second)");
        assert_ok(
            &cli(dir, home, &api_key, &["agents", "pull", "--agent", &agent_id, "--all"]),
            "agents pull (second)",
        );
        let second = std::fs::read_to_string(&config_file).expect("read config after second pull");

        assert_eq!(
            first, second,
            "config changed across a push/pull round-trip — the raw-JSON path is dropping or \
             rewriting fields.\n--- first ---\n{first}\n--- second ---\n{second}"
        );

        // 6. The widget snippet should reference the live agent.
        let widget = cli(
            dir,
            home,
            &api_key,
            &["agents", "widget", "embed", &agent_id],
        );
        assert_ok(&widget, "agents widget embed");
        assert!(
            stdout_of(&widget).contains(&agent_id),
            "widget snippet should embed the agent id"
        );

        agent_id
    }));

    // 7. Always remove the agent we created, then surface any failure.
    //    Deletion is a generated API command, so the path parameter is a flag
    //    (`--agent-id`) rather than a positional.
    if let Some(agent_id) = agent_id_from_registry(dir) {
        let cleanup = cli(dir, home, &api_key, &["agents", "delete", "--agent-id", &agent_id]);
        if !cleanup.status.success() {
            eprintln!(
                "WARNING: could not delete E2E agent {agent_id} — remove it by hand.\n{}",
                String::from_utf8_lossy(&cleanup.stderr)
            );
        }
    }

    if let Err(panic) = outcome {
        std::panic::resume_unwind(panic);
    }
}

/// Guards against the opt-in being wired to the wrong variable — if this ever
/// starts reading `ELEVENLABS_API_KEY`, a normal developer shell would begin
/// mutating a live workspace on `cargo test`.
#[test]
fn the_live_test_is_opt_in_via_a_dedicated_variable() {
    assert_eq!(OPT_IN_VAR, "ELEVENLABS_E2E_API_KEY");
    assert_ne!(OPT_IN_VAR, "ELEVENLABS_API_KEY");
}
