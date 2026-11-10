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
        // Names the developer's own Claude Code, which `setup` would install into.
        "CLAUDE_CODE_EXECPATH",
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
            Some("elevenlabs onboard connect --wait --env-path .env.local")
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
        r["run"], "elevenlabs onboard connect --env-path .env.local",
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
    // An app with no agent in it never looks one up.
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
    // Always the same question first, and no product chosen for the developer.
    assert_eq!(r["next"], "ask_what_to_build", "{r}");
    assert_eq!(
        say(&r),
        "Your key works. What would you like to build with ElevenLabs, or should I just add the SDK for now?"
    );
    assert_eq!(
        r["run"],
        "elevenlabs onboard check --answered --env-path .env.local"
    );
    assert!(r["details"].get("skills_dir").is_none(), "{r}");

    // Once the developer has answered: how to write the code, with the rules again.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--answered"],
    ));
    assert_eq!(r["next"], "write_code", "{r}");
    assert!(r["rules"].as_array().is_some_and(|r| !r.is_empty()), "{r}");
    assert_eq!(
        r["run"],
        "elevenlabs onboard check --answered --env-path .env.local"
    );
    assert!(
        r["details"]["load_key"]
            .as_str()
            .unwrap()
            .contains("--env-file=.env.local"),
        "{r}"
    );
    assert!(
        r["details"].get("ask_what_to_build").is_none() && r["details"].get("voice_id").is_none(),
        "{r}"
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
    // The first check secured the file.
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

    // The SDK declared and imported, but nothing in the app loads .env.local: the key
    // wouldn't reach it, so the next step is loading it, not the test.
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
    assert_eq!(r["next"], "load_key", "{r}");
    assert!(say(&r).ends_with("nothing in the app loads .env.local yet."), "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --env-path .env.local");
    assert!(
        r["details"]["load_key"].as_str().unwrap().contains("`node --env-file=.env.local`"),
        "{r}"
    );

    // Once the start script loads it, the next step is offering the test.
    write(
        cwd.path(),
        "package.json",
        r#"{"scripts":{"start":"node --env-file=.env.local src/speak.js"},"dependencies":{"@elevenlabs/elevenlabs-js":"^2"}}"#,
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "offer_test", "{r}");
    assert_eq!(r["run"], "elevenlabs onboard test --env-path .env.local");
    assert!(say(&r).contains("Want me to send a test request?"), "{r}");
    assert_eq!(r["details"]["fixed"], json!([]), "nothing left to fix");
    server.verify().await;
}

/// `GET /v1/convai/agents/{id}` answers `key` with an agent of `name` and
/// these `auth` settings.
async fn agent_endpoint(server: &MockServer, key: &str, id: &str, name: &str, auth: Value) {
    Mock::given(method("GET"))
        .and(path(format!("/v1/convai/agents/{id}")))
        .and(header("xi-api-key", key))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "agent_id": id,
            "name": name,
            "platform_settings": {"auth": auth},
        })))
        .mount(server)
        .await;
}

/// The page the agent widget's embed snippet makes.
fn embed(id: &str) -> String {
    format!(
        "<body>\n<elevenlabs-convai agent-id=\"{id}\"></elevenlabs-convai>\n\
         <script src=\"https://unpkg.com/@elevenlabs/convai-widget-embed\" async></script>\n</body>\n"
    )
}

#[tokio::test]
async fn a_page_with_the_agent_widget_ends_on_talking_to_the_agent() {
    const MINE: &str = "agent_7101k5zvyjhmfg983brhmhkd98n6";
    const LISTED: &str = "agent_2201m2jqpj2jej1tbfdrctx18r7b";
    const PRIVATE: &str = "agent_0101kf1yn87vfwat2negbz8zzp5t";
    const OTHERS: &str = "agent_1701kj50p7zzfd8vsp8hpapc4yhq";
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    agent_endpoint(
        &server,
        KEY,
        MINE,
        "Store helper",
        json!({"enable_auth": false, "allowlist": []}),
    )
    .await;
    agent_endpoint(
        &server,
        KEY,
        LISTED,
        "Shop",
        json!({"enable_auth": false, "allowlist": [{"hostname": "mystore.com"}]}),
    )
    .await;
    agent_endpoint(
        &server,
        KEY,
        PRIVATE,
        "Back office",
        json!({"enable_auth": true}),
    )
    .await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/convai/agents/{OTHERS}")))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"detail": {"status": "not_found"}})),
        )
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    let check = |args: &[&str]| reply(&mut cli(home.path(), cwd.path(), &server.uri(), args));

    // A plain site whose only ElevenLabs code is the widget: wired, with nothing to load,
    // since the widget never reads the key.
    write(cwd.path(), "index.html", &embed(MINE));
    let r = check(&["onboard", "check", "--answered"]);
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(
        say(&r),
        format!(
            "ElevenLabs is set up: a working key ending in 4567 in .env.local, and your agent \"Store helper\" \
             in your code. Talk to it at https://elevenlabs.io/app/talk-to?agent_id={MINE}, and change it at \
             https://elevenlabs.io/app/agents/agents/{MINE}."
        )
    );
    assert!(r.get("run").is_none(), "nothing left to run: {r}");
    assert_eq!(r["details"]["agent_id"], MINE);
    assert_eq!(
        r["details"]["talk_to_url"],
        format!("https://elevenlabs.io/app/talk-to?agent_id={MINE}")
    );

    // The talk-to page can't open an agent limited to the developer's own sites.
    write(cwd.path(), "index.html", &embed(LISTED));
    let r = check(&["onboard", "check"]);
    assert_eq!(r["next"], "done", "{r}");
    assert!(
        say(&r).contains("Only the sites on its allowlist can connect to it"),
        "{r}"
    );
    assert!(r["details"].get("talk_to_url").is_none(), "{r}");

    // A private agent: a page with only the widget can't reach it, and says what to do.
    write(cwd.path(), "index.html", &embed(PRIVATE));
    let r = check(&["onboard", "check"]);
    assert_eq!(r["next"], "done", "{r}");
    assert!(
        say(&r).ends_with(&format!(
            "It requires authentication, so your page can't connect to it yet: turn authentication off on \
             its page (https://elevenlabs.io/app/agents/agents/{PRIVATE}), or have your server request a \
             signed URL for each conversation."
        )),
        "{r}"
    );
    assert!(r["details"].get("talk_to_url").is_none(), "{r}");
    // With a server that signs its conversations (and loads the key), it's tried from its page.
    write(
        cwd.path(),
        "server.js",
        "const key = process.env.ELEVENLABS_API_KEY; // signs each conversation\n",
    );
    write(
        cwd.path(),
        "package.json",
        r#"{"scripts":{"start":"node --env-file=.env.local server.js"}}"#,
    );
    let r = check(&["onboard", "check"]);
    assert!(
        say(&r).ends_with(&format!(
            "It needs a signed URL to talk to it, so try it from its page: https://elevenlabs.io/app/agents/agents/{PRIVATE}."
        )),
        "{r}"
    );
    std::fs::remove_file(cwd.path().join("server.js")).unwrap();
    std::fs::remove_file(cwd.path().join("package.json")).unwrap();

    // An agent the key can't open, like a sample's, is never offered: the test request is.
    write(cwd.path(), "index.html", &embed(OTHERS));
    let r = check(&["onboard", "check"]);
    assert_eq!(r["next"], "offer_test", "{r}");
    assert!(say(&r).contains("and ElevenLabs in your code"), "{r}");

    // Nor is the widget's stand-in ID; a page without the widget still needs code.
    write(cwd.path(), "index.html", &embed("your-agent-id"));
    assert_eq!(check(&["onboard", "check"])["next"], "offer_test");
    write(cwd.path(), "index.html", "<body>Hello</body>\n");
    assert_eq!(
        check(&["onboard", "check", "--answered"])["next"],
        "write_code"
    );

    // The widget installed from npm in a Vite app reads no key either, so there's nothing to load.
    write(
        cwd.path(),
        "package.json",
        r#"{"dependencies":{"vite":"7","@elevenlabs/convai-widget-embed":"0.4"}}"#,
    );
    write(
        cwd.path(),
        "src/main.js",
        &format!("import \"@elevenlabs/convai-widget-embed\";\nexport const id = \"{MINE}\";\n"),
    );
    let r = check(&["onboard", "check"]);
    assert_eq!(
        (r["next"].as_str(), r["details"]["agent_id"].as_str()),
        (Some("done"), Some(MINE)),
        "{r}"
    );
    std::fs::remove_dir_all(cwd.path().join("src")).unwrap();

    // The SDK in a Next.js app (which reads .env.local), with the agent's ID in the env file.
    write(cwd.path(), "index.html", "");
    write(
        cwd.path(),
        "package.json",
        r#"{"dependencies":{"next":"15","@elevenlabs/react":"1"}}"#,
    );
    write(
        cwd.path(),
        "app/page.tsx",
        "import { useConversation } from \"@elevenlabs/react\";\nconst agentId = process.env.NEXT_PUBLIC_AGENT_ID;\n",
    );
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\nNEXT_PUBLIC_AGENT_ID={MINE}\n"),
    );
    let r = check(&["onboard", "check"]);
    assert_eq!(
        (r["next"].as_str(), r["details"]["agent_id"].as_str()),
        (Some("done"), Some(MINE)),
        "{r}"
    );
}

#[tokio::test]
async fn agent_lookups_stop_at_three_and_at_a_key_that_cant_read_agents() {
    let ids = [
        "agent_0101kf1yn87vfwat2negbz8zzp5t",
        "agent_1701kj50p7zzfd8vsp8hpapc4yhq",
        "agent_2201m2jqpj2jej1tbfdrctx18r7b",
        "agent_2701m62trrmgfa1vneex0grrd868",
    ];
    let page: String = ids.iter().map(|id| embed(id)).collect();

    // None of them is the developer's: three are looked up, then the test request is offered.
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    Mock::given(method("GET"))
        .and(path_regex("^/v1/convai/agents/agent_"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"detail": {"status": "not_found"}})),
        )
        .expect(3)
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    write(
        cwd.path(),
        ".env.local",
        &format!("ELEVENLABS_API_KEY={KEY}\n"),
    );
    write(cwd.path(), "index.html", &page);
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "offer_test", "{r}");
    server.verify().await;

    // A key without agent access is refused on the first one, so the rest aren't tried.
    let restricted = MockServer::start().await;
    user_endpoint_accepting(&restricted, KEY).await;
    Mock::given(method("GET"))
        .and(path_regex("^/v1/convai/agents/agent_"))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"detail": {"status": "missing_permissions"}})),
        )
        .expect(1)
        .mount(&restricted)
        .await;
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &restricted.uri(),
        &["onboard", "check"],
    ));
    assert_eq!(r["next"], "offer_test", "{r}");
    restricted.verify().await;
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
    assert_eq!(r["next"], "ask_what_to_build", "{r}");
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
            .contains(&format!("--env-path {env}")),
        "{r}"
    );
}

#[tokio::test]
async fn check_tells_a_dotenv_key_apart_from_the_env_file() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    write(cwd.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    // How to load the key comes with the code, once the developer has answered.
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "check", "--answered"],
    ));
    assert_eq!(r["details"]["api_key_source"], "dotenv", "{r}");
    // The code is told to load the file the key is in, not the default env file.
    assert_eq!(r["details"]["env_file"], ".env", "{r}");
    assert!(
        r["details"]["load_key"]
            .as_str()
            .unwrap()
            .contains("It's in .env. "),
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
        r["run"], "elevenlabs onboard check --env-path .env.local",
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
    assert_eq!(r["run"], "elevenlabs onboard connect --env-path .env.local");
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
        r["run"], "elevenlabs onboard connect --env-path .env.local --no-browser",
        "init's flags go on to connect"
    );

    // Under npx, every next command runs the same latest CLI.
    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "init"])
            .env("npm_command", "exec"),
    );
    assert_eq!(
        r["run"],
        "npx -y @elevenlabs/cli@latest onboard connect --env-path .env.local"
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
        r["next"], "ask_what_to_build",
        "the check reached the API directly: {r}"
    );
}

#[tokio::test]
async fn init_checks_a_key_that_is_already_there() {
    // The check's own reply, so the question about what to build can't be skipped.
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
    assert_eq!(r["next"], "ask_what_to_build", "{r}");
    assert!(say(&r).ends_with("or should I just add the SDK for now?"), "{r}");
    assert_eq!(r["run"], "elevenlabs onboard check --answered --env-path .env.local");
    assert_eq!(r["details"]["env_file"], ".env.local");
    assert!(r["rules"].as_array().is_some_and(|r| !r.is_empty()), "{r}");

    // The SDK already wired in, and the key loaded: straight to the test offer.
    write(
        cwd.path(),
        "package.json",
        r#"{"scripts":{"start":"node --env-file=.env.local index.js"},"dependencies":{"@elevenlabs/elevenlabs-js":"^2"}}"#,
    );
    write(
        cwd.path(),
        "index.js",
        "import { ElevenLabsClient } from \"@elevenlabs/elevenlabs-js\";\n",
    );
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        &server.uri(),
        &["onboard", "init"],
    ));
    assert_eq!(r["next"], "offer_test", "{r}");
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
            Some("elevenlabs onboard check --env-path .env.local")
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
        run.starts_with("elevenlabs onboard connect --replace --env-path ")
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
            Some("elevenlabs onboard connect --env-path .env.local")
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
    for flag in ["--env-path", "--no-browser", "--replace", "--wait"] {
        assert!(help.contains(flag), "connect help lacks {flag}:\n{help}");
    }
    // The old name still parses, but isn't offered: through npx, Node takes it.
    assert!(!help.contains("--env-file"), "{help}");
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
    let deadline = Instant::now() + Duration::from_secs(60);
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
    // The approval page records where the CLI runs.
    let this_os = match std::env::consts::OS {
        os @ ("windows" | "macos" | "linux") => os,
        _ => "other",
    };
    assert_eq!(query(&url, "cli_os"), this_os, "{url}");
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
        r["run"].as_str().unwrap().contains("--env-path other.env"),
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
    // Generous: the background sign-in is slow on a loaded machine, and the loop ends when it does.
    let deadline = Instant::now() + Duration::from_secs(60);
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

/// Where the file credential store keeps sign-ins for a `HOME` of `home`.
fn credential_store(home: &Path) -> PathBuf {
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/elevenlabs")
    } else if cfg!(windows) {
        home.join("AppData").join("Roaming").join("elevenlabs")
    } else {
        home.join(".config/elevenlabs")
    }
}

#[tokio::test]
async fn a_sign_in_the_credential_store_refuses_still_finishes() {
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
    // A file where the store keeps its folder, so saving the sign-in fails, as
    // a keychain that refuses the CLI does.
    let store = credential_store(home.path());
    std::fs::create_dir_all(store.parent().unwrap()).unwrap();
    std::fs::write(&store, "").unwrap();
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
    let (session, url) = approval(home.path());
    let page = approve(&url);
    assert!(
        page.contains("Location: https://elevenlabs.io/app/oauth/cli/success\r\n"),
        "{page}"
    );
    let deadline = Instant::now() + Duration::from_secs(60);
    let done = loop {
        let r = connect(home.path(), cwd.path(), &["--wait", "--no-browser"]);
        if r["next"] != "wait" || Instant::now() >= deadline {
            break r;
        }
    };
    // Signed in: no key reached the clipboard, so the env file was prepared for one.
    assert_eq!(done["details"]["key_status"], "no_key", "{done}");
    let log = std::fs::read_to_string(session.join("worker.log")).unwrap();
    assert!(log.contains("couldn't be saved"), "{log}");
}

/// What the approval link and the requests say about the machine, the agent
/// and the last sign-in follows the telemetry switch: with it off, nothing.
#[tokio::test]
async fn with_telemetry_off_nothing_is_said_about_the_machine_or_the_last_sign_in() {
    let _turn = signing_in().await;
    if callback_ports_busy() {
        eprintln!("skipped: another sign-in is listening on the callback ports");
        return;
    }
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    let (home, cwd) = temp();
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    // A sign-in run by Claude Code, for its own env file so each one is new,
    // declined once its link is out. Returns that link.
    let declined_sign_in = |env_file: &str, telemetry: &[(&str, &str)], args: &[&str]| {
        let mut cmd = cli(
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
                    "--env-file",
                    env_file,
                ],
                args,
            ]
            .concat(),
        );
        cmd.env("CLAUDECODE", "1").envs(telemetry.iter().copied());
        let url = link(&reply(&mut cmd));
        decline(&url);
        let (session, _) = approval(home.path());
        let deadline = Instant::now() + Duration::from_secs(30);
        while !session.join("result.json").exists() {
            assert!(Instant::now() < deadline, "the sign-in never ended");
            std::thread::sleep(Duration::from_millis(50));
        }
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

    let first = declined_sign_in("a.env", &[], &[]);
    assert_eq!(query(&first, "cli_agent"), "claude-code", "{first}");
    for (env_file, telemetry, args) in [
        ("b.env", &[][..], &["--telemetry", "off"][..]),
        ("c.env", &[("DO_NOT_TRACK", "1")][..], &[][..]),
        ("d.env", &[("ELEVENLABS_TELEMETRY", "off")][..], &[][..]),
    ] {
        let url = declined_sign_in(env_file, telemetry, args);
        assert_eq!(said(&url), Vec::<String>::new(), "{url}");
    }
    // Back on, the next link says how the last sign-in ended.
    let on = declined_sign_in("e.env", &[], &[]);
    assert_eq!(query(&on, "cli_prev"), "declined", "{on}");
    assert_eq!(query(&on, "cli_prev_id").len(), 16, "{on}");
    assert_eq!(query(&on, "cli_agent"), "claude-code", "{on}");

    // The requests after it say so too, unless telemetry is off.
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
        assert!(agent.contains(" signin/declined"), "{agent}");
        assert!(agent.contains(" agent/claude-code"), "{agent}");
    }
    for agent in off {
        assert!(!agent.contains("signin/"), "{agent}");
        assert!(!agent.contains("agent/"), "{agent}");
        assert!(!agent.contains("os/"), "{agent}");
    }
}

// ── setup, and the offer at the end of the flow ─────────────────────

/// The sign-in folder `onboard` keeps for the project in `cwd`, named as
/// `Session::for_project` names it.
fn session_dir(home: &Path, cwd: &Path) -> PathBuf {
    use sha2::Digest;
    let canonical = cwd.canonicalize().unwrap();
    let id: String = sha2::Sha256::digest(canonical.to_string_lossy().as_bytes())
        .iter()
        .take(8)
        .map(|b| format!("{b:02x}"))
        .collect();
    home.join(".elevenlabs").join("onboard").join(id)
}

/// What a sign-in whose page asked for the `setup` offer leaves.
fn setup_offer_due(home: &Path, cwd: &Path) -> PathBuf {
    let dir = session_dir(home, cwd);
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("setup-offer");
    std::fs::write(&file, "").unwrap();
    file
}

/// Stand-ins for the agents' programs in a folder of their own, each adding
/// the command it was given to `record`, and failing the one whose arguments
/// contain `fail`. Returns a PATH with only them and the system's tools.
#[cfg(unix)]
fn stand_ins(dir: &Path, names: &[&str], record: &Path, fail: Option<&str>) -> String {
    use std::os::unix::fs::PermissionsExt;
    for name in names {
        let fails = match fail {
            Some(args) => format!(
                "case \"$*\" in *\"{args}\"*) echo 'something broke' >&2; echo 'install refused' >&2; exit 1;; esac\n"
            ),
            None => String::new(),
        };
        let script = format!(
            "#!/bin/sh\necho \"{name} $*\" >> '{}'\n{fails}exit 0\n",
            record.display()
        );
        let path = dir.join(name);
        std::fs::write(&path, script).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
    format!("{}:/usr/bin:/bin", dir.display())
}

fn recorded(record: &Path) -> Vec<String> {
    std::fs::read_to_string(record)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

/// The same reply, apart from the clip, whose file is new each time.
fn without_clip(reply: &Value) -> String {
    let clip = reply["details"]["audio_file"].as_str().unwrap();
    say(reply).replace(clip, "<clip>")
}

const PLUGIN_QUESTION: &str = "Want me to set ElevenLabs up in {} too? It installs the ElevenLabs plugin \
    (skills and the hosted MCP server), so new sessions can use ElevenLabs right away.";

#[cfg(unix)]
#[tokio::test]
async fn a_test_request_that_worked_offers_setup_once_when_the_sign_in_asked() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ID3-fake-mp3".to_vec()))
        .mount(&server)
        .await;
    let failing = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(
            ResponseTemplate::new(500).set_body_json(json!({"detail": {"message": "down"}})),
        )
        .mount(&failing)
        .await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let record = bin.path().join("ran");
    let path = stand_ins(bin.path(), &["claude"], &record, None);
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let test = |server: &str| {
        reply(
            cli(home.path(), cwd.path(), server, &["onboard", "test"]).env("PATH", &path),
        )
    };

    let plain = test(&server.uri());
    assert_eq!(plain["next"], "done", "no offer without the sign-in asking: {plain}");

    let offer = setup_offer_due(home.path(), cwd.path());
    // A test request that failed ends as it always has, and the offer waits.
    let r = test(&failing.uri());
    assert_eq!(r["next"], "done", "{r}");
    assert!(r["details"].get("question").is_none(), "{r}");
    assert!(offer.exists());

    let r = test(&server.uri());
    assert_eq!(r["next"], "offer_setup", "{r}");
    assert_eq!(
        r["details"]["question"],
        PLUGIN_QUESTION.replace("{}", "Claude Code")
    );
    assert_eq!(r["run"], "elevenlabs onboard setup --env-path .env.local");
    assert_eq!(without_clip(&r), without_clip(&plain), "the same say");
    assert_eq!(r["details"]["status"], 200);
    assert!(std::path::Path::new(r["details"]["audio_file"].as_str().unwrap()).exists());
    assert!(r["instruction"].as_str().unwrap().contains("details.question"), "{r}");
    assert!(!offer.exists(), "made once");
    assert!(recorded(&record).is_empty(), "offering runs nothing");

    let r = test(&server.uri());
    assert_eq!(r["next"], "done", "{r}");
    assert!(r["details"].get("question").is_none(), "{r}");
}

#[cfg(unix)]
#[tokio::test]
async fn a_voice_agent_project_offers_setup_at_its_link() {
    const MINE: &str = "agent_7101k5zvyjhmfg983brhmhkd98n6";
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    agent_endpoint(
        &server,
        KEY,
        MINE,
        "Store helper",
        json!({"enable_auth": false, "allowlist": []}),
    )
    .await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let path = stand_ins(bin.path(), &["codex"], &bin.path().join("ran"), None);
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    write(cwd.path(), "index.html", &embed(MINE));
    let check = || {
        reply(
            cli(
                home.path(),
                cwd.path(),
                &server.uri(),
                &["onboard", "check", "--answered", "--no-browser"],
            )
            .env("PATH", &path),
        )
    };
    // The first check also fixes the file, which the details say once.
    check();
    let plain = check();
    assert_eq!(plain["next"], "done", "{plain}");

    let offer = setup_offer_due(home.path(), cwd.path());
    let r = check();
    assert_eq!(r["next"], "offer_setup", "{r}");
    assert_eq!(r["details"]["question"], PLUGIN_QUESTION.replace("{}", "Codex"));
    assert_eq!(
        r["run"],
        "elevenlabs onboard setup --env-path .env.local --no-browser"
    );
    assert_eq!(r["say"], plain["say"]);
    let mut details = r["details"].clone();
    details.as_object_mut().unwrap().remove("question");
    assert_eq!(details, plain["details"], "the same details, and the question");
    assert!(!offer.exists());
    assert_eq!(check()["next"], "done");
}

#[cfg(unix)]
#[tokio::test]
async fn with_only_cursor_there_is_no_offer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE}")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ID3".to_vec()))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    std::fs::create_dir(home.path().join(".cursor")).unwrap();
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let offer = setup_offer_due(home.path(), cwd.path());
    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "test"]).env("PATH", "/usr/bin:/bin"),
    );
    assert_eq!(r["next"], "done", "{r}");
    assert!(offer.exists(), "kept: Cursor has nothing to install with");
}

/// The six commands, in order, that install the plugin in Claude Code and Codex.
const INSTALLS: [&str; 6] = [
    "claude plugin marketplace update elevenlabs",
    "claude plugin install elevenlabs --marketplace elevenlabs/plugin --scope user",
    "claude plugin update elevenlabs@elevenlabs",
    "codex plugin marketplace add elevenlabs/plugin",
    "codex plugin marketplace upgrade elevenlabs",
    "codex plugin add elevenlabs@elevenlabs",
];

#[cfg(unix)]
#[tokio::test]
async fn onboard_setup_installs_in_each_agent_and_never_makes_speech() {
    let server = MockServer::start().await;
    user_endpoint_accepting(&server, KEY).await;
    Mock::given(method("POST"))
        .and(path_regex("^/v1/text-to-speech"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let record = bin.path().join("ran");
    let path = stand_ins(bin.path(), &["claude", "codex"], &record, None);
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));
    let setup = |args: &[&str], env: &[(&str, &str)]| {
        reply(
            cli(
                home.path(),
                cwd.path(),
                &server.uri(),
                &[&["onboard", "setup", "--env-path", ".env.local"][..], args].concat(),
            )
            .env("PATH", &path)
            .envs(env.iter().copied()),
        )
    };
    let user_requests = || async {
        server
            .received_requests()
            .await
            .unwrap()
            .into_iter()
            .filter(|r| r.url.path() == "/v1/user")
            .map(|r| r.headers["user-agent"].to_str().unwrap().to_string())
            .collect::<Vec<_>>()
    };

    let r = setup(&["--no-browser"], &[]);
    assert_eq!(r["next"], "done", "{r}");
    assert!(r.get("run").is_none(), "{r}");
    assert_eq!(recorded(&record), INSTALLS);
    assert_eq!(
        say(&r),
        "Claude Code: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, restart \
         Claude Code, then run /mcp and sign in to the ElevenLabs MCP server. Codex: installed the ElevenLabs \
         plugin (skills and the hosted MCP server). To finish, restart Codex, then run `codex mcp login \
         elevenlabs` to sign in to the ElevenLabs MCP server."
    );
    let results = r["details"]["setup"].as_array().unwrap();
    assert_eq!(results.len(), 2, "{r}");
    assert!(results.iter().all(|a| a["ok"] == true), "{r}");
    let agents = user_requests().await;
    assert_eq!(agents.len(), 1, "one account request with telemetry on: {agents:?}");
    assert!(agents[0].contains("cmd/onboard.setup"), "{}", agents[0]);

    std::fs::remove_file(&record).unwrap();
    for off in [&[("ELEVENLABS_TELEMETRY", "off")][..], &[("DO_NOT_TRACK", "1")][..]] {
        let r = setup(&[], off);
        assert_eq!(r["next"], "done", "{r}");
    }
    assert_eq!(recorded(&record).len(), 12, "installed both times");
    assert_eq!(user_requests().await.len(), 1, "none with telemetry off");

    // Cursor gets its link with the rest.
    std::fs::create_dir(home.path().join(".cursor")).unwrap();
    let r = setup(&[], &[]);
    assert!(
        say(&r).ends_with("Cursor: open https://cursor.com/marketplace/elevenlabs and select Install."),
        "{r}"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn a_failed_install_ends_on_the_command_to_run_later() {
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let record = bin.path().join("ran");
    let path = stand_ins(bin.path(), &["claude", "codex"], &record, Some("plugin install"));
    for (npm, later) in [
        (None, "elevenlabs setup --yes"),
        (Some("exec"), "npx -y @elevenlabs/cli@latest setup --yes"),
    ] {
        let mut cmd = cli(home.path(), cwd.path(), &server.uri(), &["onboard", "setup"]);
        cmd.env("PATH", &path);
        if let Some(npm) = npm {
            cmd.env("npm_command", npm);
        }
        let r = reply(&mut cmd);
        assert_eq!(r["next"], "done", "{r}");
        assert!(
            say(&r).starts_with(
                "Claude Code: failed: `claude plugin install elevenlabs --marketplace elevenlabs/plugin --scope user` \
                 exited with 1: install refused. Codex: installed"
            ),
            "{r}"
        );
        assert!(
            say(&r).ends_with(&format!("To try again later, run `{later}`.")),
            "{r}"
        );
        assert_eq!(r["details"]["setup"][0]["ok"], false, "{r}");
    }
    // No request without a key, telemetry on or not.
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(unix)]
#[test]
fn setup_says_what_it_would_do_and_does_it_with_yes() {
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let record = bin.path().join("ran");
    let path = stand_ins(bin.path(), &["claude", "codex"], &record, None);
    let run = |args: &[&str], path: &str| {
        cli(home.path(), cwd.path(), OFFLINE, args)
            .env("PATH", path)
            .output()
            .unwrap()
    };

    let out = run(&["setup"], &path);
    assert!(out.status.success(), "{}", all_output(&out));
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert_eq!(
        stdout,
        format!(
            "Claude Code ({0}/claude): install the ElevenLabs plugin (skills and the hosted MCP server)\n\
             Codex ({0}/codex): install the ElevenLabs plugin (skills and the hosted MCP server)\n\
             Nothing was changed. Run `elevenlabs setup --yes` to do it.\n",
            bin.path().display()
        )
    );
    assert!(recorded(&record).is_empty(), "nothing ran");

    for _ in 0..2 {
        let out = run(&["setup", "--yes"], &path);
        assert!(out.status.success(), "{}", all_output(&out));
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(stdout.contains("\nTo finish:\n- restart Claude Code, then run /mcp"), "{stdout}");
    }
    assert_eq!(recorded(&record), [INSTALLS, INSTALLS].concat(), "again: the update commands run too");

    let failing = tempfile::tempdir().unwrap();
    let path = stand_ins(failing.path(), &["claude"], &failing.path().join("ran"), Some("plugin update"));
    let out = run(&["setup", "--yes"], &path);
    assert_eq!(out.status.code(), Some(1), "{}", all_output(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "Claude Code: failed: `claude plugin update elevenlabs@elevenlabs` exited with 1: install refused\n\
         To try again, run `elevenlabs setup --yes`.\n"
    );

    // Only Cursor: its link, and nothing written.
    let (home, cwd) = temp();
    std::fs::create_dir(home.path().join(".cursor")).unwrap();
    let out = cli(home.path(), cwd.path(), OFFLINE, &["setup", "--yes"])
        .env("PATH", "/usr/bin:/bin")
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", all_output(&out));
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "Cursor: open https://cursor.com/marketplace/elevenlabs and select Install\n"
    );
    let entries: Vec<_> = std::fs::read_dir(home.path()).unwrap().flatten().map(|e| e.file_name()).collect();
    assert_eq!(entries, [".cursor"], "nothing written");
}

/// Kills the process when the test ends, passed or not.
#[cfg(unix)]
struct KillOnDrop(i32);

#[cfg(unix)]
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        unsafe { libc::kill(self.0, libc::SIGKILL) };
    }
}

/// Stops the child when the test ends, unless it has already exited.
#[cfg(unix)]
struct StopOnDrop(std::process::Child);

#[cfg(unix)]
impl Drop for StopOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(unix)]
#[test]
fn ctrl_c_stops_the_agent_command_and_what_it_started() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::process::CommandExt;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let pid = bin.path().join("pid");
    let claude = bin.path().join("claude");
    std::fs::write(
        &claude,
        format!("#!/bin/sh\nsleep 300 &\necho $! > '{0}.new'\nmv '{0}.new' '{0}'\nwait\n", pid.display()),
    )
    .unwrap();
    std::fs::set_permissions(&claude, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut cmd = cli(home.path(), cwd.path(), OFFLINE, &["setup", "--yes"]);
    cmd.env("PATH", format!("{}:/usr/bin:/bin", bin.path().display()));
    // SIGINT at its default, as from a terminal, even where these tests run with it ignored.
    unsafe {
        cmd.pre_exec(|| {
            libc::signal(libc::SIGINT, libc::SIG_DFL);
            Ok(())
        });
    }
    let mut setup = StopOnDrop(cmd.spawn().unwrap());
    let setup = &mut setup.0;

    let deadline = Instant::now() + Duration::from_secs(20);
    while !pid.exists() {
        assert!(Instant::now() < deadline, "the stand-in never started");
        std::thread::sleep(Duration::from_millis(20));
    }
    let sleep: i32 = std::fs::read_to_string(&pid).unwrap().trim().parse().unwrap();
    let _sleep = KillOnDrop(sleep);

    unsafe { libc::kill(setup.id() as i32, libc::SIGINT) };
    let deadline = Instant::now() + Duration::from_secs(5);
    let status = loop {
        if let Some(status) = setup.try_wait().unwrap() {
            break status;
        }
        assert!(Instant::now() < deadline, "setup still runs after Ctrl-C");
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(status.code(), Some(130), "{status:?}");
    while unsafe { libc::kill(sleep, 0) } == 0 {
        assert!(Instant::now() < deadline, "sleep {sleep} still runs");
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
#[tokio::test]
async fn with_dry_run_setup_installs_nothing() {
    let server = MockServer::start().await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let record = bin.path().join("ran");
    let path = stand_ins(bin.path(), &["claude", "codex"], &record, None);
    write(cwd.path(), ".env.local", &format!("ELEVENLABS_API_KEY={KEY}\n"));

    let out = cli(home.path(), cwd.path(), &server.uri(), &["setup", "--yes", "--dry-run"])
        .env("PATH", &path)
        .output()
        .unwrap();
    assert!(out.status.success(), "{}", all_output(&out));
    assert!(
        String::from_utf8_lossy(&out.stdout)
            .ends_with("Nothing was changed. Run `elevenlabs setup --yes` to do it.\n"),
        "{}",
        all_output(&out)
    );

    let r = reply(
        cli(home.path(), cwd.path(), &server.uri(), &["onboard", "setup", "--env-path", ".env.local", "--dry-run"])
            .env("PATH", &path),
    );
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(
        say(&r),
        "Nothing was installed in the coding agents, because of --dry-run. To set them up, run \
         `elevenlabs setup --yes`."
    );
    assert!(recorded(&record).is_empty(), "nothing ran");
    assert!(server.received_requests().await.unwrap().is_empty(), "no account request");
}

#[test]
fn onboard_setup_without_a_home_folder_says_so_and_ends() {
    let (home, cwd) = temp();
    let r = reply(
        cli(home.path(), cwd.path(), OFFLINE, &["onboard", "setup"])
            .env_remove("HOME")
            .env_remove("USERPROFILE")
            .env("PATH", "/usr/bin:/bin"),
    );
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(
        say(&r),
        "ElevenLabs could not be set up in the coding agents: no home folder was found. To try again \
         later, run `elevenlabs setup --yes`."
    );
}

#[tokio::test]
async fn the_sign_in_asks_for_the_setup_offer_and_keeps_the_pages_answer() {
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
            "expires_in": 3600
        })))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    let authorize = format!("{}/app/oauth/authorize", server.uri());
    // Signs in with telemetry off, and the page's callback carrying `params`.
    let sign_in = |params: &str| {
        let first = reply(&mut cli(
            home.path(),
            cwd.path(),
            &server.uri(),
            &["onboard", "connect", "--no-browser", "--authorize-url", &authorize, "--telemetry", "off"],
        ));
        let url = link(&first);
        assert_eq!(query(&url, "setup_offer"), "1", "{url}");
        callback(&url, &format!("code=approved{params}"));
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            let r = connect(home.path(), cwd.path(), &["--wait", "--no-browser"]);
            if r["next"] != "wait" || Instant::now() >= deadline {
                assert_eq!(r["details"]["key_status"], "no_key", "{r}");
                break;
            }
        }
    };
    let offer = session_dir(home.path(), cwd.path()).join("setup-offer");
    sign_in("&setup_offer=1");
    assert!(offer.exists(), "the page asked for the offer");
    sign_in("");
    assert!(!offer.exists(), "a new sign-in without it clears the last one's");
    sign_in("&setup_offer=yes");
    assert!(!offer.exists());
}
