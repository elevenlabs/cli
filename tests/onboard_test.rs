//! Integration tests for `elevenlabs onboard` and `onboard status` against a
//! local mock API. The browser sign-in itself is the framework's and is not
//! driven here; these cover what `status` reports for each place a key can
//! live and the refusals that must happen before a browser opens.

use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── Running the CLI with the developer's own environment kept out ──────

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

/// The CLI pointed at `server`, run in `cwd`, with `home` as `HOME`.
pub fn cli(home: &Path, cwd: &Path, server: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(["--base-url", server])
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        // The file store follows these before HOME; CI runners set the first.
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("APPDATA")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env_remove("ELEVENLABS_API_KEY")
        .env_remove("ELEVENLABS_BASE_URL")
        .env_remove("ELEVENLABS_VIA")
        .env_remove("ELEVENLABS_OUTPUT");
    cmd
}

/// Store a sign-in the way `auth login` would, via the framework's own paste path.
pub fn seed_sign_in(home: &Path, cwd: &Path, server: &str, token: &str) {
    let mut child = cli(
        home,
        cwd,
        server,
        &["auth", "login", "--with-token", "--scheme", "OAuth"],
    )
    .stdin(Stdio::piped())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("spawn auth login");
    child
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(token.as_bytes())
        .expect("write the token to auth login");
    assert!(
        child.wait().expect("auth login exits").success(),
        "seeding the sign-in failed"
    );
}

/// Errors go to stdout as JSON when piped, so failures are looked for in both streams.
pub fn all_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Parse stdout as JSON, failing with both streams when the CLI did not succeed.
pub fn json_stdout(output: &Output) -> serde_json::Value {
    assert!(output.status.success(), "{}", all_output(output));
    serde_json::from_slice(&output.stdout).expect("stdout is JSON")
}

/// `git init` plus one commit tracking `tracked`, so `ls-files` has something to find.
pub fn git_repo_tracking(dir: &Path, tracked: &str) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(dir)
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_AUTHOR_NAME", "t")
            .env("GIT_AUTHOR_EMAIL", "t@t")
            .env("GIT_COMMITTER_NAME", "t")
            .env("GIT_COMMITTER_EMAIL", "t@t")
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    std::fs::write(dir.join(tracked), "TRACKED=1\n").unwrap();
    git(&["add", tracked]);
    git(&["-c", "commit.gpgsign=false", "commit", "-q", "-m", "init"]);
}

const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";

fn temp() -> (tempfile::TempDir, tempfile::TempDir) {
    (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap())
}

/// `/v1/user` accepts exactly `key` and rejects every other credential.
async fn user_endpoint_accepting(server: &MockServer, key: &str) {
    Mock::given(method("GET"))
        .and(path("/v1/user"))
        .and(header("xi-api-key", key))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"user_id": "user_1"})))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/user"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({"detail": "nope"})))
        .mount(server)
        .await;
}

// ── status ──────────────────────────────────────────────────────────

#[tokio::test]
async fn status_with_nothing_configured_reports_that_without_asking_the_api() {
    let server = MockServer::start().await;
    let (home, cwd) = temp();

    let facts = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "status"],
        )
        .output()
        .unwrap(),
    );

    assert_eq!(facts["signed_in"], false);
    assert!(facts["browser_reachable"].is_boolean());
    assert_eq!(facts["env_file"], ".env.local");
    assert_eq!(facts["env_file_exists"], false);
    assert_eq!(facts["api_key_present"], false);
    assert_eq!(facts["api_key_source"], "none");
    assert!(facts["api_key_valid"].is_null());
    assert!(facts["api_key_last4"].is_null());
    assert_eq!(server.received_requests().await.unwrap().len(), 0);
}

#[tokio::test]
async fn status_validates_the_env_file_key_without_printing_it() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env.local"),
        format!("export ELEVENLABS_API_KEY=\"{KEY}\"\n"),
    )
    .unwrap();
    seed_sign_in(home.path(), cwd.path(), &server.uri(), "token_1");

    let out = cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "status"],
    )
    .output()
    .unwrap();

    let facts = json_stdout(&out);
    assert_eq!(facts["signed_in"], true);
    assert_eq!(facts["env_file_exists"], true);
    assert_eq!(facts["api_key_present"], true);
    assert_eq!(facts["api_key_source"], "env_file");
    assert_eq!(facts["api_key_valid"], true);
    assert_eq!(facts["api_key_last4"], "4567");
    assert!(
        !all_output(&out).contains(KEY),
        "the key never appears in the output"
    );
}

#[tokio::test]
async fn status_reports_a_rejected_key_and_one_from_the_environment() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env"),
        "ELEVENLABS_API_KEY=sk_stale_0000000000000000\n",
    )
    .unwrap();

    let facts = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "status", "--env-file", ".env"],
        )
        .output()
        .unwrap(),
    );
    assert_eq!(facts["env_file"], ".env");
    assert_eq!(facts["api_key_source"], "env_file");
    assert_eq!(facts["api_key_valid"], false, "the API said 401");

    // The shell's variable wins over the file and is reported as such.
    let facts = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "status", "--env-file", ".env"],
        )
        .env("ELEVENLABS_API_KEY", KEY)
        .output()
        .unwrap(),
    );
    assert_eq!(facts["api_key_source"], "env");
    assert_eq!(facts["api_key_valid"], true);
    assert_eq!(facts["api_key_last4"], "4567");
}

#[test]
fn status_cannot_check_a_key_when_the_api_is_unreachable() {
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env.local"),
        format!("ELEVENLABS_API_KEY={KEY}\n"),
    )
    .unwrap();

    let facts = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            "http://127.0.0.1:9",
            &["onboard", "status"],
        )
        .output()
        .unwrap(),
    );
    assert_eq!(facts["api_key_present"], true);
    assert!(
        facts["api_key_valid"].is_null(),
        "null, not false: offline is not rejected"
    );
}

// ── onboard: refusals before a browser opens ────────────────────────

/// Every refusal happens before the browser step, so a remote-looking shell
/// doubles as a way to stop the flow right after the pre-flight checks.
fn remote(home: &std::path::Path, cwd: &std::path::Path, args: &[&str]) -> std::process::Output {
    cli(
        home,
        cwd,
        "http://127.0.0.1:9",
        &[&["onboard"], args].concat(),
    )
    .env("SSH_CONNECTION", "10.0.0.1 22 10.0.0.2 22")
    .output()
    .unwrap()
}

#[test]
fn onboard_refuses_where_no_browser_can_reach_the_machine() {
    let (home, cwd) = temp();

    let out = remote(home.path(), cwd.path(), &[]);

    assert!(!out.status.success());
    let text = all_output(&out);
    assert!(text.contains("No browser can reach this machine"), "{text}");
    assert!(
        text.contains("elevenlabs.io/app/settings/api-keys"),
        "{text}"
    );
    assert!(
        !cwd.path().join(".env.local").exists(),
        "nothing is written"
    );
}

#[test]
fn onboard_refuses_a_tracked_file_and_a_symlink() {
    let (home, cwd) = temp();
    git_repo_tracking(cwd.path(), ".env");

    let out = remote(home.path(), cwd.path(), &["--env-file", ".env"]);
    assert!(!out.status.success());
    assert!(
        all_output(&out).contains("tracked by git"),
        "{}",
        all_output(&out)
    );
    assert_eq!(
        std::fs::read_to_string(cwd.path().join(".env")).unwrap(),
        "TRACKED=1\n"
    );

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(cwd.path().join("elsewhere"), cwd.path().join(".env.local"))
            .unwrap();
        let out = remote(home.path(), cwd.path(), &[]);
        assert!(!out.status.success());
        assert!(
            all_output(&out).contains("is a symlink"),
            "{}",
            all_output(&out)
        );
    }
}

#[test]
fn onboard_keeps_an_existing_key_unless_told_to_replace_it() {
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env.local"),
        format!("ELEVENLABS_API_KEY={KEY}\n"),
    )
    .unwrap();

    let out = remote(home.path(), cwd.path(), &[]);
    assert!(!out.status.success());
    let text = all_output(&out);
    assert!(
        text.contains("already sets ELEVENLABS_API_KEY (ending in 4567)"),
        "{text}"
    );
    assert!(!text.contains(KEY), "the key never appears in the output");

    // With --replace the file is no longer the reason to stop.
    let out = remote(home.path(), cwd.path(), &["--replace"]);
    assert!(
        all_output(&out).contains("No browser can reach"),
        "{}",
        all_output(&out)
    );

    // An empty placeholder line is not a key either.
    std::fs::write(cwd.path().join(".env.local"), "ELEVENLABS_API_KEY=\n").unwrap();
    let out = remote(home.path(), cwd.path(), &[]);
    assert!(
        all_output(&out).contains("No browser can reach"),
        "{}",
        all_output(&out)
    );
}

#[test]
fn onboard_help_lists_its_flags_but_the_command_itself_is_hidden() {
    let (home, cwd) = temp();

    let out = cli(
        home.path(),
        cwd.path(),
        "http://127.0.0.1:9",
        &["onboard", "--help"],
    )
    .output()
    .unwrap();

    assert!(out.status.success());
    let help = String::from_utf8_lossy(&out.stdout);
    for flag in [
        "--product",
        "--env-file",
        "--no-browser",
        "--replace",
        "status",
    ] {
        assert!(help.contains(flag), "help lacks {flag}:\n{help}");
    }
    assert!(
        !help.contains("--authorize-url"),
        "development-only flag stays hidden"
    );

    // Run by the skill, not discovered by people: absent from the top-level help.
    let top = cli(home.path(), cwd.path(), "http://127.0.0.1:9", &["--help"])
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&top.stdout).contains("onboard"));
}

#[test]
fn onboard_refuses_when_a_dotenv_in_the_tree_already_holds_a_key() {
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env"),
        format!("ELEVENLABS_API_KEY={KEY}\n"),
    )
    .unwrap();

    // The default target is .env.local, but the SDKs would keep reading .env.
    let out = remote(home.path(), cwd.path(), &[]);
    assert!(!out.status.success());
    let text = all_output(&out);
    assert!(text.contains("already sets ELEVENLABS_API_KEY"), "{text}");
    assert!(text.contains("--env-file"), "{text}");
    assert!(!text.contains(KEY), "the key never appears in the output");

    // Naming that file is the normal existing-key refusal, and --replace moves on.
    let out = remote(home.path(), cwd.path(), &["--env-file", ".env"]);
    assert!(
        all_output(&out).contains("ending in 4567"),
        "{}",
        all_output(&out)
    );
    let out = remote(home.path(), cwd.path(), &["--replace"]);
    assert!(
        all_output(&out).contains("No browser can reach"),
        "{}",
        all_output(&out)
    );
}

#[tokio::test]
async fn status_tells_a_dotenv_key_apart_from_a_shell_variable() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    std::fs::write(
        cwd.path().join(".env"),
        format!("ELEVENLABS_API_KEY={KEY}\n"),
    )
    .unwrap();

    let json = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "status"],
        )
        .output()
        .unwrap(),
    );
    assert_eq!(json["api_key_present"], true, "{json}");
    assert_eq!(json["api_key_source"], "dotenv", "{json}");
    assert_eq!(json["env_file_exists"], false, "{json}");
    assert!(
        json["dotenv_file"]
            .as_str()
            .is_some_and(|p| p.ends_with(".env")),
        "{json}"
    );
    assert_eq!(json["api_key_valid"], true, "{json}");

    // The same key named explicitly as the env file is an env_file key.
    let json = json_stdout(
        &cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "status", "--env-file", ".env"],
        )
        .output()
        .unwrap(),
    );
    assert_eq!(json["api_key_source"], "env_file", "{json}");
    assert_eq!(json["dotenv_file"], serde_json::Value::Null, "{json}");
}
