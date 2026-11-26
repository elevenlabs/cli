//! Integration tests for `elevenlabs migrate openai` and the skills copy it
//! shares with `onboard`: the real binary against a local mock that serves
//! the skills download and stands in for the API. In a file of their own, so
//! changes to `onboard`'s own tests don't collide with these. They never
//! touch the system clipboard, a browser, or GitHub.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{json, Value};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";

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

const MIGRATION_GUIDE: &str = "# Migrate from OpenAI\n";

/// A download of the skills repo holding `files`, in one top folder as GitHub wraps it.
fn skills_archive(files: &[(&str, &str)]) -> Vec<u8> {
    let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(
        Vec::new(),
        Default::default(),
    ));
    for (file, text) in files {
        let mut header = tar::Header::new_gnu();
        header.set_size(text.len() as u64);
        header.set_mode(0o644);
        tar.append_data(&mut header, format!("skills-main/{file}"), text.as_bytes())
            .unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

/// `server` hands out the skills where the CLI is told to look for them
/// (`skills_from`); with `failures`, only after that many failed downloads.
async fn serve_skills(server: &MockServer, files: &[(&str, &str)], failures: u64) {
    if failures > 0 {
        Mock::given(method("GET"))
            .and(path("/skills.tar.gz"))
            .respond_with(ResponseTemplate::new(503))
            .up_to_n_times(failures)
            .with_priority(1)
            .mount(server)
            .await;
    }
    Mock::given(method("GET"))
        .and(path("/skills.tar.gz"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(skills_archive(files)))
        .mount(server)
        .await;
}

fn skills_from<'a>(cmd: &'a mut Command, server: &MockServer) -> &'a mut Command {
    cmd.env(
        "ELEVENLABS_SKILLS_ARCHIVE",
        format!("{}/skills.tar.gz", server.uri()),
    )
}

/// The requests for the skills, as (path, User-Agent).
async fn skills_requests(server: &MockServer) -> Vec<String> {
    let sent = server.received_requests().await.unwrap();
    sent.iter()
        .filter(|r| r.url.path() == "/skills.tar.gz")
        .map(|r| r.headers["user-agent"].to_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn the_hand_overs_command_puts_the_skills_in_its_folder() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    // The first download fails, as a dropped connection would.
    serve_skills(
        &server,
        &[
            ("README.md", "readme"),
            ("text-to-speech/SKILL.md", "# Text to speech\n"),
            ("text-to-speech/references/streaming.md", "streaming"),
            ("speech-to-text/SKILL.md", "# Speech to text\n"),
        ],
        1,
    )
    .await;
    let (home, cwd) = temp();
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let handed = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init"],
    ));
    let dir = handed["details"]["skills_dir"].as_str().unwrap();
    let fetch = handed["details"]["fetch_skills"].as_str().unwrap();
    assert_eq!(fetch, format!("elevenlabs onboard skills {dir}"));
    assert!(
        skills_requests(&server).await.is_empty(),
        "nothing is downloaded until the agent asks"
    );

    let out = skills_from(
        &mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "skills", dir],
        ),
        &server,
    )
    .env("CLAUDECODE", "1")
    .output()
    .unwrap();
    assert!(out.status.success(), "{}", all_output(&out));
    // One skill a line: what the agent picks from.
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "speech-to-text\ntext-to-speech\n"
    );
    let dir = Path::new(dir);
    assert_eq!(
        std::fs::read_to_string(dir.join("text-to-speech/SKILL.md")).unwrap(),
        "# Text to speech\n"
    );
    assert!(dir.join("text-to-speech/references/streaming.md").is_file());
    assert!(dir.join("README.md").is_file());
    // Nothing of it is in the project.
    let project: Vec<_> = std::fs::read_dir(cwd.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    assert_eq!(project.len(), 2, "{project:?}");
    // It took the second try, and GitHub is told nothing about the run.
    assert_eq!(
        skills_requests(&server).await,
        ["elevenlabs-cli", "elevenlabs-cli"]
    );
}

#[tokio::test]
async fn a_skills_download_that_keeps_failing_says_to_go_on_without_one() {
    let server = MockServer::start().await;
    serve_skills(&server, &[("music/SKILL.md", "music")], 5).await;
    let (home, cwd) = temp();
    let dir = home.path().join("elevenlabs-skills-test");
    std::fs::create_dir(&dir).unwrap();
    let out = skills_from(
        &mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "skills", dir.to_str().unwrap()],
        ),
        &server,
    )
    .output()
    .unwrap();
    assert!(!out.status.success());
    let said = all_output(&out);
    assert!(
        said.contains("GitHub answered 503. Go on without a skill."),
        "{said}"
    );
    assert_eq!(
        std::fs::read_dir(&dir).unwrap().count(),
        0,
        "nothing half-written"
    );
    assert_eq!(
        skills_requests(&server).await.len(),
        2,
        "one more try, then it stops"
    );
}

#[tokio::test]
async fn migrate_openai_hands_over_the_guide_and_touches_nothing_else() {
    let server = MockServer::start().await;
    serve_skills(
        &server,
        &[
            ("migrate-from-openai/SKILL.md", MIGRATION_GUIDE),
            ("migrate-from-openai/references/python.md", "python"),
            ("text-to-speech/SKILL.md", "tts"),
        ],
        0,
    )
    .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        "package.json",
        r#"{"dependencies":{"openai":"^6"}}"#,
    );
    let r = reply(skills_from(
        &mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["migrate", "openai"],
        ),
        &server,
    ));
    assert_eq!(r["next"], "follow_guide", "{r}");
    let told = r["instruction"].as_str().unwrap();
    assert!(
        told.contains("read `details.guide` and follow it from its first step to its last")
            && told.contains("Don't run `run` now"),
        "{told}"
    );
    // The guide is in a copy of the skills in the temp folder (here HOME), never the project.
    let guide = PathBuf::from(r["details"]["guide"].as_str().unwrap());
    let dir = PathBuf::from(r["details"]["skills_dir"].as_str().unwrap());
    assert_eq!(guide, dir.join("migrate-from-openai").join("SKILL.md"));
    assert!(
        dir.starts_with(home.path()) && !dir.starts_with(cwd.path()),
        "{r}"
    );
    assert_eq!(std::fs::read_to_string(&guide).unwrap(), MIGRATION_GUIDE);
    assert!(dir
        .join("migrate-from-openai/references/python.md")
        .is_file());
    assert!(dir.join("text-to-speech/SKILL.md").is_file());
    // The hand-off the guide ends on is onboarding's own key step, marked as a migration's.
    assert_eq!(r["run"], "elevenlabs onboard init --from openai");
    // Onboarding's rules, as they are.
    let rules = r["rules"].as_array().unwrap();
    assert!(
        rules.len() == 6
            && rules[1]
                .as_str()
                .unwrap()
                .starts_with("Never ask for an API key"),
        "{r}"
    );
    // No sign-in, no key, nothing sent to ElevenLabs, and the project is as it was.
    let sent = server.received_requests().await.unwrap();
    assert!(
        sent.iter().all(|r| r.url.path() == "/skills.tar.gz"),
        "{sent:?}"
    );
    assert_eq!(std::fs::read_dir(cwd.path()).unwrap().count(), 1);
    assert!(!cwd.path().join(".env").exists());

    // Through npx, the next commands stay on npx.
    let r = reply(skills_from(
        cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["migrate", "openai"],
        )
        .env("npm_command", "exec"),
        &server,
    ));
    assert_eq!(
        r["run"],
        "npx -y @elevenlabs/cli@latest onboard init --from openai"
    );
}

#[tokio::test]
async fn a_projects_env_cannot_choose_the_guide() {
    let project_pick = MockServer::start().await;
    serve_skills(
        &project_pick,
        &[("migrate-from-openai/SKILL.md", "# Something else\n")],
        0,
    )
    .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env",
        &format!(
            "ELEVENLABS_SKILLS_ARCHIVE={}/skills.tar.gz\n",
            project_pick.uri()
        ),
    );
    // GitHub sits behind a proxy where nothing listens, so the test never reaches it.
    let dead = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let proxy = format!("http://{}", dead.local_addr().unwrap());
    drop(dead);
    let r = reply(
        cli(
            home.path(),
            cwd.path(),
            &project_pick.uri(),
            &["migrate", "openai"],
        )
        .env("ELEVENLABS_PROXY", proxy)
        .env("ELEVENLABS_NO_PROXY", "127.0.0.1"),
    );
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(skills_requests(&project_pick).await.is_empty());
}

#[tokio::test]
async fn migrate_openai_without_its_guide_changes_nothing_and_says_so() {
    let (home, cwd) = temp();
    let left_behind = || {
        std::fs::read_dir(home.path())
            .unwrap()
            .filter(|e| {
                e.as_ref()
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("elevenlabs-skills-")
            })
            .count()
    };
    // GitHub can't be reached.
    let server = MockServer::start().await;
    serve_skills(
        &server,
        &[("migrate-from-openai/SKILL.md", MIGRATION_GUIDE)],
        5,
    )
    .await;
    let r = reply(skills_from(
        &mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["migrate", "openai"],
        ),
        &server,
    ));
    assert_eq!(r["next"], "ask_user", "{r}");
    assert_eq!(
        say(&r),
        "I couldn't get the ElevenLabs migration guide (GitHub answered 503), so nothing in your project has \
         changed. Tell me when you'd like me to try again."
    );
    assert_eq!(r["run"], "elevenlabs migrate openai");
    assert_eq!(
        skills_requests(&server).await.len(),
        2,
        "one more try, then it stops"
    );
    assert_eq!(left_behind(), 0);

    // The skills arrive, but the guide isn't among them.
    let server = MockServer::start().await;
    serve_skills(&server, &[("text-to-speech/SKILL.md", "tts")], 0).await;
    let r = reply(skills_from(
        &mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["migrate", "openai"],
        ),
        &server,
    ));
    assert_eq!(r["next"], "ask_user", "{r}");
    assert!(
        say(&r).contains("have no `migrate-from-openai` skill yet"),
        "{r}"
    );
    assert_eq!(left_behind(), 0);
    assert_eq!(std::fs::read_dir(cwd.path()).unwrap().count(), 0);
}

/// The command each request to the mock was counted as, in order.
async fn counted_as(server: &MockServer) -> Vec<String> {
    let sent = server.received_requests().await.unwrap();
    sent.iter()
        .map(|r| r.headers["user-agent"].to_str().unwrap().to_string())
        .map(|agent| {
            agent
                .split(' ')
                .find(|tag| tag.starts_with("cmd/"))
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

#[tokio::test]
async fn a_migrations_hand_off_is_counted_once_it_has_a_working_key() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let onboard = |args: &[&str]| {
        reply(&mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &[&["onboard"][..], args].concat(),
        ))
    };
    // The reply is onboarding's own, with or without the mark: only `run` carries it on.
    let plain = onboard(&["init"]);
    let marked = onboard(&["init", "--from", "openai"]);
    assert_eq!(marked["next"], "integrate", "{marked}");
    for field in ["instruction", "say", "rules"] {
        assert_eq!(marked[field], plain[field], "{field}");
    }
    assert_eq!(
        plain["run"],
        "elevenlabs onboard check --written --env-path .env"
    );
    assert_eq!(
        marked["run"],
        "elevenlabs onboard check --written --env-path .env --from openai"
    );
    // The closing step keeps the mark, and counts nothing: the key step already did.
    let offer = onboard(&[
        "check",
        "--written",
        "--env-path",
        ".env",
        "--from",
        "openai",
    ]);
    assert_eq!(offer["next"], "offer_test", "{offer}");
    assert_eq!(
        offer["run"],
        "elevenlabs onboard test --env-path .env --from openai"
    );
    onboard(&["init", "--from", "openai", "--telemetry", "off"]);
    // Each run checks the key under its own name; only a marked one that finds a working key,
    // with telemetry on, adds the count.
    assert_eq!(
        counted_as(&server).await,
        [
            "cmd/onboard.init",
            "cmd/onboard.init",
            "cmd/migrate.openai",
            "cmd/onboard.check",
            "cmd/onboard.init"
        ]
    );

    // A key ElevenLabs rejects is no migration yet.
    let (home, cwd) = temp();
    write(cwd.path(), ".env", "ELEVENLABS_API_KEY=sk_rejected\n");
    let before = server.received_requests().await.unwrap().len();
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init", "--from", "openai"],
    ));
    assert_ne!(r["next"], "integrate", "{r}");
    assert_eq!(server.received_requests().await.unwrap().len(), before + 1);
}

#[tokio::test]
async fn a_hand_off_whose_key_arrives_on_a_later_step_is_counted_there() {
    // No browser reaches this machine, so the developer pastes a key and `check` finds it.
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    git(cwd.path(), &["init", "-q"]);
    let remote = |args: &[&str]| {
        reply(
            cli(home.path(), cwd.path(), &server.uri(), args)
                .env("SSH_CONNECTION", "10.0.0.1 22 10.0.0.2 22"),
        )
    };
    let asked = remote(&["onboard", "init", "--from", "openai"]);
    assert_eq!(asked["next"], "ask_user", "{asked}");
    assert_eq!(
        asked["run"],
        "elevenlabs onboard check --env-path .env --from openai"
    );
    assert!(
        counted_as(&server).await.is_empty(),
        "no key yet, so nothing to count"
    );

    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let found = remote(&["onboard", "check", "--env-path", ".env", "--from", "openai"]);
    assert_eq!(found["next"], "integrate", "{found}");
    assert_eq!(
        counted_as(&server).await,
        ["cmd/onboard.check", "cmd/migrate.openai"]
    );
}
