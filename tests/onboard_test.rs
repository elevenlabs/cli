//! Integration tests for `elevenlabs onboard` (`init`, `connect`, `check`,
//! `test`) against a local mock API. The clipboard hand-off needs a real
//! approval page and clipboard, so it is covered by the onboarding harness
//! (and `store_key`'s outcomes by the unit tests); these run the real binary
//! for every reply that doesn't, and the background sign-in up to a declined
//! approval. They never touch the system clipboard, a browser, or GitHub.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";
const VOICE: &str = "JBFqnCBsd6RMkjVDRZzb";
const OFFLINE: &str = "http://127.0.0.1:9";

/// The CLI pointed at `server`, run in `cwd`, with `home` as `HOME` and the
/// developer's own environment kept out (including the agent it runs in).
fn cli(home: &Path, cwd: &Path, server: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(["--base-url", server])
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env("TMPDIR", home)
        // The file store follows these before HOME; CI runners set the first.
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("APPDATA")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("ELEVENLABS_ONBOARD_NO_CLIPBOARD", "1")
        // A browser reaches this machine unless a test says otherwise.
        .env("DISPLAY", ":0");
    for var in [
        "ELEVENLABS_API_KEY",
        "ELEVENLABS_BASE_URL",
        "ELEVENLABS_VIA",
        "ELEVENLABS_OUTPUT",
        "SSH_CONNECTION",
        "SSH_TTY",
        "SSH_CLIENT",
        "CODESPACES",
        "REMOTE_CONTAINERS",
        "CLAUDECODE",
        "CODEX_THREAD_ID",
        "CURSOR_AGENT",
        "npm_command",
    ] {
        cmd.env_remove(var);
    }
    cmd
}

fn all_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Run and parse stdout as the reply, failing with both streams when the CLI did not succeed.
fn reply(cmd: &mut Command) -> Value {
    let output = cmd.output().unwrap();
    assert!(output.status.success(), "{}", all_output(&output));
    assert!(
        !all_output(&output).contains(KEY),
        "the key never appears in the output"
    );
    serde_json::from_slice(&output.stdout).expect("stdout is JSON")
}

fn say(reply: &Value) -> &str {
    reply["say"].as_str().unwrap_or_default()
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        // Nor the developer's default excludes file (~/.config/git/ignore).
        .env("HOME", dir)
        .env_remove("XDG_CONFIG_HOME")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

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
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"detail": {"status": "invalid_api_key"}})),
        )
        .mount(server)
        .await;
}

fn write(dir: &Path, file: &str, text: &str) {
    let path = dir.join(file);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

// ── check ───────────────────────────────────────────────────────────

#[tokio::test]
async fn check_with_nothing_configured_says_to_connect_without_asking_the_api() {
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "run");
    assert_eq!(r["run"], "elevenlabs onboard connect --env-file .env.local");
    assert_eq!(server.received_requests().await.unwrap().len(), 0);
    // `status` is the same command.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "status"],
    ));
    assert_eq!(r["run"], "elevenlabs onboard connect --env-file .env.local");
}

/// While a sign-in for the project is open, `check` and `init` wait on it
/// rather than starting another. The open sign-in is stood in for by its held lock.
#[test]
fn check_and_init_wait_on_an_open_sign_in() {
    use sha2::Digest;
    let (home, cwd) = temp();
    let canonical = cwd.path().canonicalize().unwrap();
    let id = sha2::Sha256::digest(canonical.to_string_lossy().as_bytes());
    let id: String = id.iter().take(8).map(|b| format!("{b:02x}")).collect();
    let session = home.path().join(".elevenlabs/onboard").join(id);
    std::fs::create_dir_all(&session).unwrap();
    let lock = std::fs::File::create(session.join("worker.lock")).unwrap();
    lock.lock().unwrap();
    let url = "http://127.0.0.1:9/app/oauth/authorize?x=1";
    std::fs::write(session.join("worker.log"), format!("  URL: {url}\n")).unwrap();

    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "check"],
    ));
    assert_eq!(
        (r["next"].as_str(), r["run"].as_str()),
        (
            Some("wait"),
            Some("elevenlabs onboard connect --wait --env-file .env.local")
        ),
        "{r}"
    );
    assert!(say(&r).contains(url), "{r}");
    // So does init, run again mid-approval.
    let again = reply(&mut cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "init"],
    ));
    assert_eq!(again["next"], "wait", "{again}");

    lock.unlock().unwrap();
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "check"],
    ));
    assert_eq!(
        r["run"], "elevenlabs onboard connect --env-file .env.local",
        "{r}"
    );
}

#[test]
fn check_on_a_remote_machine_needs_no_home_folder() {
    let (home, cwd) = temp();
    let r = reply(
        cli(home.path(), cwd.path(), OFFLINE, &["onboard", "check"])
            .env_remove("HOME")
            .env_remove("USERPROFILE")
            .env("SSH_CONNECTION", "10.0.0.1 52000 10.0.0.2 22"),
    );
    assert_eq!(r["next"], "ask_user", "the manual key steps: {r}");
}

#[tokio::test]
async fn check_verifies_the_key_secures_the_file_and_asks_for_the_code() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    write(
        cwd.path(),
        ".env.local",
        &format!("export ELEVENLABS_API_KEY=\"{KEY}\"\n"),
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            cwd.path().join(".env.local"),
            std::fs::Permissions::from_mode(0o644),
        )
        .unwrap();
    }

    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "write_code", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --env-file .env.local");
    assert_eq!(r["details"]["api_key_source"], "env_file");
    assert_eq!(r["details"]["key_last4"], "4567");
    assert_eq!(r["details"]["voice_id"], VOICE);
    assert!(
        r["details"]["load_key"]
            .as_str()
            .unwrap()
            .contains("--env-file=.env.local"),
        "{r}"
    );
    assert!(
        r["details"]["what_to_build"]
            .as_str()
            .unwrap()
            .contains("ask them before writing any code"),
        "{r}"
    );
    let fixed = r["details"]["fixed"].to_string();
    assert!(fixed.contains(".gitignore"), "{fixed}");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert!(fixed.contains("readable only by you"), "{fixed}");
        let mode = std::fs::metadata(cwd.path().join(".env.local"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    // With the SDK declared and imported, the next step is offering the test.
    write(
        cwd.path(),
        "package.json",
        r#"{"dependencies":{"@elevenlabs/elevenlabs-js":"^2"}}"#,
    );
    write(
        cwd.path(),
        "src/speak.js",
        "import { ElevenLabsClient } from \"@elevenlabs/elevenlabs-js\";\n",
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "offer_test", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard test --env-file .env.local");
    assert!(say(&r).contains("Want me to send a test request?"), "{r}");
    assert_eq!(r["details"]["fixed"], json!([]), "nothing left to fix");
}

#[tokio::test]
async fn a_key_that_cannot_read_the_account_still_works() {
    // Keys made by hand on the API keys page may lack `user_read`, which only the check needs.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/user"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"detail": {"status": "missing_permissions"}})),
        )
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "write_code", "{r}");
}

#[tokio::test]
async fn an_error_from_elevenlabs_is_not_a_rejected_key() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/user"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(say(&r).contains("answered 503"), "{r}");
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "check"],
    ));
    assert!(say(&r).contains("couldn't be reached"), "{r}");
}

#[tokio::test]
async fn check_stops_at_a_tracked_env_file() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    git(cwd.path(), &["add", ".env.local"]);
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "fix", "{r}");
    assert!(say(&r).contains("git rm --cached .env.local"), "{r}");
}

#[tokio::test]
async fn check_says_how_to_replace_a_rejected_key_wherever_it_lives() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env",
        "ELEVENLABS_API_KEY=sk_stale_0000000000000000\n",
    );

    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--env-file", ".env"],
    ));
    assert_eq!(r["next"], "run", "{r}");
    assert_eq!(
        r["run"],
        "elevenlabs onboard connect --replace --env-file .env"
    );

    // A rejected shell variable can only be fixed in the shell.
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "check", "--env-file", ".env"],
        )
        .env("ELEVENLABS_API_KEY", "sk_shell_000000000000000000"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(say(&r).contains("set in your shell"), "{r}");

    // A working shell variable wins over the file.
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "check", "--env-file", ".env"],
        )
        .env("ELEVENLABS_API_KEY", KEY),
    );
    assert_eq!(r["details"]["api_key_source"], "shell", "{r}");
}

#[tokio::test]
async fn a_rejected_key_on_a_remote_machine_is_replaced_in_its_own_file() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, root) = temp();
    let app = root.path().join("app");
    write(
        root.path(),
        ".env",
        "ELEVENLABS_API_KEY=sk_stale_0000000000000000\n",
    );
    write(&app, "package.json", "{}");
    let r = reply(
        cli(home.path(), &app, &server.uri(), &["onboard", "check"])
            .env("SSH_CONNECTION", "1 2 3 4"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    let env = r["details"]["env_file"].as_str().unwrap().to_string();
    assert!(env.ends_with(".env") && !env.ends_with(".env.local"), "{r}");
    assert!(say(&r).contains(&format!("in {env}")), "{r}");
    assert!(
        r["run"]
            .as_str()
            .unwrap()
            .contains(&format!("--env-file {env}")),
        "{r}"
    );
}

#[tokio::test]
async fn check_tells_a_dotenv_key_apart_from_the_env_file() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["details"]["api_key_source"], "dotenv", "{r}");
    // The code is told to load the file the key is in, not the default env file.
    assert_eq!(r["details"]["env_file"], ".env", "{r}");
    assert!(
        r["details"]["load_key"]
            .as_str()
            .unwrap()
            .contains("It's in .env:"),
        "{r}"
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--env-file", ".env"],
    ));
    assert_eq!(r["details"]["api_key_source"], "env_file", "{r}");
}

// ── test ────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_makes_one_text_to_speech_request_and_saves_the_clip() {
    let server = MockServer::start().await;
    let audio = b"ID3-fake-mp3".to_vec();
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .and(header("xi-api-key", KEY))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(audio.clone()))
        .expect(1)
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );

    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "test"],
    ));
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(r["details"]["status"], 200);
    let clip = PathBuf::from(r["details"]["audio_file"].as_str().unwrap());
    assert_eq!(std::fs::read(&clip).unwrap(), audio);
    assert!(say(&r).contains("Play"), "{r}");
    assert!(
        say(&r).contains("ElevenLabs is set up: your key is in .env.local"),
        "the last word says what is set up: {r}"
    );
    let sent = &server.received_requests().await.unwrap()[0];
    let body: Value = serde_json::from_slice(&sent.body).unwrap();
    assert_eq!(body["text"], "Welcome to ElevenLabs");
}

#[tokio::test]
async fn test_reports_missing_credits_and_other_failures_as_done() {
    let server = MockServer::start().await;
    // An exhausted quota, as ElevenLabs reports it.
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"detail": {"status": "quota_exceeded", "message": "quota"}})),
        )
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "test"],
    ));
    assert_eq!(
        (r["next"].as_str(), r["details"]["status"].as_u64()),
        (Some("done"), Some(401)),
        "{r}"
    );
    assert!(say(&r).contains("needs credits"), "{r}");

    let failing = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_json(json!({"detail": {"message": "speech service down"}})),
        )
        .mount(&failing)
        .await;
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &failing.uri(),
        &["onboard", "test"],
    ));
    assert!(say(&r).contains("500: speech service down"), "{r}");
    assert!(say(&r).contains("try again later"), "{r}");

    // A key the request refuses won't work later either.
    let refused = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "detail": {"status": "missing_permissions", "message": "missing text_to_speech"}
        })))
        .mount(&refused)
        .await;
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &refused.uri(),
        &["onboard", "test"],
    ));
    assert!(
        say(&r).contains("401: missing text_to_speech") && !say(&r).contains("try again"),
        "{r}"
    );

    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "test"],
    ));
    assert_eq!(r["next"], "done", "{r}");
    assert!(say(&r).contains("couldn't reach"), "{r}");

    std::fs::remove_file(cwd.path().join(".env.local")).unwrap();
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "test"],
    ));
    assert_eq!(
        r["run"], "elevenlabs onboard check --env-file .env.local",
        "no key: nothing is sent"
    );
}

// ── init ────────────────────────────────────────────────────────────

#[tokio::test]
async fn init_hands_on_to_sign_in() {
    let server = MockServer::start().await;
    let (home, cwd) = temp();

    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "init"])
            .env("CLAUDECODE", "1"),
    );
    assert_eq!(r["next"], "run", "{r}");
    assert!(say(&r).contains("approval page in your browser"), "{r}");
    assert_eq!(r["run"], "elevenlabs onboard connect --env-file .env.local");
    assert!(r["rules"]
        .to_string()
        .contains("Never open, print or cat the env file"));
    assert!(!cwd.path().join(".claude").exists(), "no skills are installed");

    // Flags go on to the next command.
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "init", "--no-browser"],
        )
        .env("CLAUDECODE", "1")
        .env("CODEX_THREAD_ID", "t"),
    );
    assert_eq!(
        r["run"], "elevenlabs onboard connect --env-file .env.local --no-browser",
        "init's flags go on to connect"
    );

    // Under npx, every next command runs the same latest CLI.
    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "init"])
            .env("npm_command", "exec"),
    );
    assert_eq!(
        r["run"],
        "npx -y @elevenlabs/cli@latest onboard connect --env-file .env.local"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn a_repositorys_own_git_config_cannot_run_a_command() {
    // A checkout's .git/config is read by every git command in it, and its
    // core.fsmonitor runs on `ls-files` and `check-ignore`.
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let marker = home.path().join("payload-ran");
    let payload = home.path().join("payload.sh");
    std::fs::write(
        &payload,
        format!("#!/bin/sh\ntouch {}\nexit 1\n", marker.display()),
    )
    .unwrap();
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&payload, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    git(cwd.path(), &["init", "-q"]);
    write(cwd.path(), "README.md", "app\n");
    git(cwd.path(), &["add", "README.md"]);
    git(cwd.path(), &["commit", "-qm", "app"]);
    git(
        cwd.path(),
        &["config", "core.fsmonitor", &payload.display().to_string()],
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init"],
    ));
    assert_eq!(r["next"], "run", "{r}");
    assert!(!marker.exists(), "the repository's fsmonitor ran a command");
}

#[tokio::test]
async fn a_projects_env_cannot_send_the_key_through_a_proxy() {
    // Proxy and TLS settings in a repository's `.env` are refused, so the key goes straight to the API.
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env",
        &format!(
            "ELEVENLABS_API_KEY={KEY}\nHTTP_PROXY=http://127.0.0.1:9\nHTTPS_PROXY=http://127.0.0.1:9\n\
             ALL_PROXY=http://127.0.0.1:9\nhttp_proxy=http://127.0.0.1:9\nhttps_proxy=http://127.0.0.1:9\n\
             all_proxy=http://127.0.0.1:9\n"
        ),
    );
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "check", "--env-file", ".env"],
        )
        .env_remove("HTTP_PROXY")
        .env_remove("HTTPS_PROXY")
        .env_remove("ALL_PROXY")
        .env_remove("http_proxy")
        .env_remove("https_proxy")
        .env_remove("all_proxy"),
    );
    assert_eq!(
        r["next"], "write_code",
        "the check reached the API directly: {r}"
    );
}

#[tokio::test]
async fn init_goes_straight_to_check_when_a_key_already_works() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init"],
    ));
    assert_eq!(r["next"], "run", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --env-file .env.local");
}

#[test]
fn init_on_a_remote_machine_prepares_the_manual_key() {
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    let r = reply(
        cli(home.path(), cwd.path(), OFFLINE, &["onboard", "init"])
            .env("SSH_CONNECTION", "10.0.0.1 22 10.0.0.2 22"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --env-file .env.local");
    assert!(
        say(&r).contains("elevenlabs.io/app/settings/api-keys"),
        "{r}"
    );
    assert!(std::fs::read_to_string(cwd.path().join(".env.local"))
        .unwrap()
        .ends_with("ELEVENLABS_API_KEY=\n"));
    assert!(std::fs::read_to_string(cwd.path().join(".gitignore"))
        .unwrap()
        .contains("/.env.local"));
}

// ── connect: everything before the browser ──────────────────────────

fn connect(home: &Path, cwd: &Path, args: &[&str]) -> Value {
    reply(&mut cli(
        home,
        cwd,
        OFFLINE,
        &[&["onboard", "connect"], args].concat(),
    ))
}

#[test]
fn connect_refuses_a_tracked_file_and_a_symlink() {
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    write(cwd.path(), ".env", "TRACKED=1\n");
    git(cwd.path(), &["add", ".env"]);
    let r = connect(
        home.path(),
        cwd.path(),
        &["--env-file", ".env", "--replace"],
    );
    assert_eq!(r["next"], "fix", "{r}");
    assert!(say(&r).contains("git rm --cached .env"), "{r}");
    assert_eq!(
        r["run"], "elevenlabs onboard connect --replace --env-file .env",
        "the fix makes the same step work"
    );
    assert_eq!(
        std::fs::read_to_string(cwd.path().join(".env")).unwrap(),
        "TRACKED=1\n"
    );

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(cwd.path().join("elsewhere"), cwd.path().join(".env.local"))
            .unwrap();
        let r = connect(home.path(), cwd.path(), &[]);
        assert!(say(&r).contains("is a symlink"), "{r}");
    }
}

#[test]
fn connect_keeps_an_existing_key_unless_told_to_replace_it() {
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let r = connect(home.path(), cwd.path(), &[]);
    assert_eq!(
        (r["next"].as_str(), r["run"].as_str()),
        (
            Some("run"),
            Some("elevenlabs onboard check --env-file .env.local")
        ),
        "{r}"
    );
    assert!(say(&r).contains("ending in 4567"), "{r}");

    // With --replace the key is no longer the reason to stop: here, the remote shell is,
    // even with --no-browser (the page would copy the key on another machine).
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            OFFLINE,
            &["onboard", "connect", "--replace", "--no-browser"],
        )
        .env("SSH_CONNECTION", "1 2 3 4"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(
        say(&r).contains("A browser can't reach this machine"),
        "{r}"
    );
}

#[test]
fn connect_replaces_a_dotenv_key_in_that_file() {
    let (home, cwd) = temp();
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let r = connect(home.path(), cwd.path(), &["--replace", "--no-browser"]);
    assert_eq!(r["next"], "run", "{r}");
    let run = r["run"].as_str().unwrap();
    assert!(
        run.starts_with("elevenlabs onboard connect --replace --env-file ")
            && run.ends_with(".env --no-browser"),
        "{run}"
    );
}

#[test]
fn connect_wait_without_a_sign_in_starts_one() {
    let (home, cwd) = temp();
    let r = connect(home.path(), cwd.path(), &["--wait"]);
    assert_eq!(
        (r["next"].as_str(), r["run"].as_str()),
        (
            Some("run"),
            Some("elevenlabs onboard connect --env-file .env.local")
        ),
        "{r}"
    );
}

#[test]
fn onboard_lists_its_steps_and_stays_out_of_the_top_level_help() {
    let (home, cwd) = temp();
    let out = cli(home.path(), cwd.path(), OFFLINE, &["onboard"])
        .output()
        .unwrap();
    let help = all_output(&out);
    for step in ["init", "connect", "check", "test"] {
        assert!(help.contains(step), "help lacks {step}:\n{help}");
    }
    let out = cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "connect", "--help"],
    )
    .output()
    .unwrap();
    let help = String::from_utf8_lossy(&out.stdout);
    for flag in ["--env-file", "--no-browser", "--replace", "--wait"] {
        assert!(help.contains(flag), "connect help lacks {flag}:\n{help}");
    }
    for hidden in ["--authorize-url", "--worker", "--launch-worker"] {
        assert!(!help.contains(hidden), "{hidden} stays hidden:\n{help}");
    }
    let top = cli(home.path(), cwd.path(), OFFLINE, &["--help"])
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&top.stdout).contains("onboard"));
}

// ── connect: the background sign-in ─────────────────────────────────

/// The session folder the worker writes to, and the approval link it logged.
fn approval(home: &Path) -> (PathBuf, String) {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        for dir in std::fs::read_dir(home.join(".elevenlabs/onboard"))
            .into_iter()
            .flatten()
            .flatten()
        {
            let log = std::fs::read_to_string(dir.path().join("worker.log")).unwrap_or_default();
            if let Some(url) = log.lines().find_map(|l| l.trim().strip_prefix("URL: ")) {
                return (dir.path(), url.to_string());
            }
        }
        assert!(
            Instant::now() < deadline,
            "the worker never printed the approval link"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn query(url: &str, name: &str) -> String {
    reqwest::Url::parse(url)
        .unwrap()
        .query_pairs()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.into_owned())
        .unwrap_or_else(|| panic!("no {name} in {url}"))
}

/// The browser's side of a declined approval: the page sends it back to the
/// callback. `localhost` is dialed as a browser would, IPv4 or IPv6.
fn decline(url: &str) {
    callback(url, "error=access_denied&error_description=declined");
}

/// The browser's side of an approval: the page sends the code to the callback.
fn approve(url: &str) {
    callback(url, "code=approved");
}

fn callback(url: &str, params: &str) {
    let redirect = reqwest::Url::parse(&query(url, "redirect_uri")).unwrap();
    let authority = format!(
        "{}:{}",
        redirect.host_str().unwrap(),
        redirect.port().unwrap()
    );
    let state = query(url, "state");
    let mut stream = std::net::TcpStream::connect(&authority).unwrap();
    write!(
        stream,
        "GET {}?{params}&state={state} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n\r\n",
        redirect.path()
    )
    .unwrap();
    let _ = std::io::Read::read_to_end(&mut stream, &mut Vec::new());
}

fn reply_of(child: std::process::Child) -> Value {
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "{}", all_output(&out));
    serde_json::from_slice(&out.stdout).unwrap()
}

/// The approval link in a `wait` reply.
fn link(reply: &Value) -> String {
    let say = say(reply);
    let at = say.find("open this link: ").expect("a link") + "open this link: ".len();
    say[at..].trim().to_string()
}

/// The tests that sign in take turns: each listens on the callback ports.
async fn signing_in() -> tokio::sync::MutexGuard<'static, ()> {
    static PORTS: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    PORTS.lock().await
}

/// Something already listens on the sign-in's callback ports (another sign-in
/// on this machine), which this test would disturb.
fn callback_ports_busy() -> bool {
    [8484, 8483, 8482]
        .iter()
        .any(|p| std::net::TcpStream::connect(("localhost", *p)).is_ok())
}

#[tokio::test]
async fn connect_signs_in_in_the_background_and_every_waiter_gets_the_result() {
    let _turn = signing_in().await;
    if callback_ports_busy() {
        eprintln!("skipped: another sign-in is listening on the callback ports");
        return;
    }
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    let start = |args: &[&str]| {
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &[
                &[
                    "onboard",
                    "connect",
                    "--no-browser",
                    "--authorize-url",
                    &authorize,
                ],
                args,
            ]
            .concat(),
        )
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
    };
    // Without a browser, connect hands the link over as soon as the worker has it.
    let first = reply_of(start(&[]));
    assert_eq!(first["next"], "wait", "{first}");
    let (session, url) = approval(home.path());
    assert!(say(&first).contains(&url), "{first}");
    let worker = std::fs::read_to_string(session.join("worker.pid")).unwrap();
    assert!(url.starts_with(&authorize), "{url}");
    assert_eq!(query(&url, "key_handoff"), "clipboard");
    assert!(
        !url.contains("product="),
        "one key for every product: {url}"
    );

    // Another connect for the same request waits on that sign-in.
    let again = reply_of(start(&[]));
    assert!(say(&again).contains(&url), "{again}");
    assert_eq!(
        std::fs::read_to_string(session.join("worker.pid")).unwrap(),
        worker,
        "still the first worker"
    );

    // One for another env file replaces it, with a new approval link.
    let other = reply_of(start(&["--env-file", "other.env"]));
    assert_eq!(other["next"], "wait", "{other}");
    let url = link(&other);
    assert_ne!(
        std::fs::read_to_string(session.join("worker.pid")).unwrap(),
        worker,
        "a new worker for the new request"
    );

    // --wait keeps waiting, and gets the result when it lands.
    let waiter = start(&["--env-file", "other.env", "--wait"]);
    std::thread::sleep(Duration::from_millis(500));
    decline(&url);
    let r = reply_of(waiter);
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(say(&r).contains("declined"), "{r}");
    assert_eq!(r["details"]["key_status"], "sign_in_failed");
    assert!(
        r["run"].as_str().unwrap().contains("--env-file other.env"),
        "{r}"
    );
    // The result stays readable until a new sign-in starts.
    let r = connect(
        home.path(),
        cwd.path(),
        &["--env-file", "other.env", "--wait"],
    );
    assert!(say(&r).contains("declined"), "{r}");
    assert!(
        !cwd.path().join(".env.local").exists(),
        "nothing was stored"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn a_projects_env_cannot_reach_the_git_the_sign_in_runs() {
    let _turn = signing_in().await;
    if callback_ports_busy() {
        eprintln!("skipped: another sign-in is listening on the callback ports");
        return;
    }
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "refresh"
        })))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    // The background sign-in is this program started again, after this one loaded the `.env`.
    let trace = home.path().join("git-trace");
    write(
        cwd.path(),
        ".env",
        &format!("GIT_TRACE={}\n", trace.display()),
    );
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    let first = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &[
            "onboard",
            "connect",
            "--no-browser",
            "--authorize-url",
            &authorize,
        ],
    ));
    assert_eq!(first["next"], "wait", "{first}");
    let (_, url) = approval(home.path());
    approve(&url);
    let deadline = Instant::now() + Duration::from_secs(20);
    let done = loop {
        let r = connect(home.path(), cwd.path(), &["--wait", "--no-browser"]);
        if r["next"] != "wait" || Instant::now() >= deadline {
            break r;
        }
    };
    // No key reached the clipboard, so the env file was prepared for one, and gitignored.
    assert_eq!(done["details"]["key_status"], "no_key", "{done}");
    assert!(std::fs::read_to_string(cwd.path().join(".gitignore"))
        .unwrap()
        .contains(".env.local"));
    assert!(
        !trace.exists(),
        "the .env's GIT_TRACE reached the worker's git"
    );
}
