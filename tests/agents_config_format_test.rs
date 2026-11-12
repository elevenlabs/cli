//! Integration tests for YAML and TOML agent config files: `agents pull`,
//! `push` and `add` against a local mock API. The conversions themselves are
//! unit-tested in `cli/elevenlabs/workflow/config_format.rs`.

use std::path::Path;
use std::process::Output;

use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";

fn cli(dir: &Path, server: &str, args: &[&str]) -> Output {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(["--base-url", server])
        .args(args)
        .current_dir(dir)
        .env("HOME", dir)
        .env("TMPDIR", dir)
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("APPDATA")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("ELEVENLABS_API_KEY", KEY)
        .env("ELEVENLABS_TELEMETRY", "0");
    for var in ["ELEVENLABS_BASE_URL", "ELEVENLABS_VIA", "ELEVENLABS_OUTPUT"] {
        cmd.env_remove(var);
    }
    cmd.output().expect("failed to spawn the elevenlabs binary")
}

fn ok(out: Output, what: &str) -> String {
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "{what} failed:\n{text}");
    text
}

/// A live agent as the API returns it, nulls included.
fn live_agent() -> Value {
    json!({
        "agent_id": "agent_1",
        "name": "Support",
        "conversation_config": {
            "agent": {
                "first_message": "Hi, how can I help?",
                "language": null,
                "prompt": {
                    "prompt": "You are a support agent.\nBe brief.",
                    "llm": "gemini-2.5-flash",
                    "temperature": 0.3,
                    "tool_ids": ["tool_1"]
                }
            },
            "tts": {"voice_id": "voice_1", "stability": 0.5}
        },
        "platform_settings": {"testing": {"attached_tests": [{"test_id": "test_1"}]}},
        "tags": ["support"]
    })
}

async fn serve_agent(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(live_agent()))
        .mount(server)
        .await;
}

fn registered_config(dir: &Path) -> String {
    let index: Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("agents.json")).unwrap()).unwrap();
    index["agents"][0]["config"].as_str().unwrap().to_string()
}

fn last_body(requests: &[wiremock::Request], verb: &str) -> Value {
    let req = requests
        .iter()
        .rev()
        .find(|r| r.method.as_str() == verb)
        .unwrap_or_else(|| panic!("no {verb} request was sent"));
    serde_json::from_slice(&req.body).unwrap()
}

#[tokio::test]
async fn pull_writes_toml_and_push_sends_it_back() {
    let server = MockServer::start().await;
    serve_agent(&server).await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_1"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    ok(cli(dir, &server.uri(), &["agents", "init"]), "init");
    ok(
        cli(
            dir,
            &server.uri(),
            &[
                "agents",
                "pull",
                "--agent",
                "agent_1",
                "--config-format",
                "toml",
                "--yes",
            ],
        ),
        "pull",
    );

    let config = registered_config(dir);
    assert!(config.ends_with("Support.toml"), "{config}");
    let text = std::fs::read_to_string(dir.join(&config)).unwrap();
    assert!(
        text.contains("[conversation_config.agent.prompt]"),
        "{text}"
    );
    assert!(
        text.contains("[[platform_settings.testing.attached_tests]]"),
        "{text}"
    );
    assert!(
        !text.contains("language"),
        "nulls are left out of TOML:\n{text}"
    );

    ok(cli(dir, &server.uri(), &["agents", "push"]), "push");
    let body = last_body(&server.received_requests().await.unwrap(), "PATCH");
    let mut expected = live_agent();
    expected["conversation_config"]["agent"]
        .as_object_mut()
        .unwrap()
        .remove("language");
    for key in ["conversation_config", "platform_settings", "tags", "name"] {
        assert_eq!(body[key], expected[key], "{key}");
    }
}

#[tokio::test]
async fn pull_keeps_an_existing_files_format() {
    let server = MockServer::start().await;
    serve_agent(&server).await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    ok(cli(dir, &server.uri(), &["agents", "init"]), "init");
    std::fs::write(
        dir.join("agents.json"),
        r#"{"agents": [{"config": "agent_configs/support.yaml", "id": "agent_1"}]}"#,
    )
    .unwrap();
    // Asking for TOML only applies to new files.
    ok(
        cli(
            dir,
            &server.uri(),
            &[
                "agents",
                "pull",
                "--all",
                "--agent",
                "agent_1",
                "--config-format",
                "toml",
                "--yes",
            ],
        ),
        "pull",
    );
    let text = std::fs::read_to_string(dir.join("agent_configs/support.yaml")).unwrap();
    let parsed: Value = serde_yaml::from_str(&text).unwrap();
    assert_eq!(
        parsed["conversation_config"],
        live_agent()["conversation_config"]
    );
    assert!(!dir.join("agent_configs/Support.toml").exists());
}

#[tokio::test]
async fn add_from_a_yaml_file_creates_a_yaml_config() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convai/agents/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_2"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    ok(cli(dir, &server.uri(), &["agents", "init"]), "init");
    std::fs::write(
        dir.join("support.yml"),
        "name: Support\nconversation_config:\n  agent:\n    first_message: Hello\ntags: [a]\n",
    )
    .unwrap();
    ok(
        cli(
            dir,
            &server.uri(),
            &["agents", "add", "--from-file", "support.yml"],
        ),
        "add",
    );

    let body = last_body(&server.received_requests().await.unwrap(), "POST");
    assert_eq!(
        body["conversation_config"]["agent"]["first_message"],
        "Hello"
    );
    assert_eq!(body["tags"], json!(["a"]));

    let config = registered_config(dir);
    assert!(config.ends_with("Support.yaml"), "{config}");
    let parsed: Value =
        serde_yaml::from_str(&std::fs::read_to_string(dir.join(&config)).unwrap()).unwrap();
    assert_eq!(parsed["name"], "Support");
}

#[test]
fn a_bad_toml_config_names_the_format() {
    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    std::fs::write(dir.join("broken.toml"), "name = ").unwrap();
    let out = cli(
        dir,
        "http://127.0.0.1:9",
        &["agents", "workflow", "diagram", "broken.toml"],
    );
    assert!(!out.status.success());
    // Errors print as JSON on stdout when it isn't a terminal.
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        text.contains("Invalid TOML in configuration file"),
        "{text}"
    );
}
