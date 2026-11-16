//! Integration tests for push exit codes: verify that `agents push`,
//! `tools push` and `tests push` return non-zero when items fail.

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

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn write(dir: &Path, file: &str, text: &str) {
    let p = dir.join(file);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(p, text).unwrap();
}

// ── tools push ─────────────────────────────────────────────────────

fn tool_config() -> Value {
    json!({
        "name": "lookup",
        "description": "Lookup tool",
        "type": "webhook",
        "api_schema": {
            "url": "https://api.example.com/webhook",
            "method": "POST",
            "headers": {}
        }
    })
}

fn setup_tools_project(dir: &Path, tools_json: &Value) {
    write(dir, "tool_configs/lookup.json", &tool_config().to_string());
    write(dir, "tools.json", &tools_json.to_string());
}

#[tokio::test]
async fn tools_push_success_exits_zero() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/tools/tool_1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"tool_id": "tool_1", "tool_config": tool_config()})),
        )
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tools_project(
        dir,
        &json!({"tools": [{"type": "webhook", "config": "tool_configs/lookup.json", "id": "tool_1"}]}),
    );

    let out = cli(dir, &server.uri(), &["tools", "push"]);
    assert!(out.status.success(), "expected exit 0:\n{}", stdout(&out));
}

#[tokio::test]
async fn tools_push_api_error_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/tools/tool_1"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"detail": "server error"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tools_project(
        dir,
        &json!({"tools": [{"type": "webhook", "config": "tool_configs/lookup.json", "id": "tool_1"}]}),
    );

    let out = cli(dir, &server.uri(), &["tools", "push"]);
    assert!(!out.status.success(), "expected non-zero exit:\n{}", stdout(&out));
}

#[tokio::test]
async fn tools_push_create_no_id_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convai/tools"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"tool_config": tool_config()})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tools_project(
        dir,
        &json!({"tools": [{"type": "webhook", "config": "tool_configs/lookup.json"}]}),
    );

    let out = cli(dir, &server.uri(), &["tools", "push"]);
    assert!(!out.status.success(), "expected non-zero exit when API returns no ID:\n{}", stdout(&out));
}

// ── tests push ────────────────────────────────────────────────────

fn test_config() -> Value {
    json!({
        "name": "greeting check",
        "success_condition": "agent greets the user"
    })
}

fn setup_tests_project(dir: &Path, tests_json: &Value) {
    write(dir, "test_configs/greeting.json", &test_config().to_string());
    write(dir, "tests.json", &tests_json.to_string());
}

#[tokio::test]
async fn tests_push_success_exits_zero() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v1/convai/agent-testing/test_1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"test_id": "test_1"})),
        )
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tests_project(
        dir,
        &json!({"tests": [{"config": "test_configs/greeting.json", "id": "test_1"}]}),
    );

    let out = cli(dir, &server.uri(), &["tests", "push"]);
    assert!(out.status.success(), "expected exit 0:\n{}", stdout(&out));
}

#[tokio::test]
async fn tests_push_api_error_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v1/convai/agent-testing/test_1"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"detail": "server error"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tests_project(
        dir,
        &json!({"tests": [{"config": "test_configs/greeting.json", "id": "test_1"}]}),
    );

    let out = cli(dir, &server.uri(), &["tests", "push"]);
    assert!(!out.status.success(), "expected non-zero exit:\n{}", stdout(&out));
}

#[tokio::test]
async fn tests_push_create_no_id_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convai/agent-testing/create"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "greeting check"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tests_project(
        dir,
        &json!({"tests": [{"config": "test_configs/greeting.json"}]}),
    );

    let out = cli(dir, &server.uri(), &["tests", "push"]);
    assert!(!out.status.success(), "expected non-zero exit when API returns no ID:\n{}", stdout(&out));
}
