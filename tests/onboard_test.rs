//! Integration tests for `elevenlabs onboard` (`init`, `check`, `test`)
//! against a local mock API. The clipboard hand-off needs a real approval page
//! and clipboard, so it is covered by the onboarding harness (and the
//! clipboard's outcomes by the unit tests); these run the real binary for
//! every reply that doesn't, and the sign-in up to the browser's callback.
//! They never touch the system clipboard, a browser, or GitHub.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Duration;

use serde_json::{json, Value};
use wiremock::matchers::{header, method, path, path_regex};
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
        "CLAUDE_CODE_ENTRYPOINT",
        "CODEX_THREAD_ID",
        "CODEX_SANDBOX",
        "CODEX_SANDBOX_NETWORK_DISABLED",
        "CURSOR_AGENT",
        "CURSOR_TRACE_ID",
        "GEMINI_CLI",
        "TERM_PROGRAM",
        "ELEVENLABS_TELEMETRY",
        "DO_NOT_TRACK",
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
    assert_eq!(
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .lines()
            .count(),
        1,
        "the reply is one line"
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
    assert_eq!(r["run"], "elevenlabs onboard connect --env-path .env.local");
    assert_eq!(server.received_requests().await.unwrap().len(), 0);
    // `status` is the same command.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "status"],
    ));
    assert_eq!(r["run"], "elevenlabs onboard connect --env-path .env.local");
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
async fn check_verifies_the_key_secures_the_file_and_hands_over_the_rest() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    // The CLI never reads the code, so it never looks an agent up.
    Mock::given(method("GET"))
        .and(path_regex("^/v1/convai/agents/"))
        .respond_with(ResponseTemplate::new(404))
        .expect(0)
        .mount(&server)
        .await;
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
    assert_eq!(r["details"]["api_key_source"], "env_file");
    assert_eq!(r["details"]["key_last4"], "4567");
    let fixed = r["details"]["fixed"].to_string();
    assert!(fixed.contains(".gitignore"), "{fixed}");
    // One reply for everything after the key: no stages, and no product chosen for the developer.
    assert_eq!(r["next"], "integrate", "{r}");
    assert_eq!(
        say(&r),
        "Your ElevenLabs key ending in 4567 works. It's in .env.local."
    );
    assert_eq!(
        r["details"]["question"],
        "What would you like to build with ElevenLabs, or should I just add the SDK for now?"
    );
    // `run` is the check that ends on the test offer, once the code is written.
    assert_eq!(
        r["run"],
        "elevenlabs onboard check --written --env-path .env.local"
    );
    let offer = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--written"],
    ));
    assert_eq!(offer["next"], "offer_test", "{offer}");
    assert_eq!(
        say(&offer),
        "Your ElevenLabs key ending in 4567 works. It's in .env.local. Want me to send a test request? It makes a short text-to-speech clip with your key and uses a few credits."
    );
    assert_eq!(offer["run"], "elevenlabs onboard test --env-path .env.local");
    assert!(r["rules"].as_array().is_some_and(|r| !r.is_empty()), "{r}");
    // Loading the key is a goal for the agent, not a recipe for a framework.
    let load = r["details"]["load_key"].as_str().unwrap();
    assert!(
        load.contains("It is in .env.local") && !load.contains("--env-file"),
        "{load}"
    );
    // The skills go in a new folder in the temp folder (here HOME), never the project.
    let skills = PathBuf::from(r["details"]["skills_dir"].as_str().unwrap());
    assert!(
        skills.is_dir() && skills.starts_with(home.path()) && !skills.starts_with(cwd.path()),
        "{r}"
    );
    assert_eq!(std::fs::read_dir(&skills).unwrap().count(), 0, "a new folder");
    let fetch = r["details"]["fetch_skills"].as_str().unwrap();
    assert!(
        fetch.starts_with("curl -fsSL -o ")
            && fetch.contains("https://github.com/elevenlabs/skills/archive/refs/heads/main.tar.gz")
            && fetch.contains(&format!("-C {}", skills.display())),
        "{fetch}"
    );
    // The check secured the file.
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

    // The reply is the same once the code uses the SDK: what is left is the agent's to know.
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
    // A flag an older reply asked for still works.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--answered"],
    ));
    assert_eq!(r["next"], "integrate", "{r}");
    assert_eq!(r["details"]["fixed"], json!([]), "nothing left to fix");
    server.verify().await;
}

#[tokio::test]
async fn the_reply_is_json_even_when_prose_is_asked_for() {
    // Only agents run these steps; prose would drop the instruction, rules and details.
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    for flag in [&["--human"][..], &["--format", "table"][..]] {
        let args: Vec<&str> = ["onboard", "check"].iter().chain(flag).copied().collect();
        let r = reply(&mut cli(home.path(), cwd.path(), &server.uri(), &args));
        assert!(r["instruction"].as_str().is_some() && r["details"].is_object(), "{flag:?}: {r}");
    }
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
    assert_eq!(r["next"], "integrate", "{r}");
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
        "elevenlabs onboard connect --replace --env-path .env"
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
    let (home, app) = temp();
    // The project keeps its key in `.env`, so that is the file to change.
    write(
        app.path(),
        ".env",
        "ELEVENLABS_API_KEY=sk_stale_0000000000000000\n",
    );
    write(app.path(), "package.json", "{}");
    let r = reply(
        cli(home.path(), app.path(), &server.uri(), &["onboard", "check"])
            .env("SSH_CONNECTION", "1 2 3 4"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    assert_eq!(r["details"]["env_file"], ".env", "{r}");
    assert!(say(&r).contains("in .env,"), "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --env-path .env", "{r}");
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
        r["run"], "elevenlabs onboard check --env-path .env.local",
        "no key: nothing is sent"
    );
}

// ── init ────────────────────────────────────────────────────────────

/// A sign-in the test drives: the command, run with its output piped, and the
/// approval link the sign-in printed, as `auth login` prints it.
struct SignIn {
    child: std::process::Child,
    url: String,
    /// What the command said on stderr up to the link.
    said: String,
}

impl SignIn {
    fn start(cmd: &mut Command) -> Self {
        use std::io::BufRead;
        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut stderr = std::io::BufReader::new(child.stderr.take().unwrap());
        let mut said = String::new();
        let url = loop {
            let mut line = String::new();
            let read = stderr.read_line(&mut line).unwrap();
            assert!(read > 0, "the sign-in ended before printing its link: {said}");
            said.push_str(&line);
            if let Some(url) = line.trim().strip_prefix("URL: ") {
                break url.to_string();
            }
        };
        // Keep reading, so the command never blocks on a full pipe.
        std::thread::spawn(move || {
            let mut rest = String::new();
            let _ = std::io::Read::read_to_string(&mut stderr, &mut rest);
        });
        Self { child, url, said }
    }

    /// The command's reply, once the browser's side is done.
    fn reply(self) -> Value {
        let out = self.child.wait_with_output().unwrap();
        assert!(out.status.success(), "{}", all_output(&out));
        assert_eq!(
            String::from_utf8_lossy(&out.stdout).trim_end().lines().count(),
            1,
            "the reply is one line"
        );
        serde_json::from_slice(&out.stdout).expect("stdout is JSON")
    }
}

#[tokio::test]
async fn init_signs_in_as_auth_login_does_and_a_declined_approval_ends_it() {
    let _turn = signing_in().await;
    if callback_ports_busy() {
        eprintln!("skipped: another sign-in is listening on the callback ports");
        return;
    }
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    // One command: with no key in the project, `init` is the sign-in.
    let sign_in = SignIn::start(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "init", "--no-browser", "--authorize-url", &authorize],
        )
        .env("CLAUDECODE", "1"),
    );
    // For an agent whose command is stopped before the developer approves.
    assert!(
        sign_in.said.contains("If this command is stopped before you approve, run it again and give it that long."),
        "{}",
        sign_in.said
    );
    let url = sign_in.url.clone();
    assert!(url.starts_with(&authorize), "{url}");
    // All the sign-in adds to `auth login`'s: the page is asked for a key, and told where the CLI runs.
    assert_eq!(query(&url, "key_handoff"), "clipboard");
    let this_os = match std::env::consts::OS {
        os @ ("windows" | "macos" | "linux") => os,
        _ => "other",
    };
    assert_eq!(query(&url, "cli_os"), this_os, "{url}");
    assert_eq!(query(&url, "cli_agent"), "claude-code", "{url}");
    assert!(
        !url.contains("product=") && !url.contains("cli_prev"),
        "one key for every product, and nothing kept between sign-ins: {url}"
    );

    decline(&url);
    let r = sign_in.reply();
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(say(&r).contains("declined"), "{r}");
    assert_eq!(r["details"]["key_status"], "sign_in_failed");
    // Flags go on to the next command.
    assert_eq!(
        r["run"],
        "elevenlabs onboard connect --env-path .env.local --no-browser"
    );
    assert!(r["rules"]
        .to_string()
        .contains("Never open, print or cat the env file"));
    assert!(
        !cwd.path().join(".env.local").exists() && !home.path().join(".elevenlabs/onboard").exists(),
        "nothing was stored, and no sign-in is kept running"
    );
}

#[tokio::test]
async fn an_approved_sign_in_without_a_key_on_the_clipboard_leaves_the_key_to_the_developer() {
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
    // The sign-in runs after this program loaded the project's `.env`, which must not reach git.
    let trace = home.path().join("git-trace");
    write(
        cwd.path(),
        ".env",
        &format!("GIT_TRACE={}\n", trace.display()),
    );
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    // `connect` is the same step under its older name; under npx, the next command is too.
    let sign_in = SignIn::start(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "connect", "--no-browser", "--authorize-url", &authorize],
        )
        .env("npm_command", "exec"),
    );
    let page = approve(&sign_in.url);
    assert!(
        page.contains("Location: https://elevenlabs.io/app/oauth/cli/success\r\n"),
        "{page}"
    );
    let r = sign_in.reply();
    // Signed in: the tests keep the clipboard out, so no key reached the command,
    // and the env file was prepared for one, and gitignored.
    assert_eq!(r["details"]["key_status"], "no_key", "{r}");
    assert_eq!(r["next"], "ask_user", "{r}");
    assert_eq!(
        r["run"],
        "npx -y @elevenlabs/cli@latest onboard check --env-path .env.local --no-browser"
    );
    assert!(std::fs::read_to_string(cwd.path().join(".gitignore"))
        .unwrap()
        .contains(".env.local"));
    assert!(!trace.exists(), "the .env's GIT_TRACE reached the sign-in's git");
}

/// What the approval link and the requests say about the machine and the
/// agent follows the telemetry switch: with it off, nothing.
#[tokio::test]
async fn with_telemetry_off_nothing_is_said_about_the_machine() {
    let _turn = signing_in().await;
    if callback_ports_busy() {
        eprintln!("skipped: another sign-in is listening on the callback ports");
        return;
    }
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    // A sign-in run by Claude Code, declined once its link is out. Returns that link.
    let declined_sign_in = |telemetry: &[(&str, &str)], args: &[&str]| {
        let mut cmd = cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &[
                &["onboard", "init", "--no-browser", "--authorize-url", &authorize],
                args,
            ]
            .concat(),
        );
        cmd.env("CLAUDECODE", "1").envs(telemetry.iter().copied());
        let sign_in = SignIn::start(&mut cmd);
        let url = sign_in.url.clone();
        decline(&url);
        sign_in.reply();
        url
    };
    let said = |url: &str| -> Vec<String> {
        reqwest::Url::parse(url)
            .unwrap()
            .query_pairs()
            .map(|(k, _)| k.into_owned())
            .filter(|k| k.starts_with("cli_"))
            .collect()
    };

    let first = declined_sign_in(&[], &[]);
    assert_eq!(query(&first, "cli_agent"), "claude-code", "{first}");
    for (telemetry, args) in [
        (&[][..], &["--telemetry", "off"][..]),
        (&[("DO_NOT_TRACK", "1")][..], &[][..]),
        (&[("ELEVENLABS_TELEMETRY", "off")][..], &[][..]),
    ] {
        let url = declined_sign_in(telemetry, args);
        assert_eq!(said(&url), Vec::<String>::new(), "{url}");
    }

    // The requests say which agent ran them too, unless telemetry is off.
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    for args in [&[][..], &["--telemetry", "off"][..]] {
        reply(
            cli(
                home.path(),
                cwd.path(),
                &server.uri(),
                &[&["onboard", "check"][..], args].concat(),
            )
            .env("CLAUDECODE", "1"),
        );
    }
    let agents: Vec<String> = server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|r| r.url.path() == "/v1/user")
        .map(|r| r.headers["user-agent"].to_str().unwrap().to_string())
        .collect();
    let (on, off) = agents.split_at(agents.len() / 2);
    assert!(!on.is_empty(), "{agents:?}");
    for agent in on {
        assert!(agent.contains(" agent/claude-code"), "{agent}");
        assert!(agent.contains("cmd/onboard.check"), "{agent}");
    }
    for agent in off {
        assert!(!agent.contains("agent/"), "{agent}");
        assert!(!agent.contains("os/"), "{agent}");
    }
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
    // On a machine no browser reaches, so `init` ends on the manual key and not a sign-in.
    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "init"])
            .env("SSH_CONNECTION", "1 2 3 4"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
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
        r["next"], "integrate",
        "the check reached the API directly: {r}"
    );
}

#[tokio::test]
async fn init_checks_a_key_that_is_already_there() {
    // A working key needs no sign-in: the reply is the one that hands the rest over.
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
    assert_eq!(r["next"], "integrate", "{r}");
    assert_eq!(
        say(&r),
        "Your ElevenLabs key ending in 4567 works. It's in .env.local."
    );
    assert_eq!(r["run"], "elevenlabs onboard check --written --env-path .env.local");
    assert_eq!(r["details"]["env_file"], ".env.local");
    assert!(r["rules"].as_array().is_some_and(|r| !r.is_empty()), "{r}");

    // A key the project keeps in `.env` is found there, with no file named.
    let (home, cwd) = temp();
    write(cwd.path(), "package.json", "{}");
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init"],
    ));
    assert_eq!(r["next"], "integrate", "{r}");
    assert_eq!(r["details"]["env_file"], ".env", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --written --env-path .env");
    // The request counts as `init`: no sign-in ran.
    let sent = server.received_requests().await.unwrap();
    assert!(
        sent.iter().all(|r| r.headers["user-agent"]
            .to_str()
            .unwrap()
            .contains("cmd/onboard.init")),
        "{sent:?}"
    );
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
    assert_eq!(r["run"], "elevenlabs onboard check --env-path .env.local");
    assert!(
        say(&r).contains("https://elevenlabs.io/app/api/api-keys?quickstart=true."),
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
        r["run"], "elevenlabs onboard connect --replace --env-path .env",
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

#[tokio::test]
async fn connect_keeps_an_existing_key_unless_told_to_replace_it() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    // A working key is checked and kept: no browser opens.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "connect"],
    ));
    assert_eq!(r["next"], "integrate", "{r}");
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
fn onboard_lists_its_steps_and_stays_out_of_the_top_level_help() {
    let (home, cwd) = temp();
    let out = cli(home.path(), cwd.path(), OFFLINE, &["onboard"])
        .output()
        .unwrap();
    let help = all_output(&out);
    for step in ["init", "check", "test"] {
        assert!(help.contains(step), "help lacks {step}:\n{help}");
    }
    assert!(!help.contains("connect"), "the older name stays hidden:\n{help}");
    let out = cli(
        home.path(),
        cwd.path(),
        OFFLINE,
        &["onboard", "init", "--help"],
    )
    .output()
    .unwrap();
    let help = String::from_utf8_lossy(&out.stdout);
    for flag in ["--env-path", "--no-browser", "--replace"] {
        assert!(help.contains(flag), "init help lacks {flag}:\n{help}");
    }
    // The old name still parses, but isn't offered: through npx, Node takes it.
    assert!(!help.contains("--env-file"), "{help}");
    for hidden in ["--authorize-url", "--wait", "--worker"] {
        assert!(!help.contains(hidden), "{hidden} stays hidden:\n{help}");
    }
    let top = cli(home.path(), cwd.path(), OFFLINE, &["--help"])
        .output()
        .unwrap();
    assert!(!String::from_utf8_lossy(&top.stdout).contains("onboard"));
}

// ── the sign-in ─────────────────────────────────────────────────────

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
/// Returns what the sign-in answered the browser.
fn approve(url: &str) -> String {
    callback(url, "code=approved")
}

fn callback(url: &str, params: &str) -> String {
    let redirect = reqwest::Url::parse(&query(url, "redirect_uri")).unwrap();
    let authority = format!(
        "{}:{}",
        redirect.host_str().unwrap(),
        redirect.port().unwrap()
    );
    let state = query(url, "state");
    let request = format!(
        "GET {}?{params}&state={state} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\n\r\n",
        redirect.path()
    );
    // The worker can be a moment from listening on a slow machine (CI): it may refuse the
    // connection or drop it unread. Try again a few times, as a browser's retry would.
    let mut last = None;
    for _ in 0..10 {
        let sent = std::net::TcpStream::connect(&authority).and_then(|mut stream| {
            stream.write_all(request.as_bytes())?;
            let mut answer = Vec::new();
            std::io::Read::read_to_end(&mut stream, &mut answer)?;
            Ok(answer)
        });
        match sent {
            Ok(answer) => return String::from_utf8_lossy(&answer).into_owned(),
            Err(e) => last = Some(e),
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    panic!("the callback never reached the sign-in: {last:?}");
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
