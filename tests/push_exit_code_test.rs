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

// ── agents push ───────────────────────────────────────────────────

fn agent_config(name: &str) -> Value {
    json!({
        "name": name,
        "conversation_config": {
            "agent": {
                "first_message": "Hello",
                "prompt": {"prompt": "You are helpful.", "llm": "gemini-2.5-flash"}
            }
        }
    })
}

fn setup_agents_project(dir: &Path, agents_json: &Value, configs: &[(&str, &str)]) {
    for (path, name) in configs {
        write(dir, path, &agent_config(name).to_string());
    }
    write(dir, "agents.json", &agents_json.to_string());
}

#[tokio::test]
async fn agents_push_success_exits_zero() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_1"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_agents_project(
        dir,
        &json!({"agents": [{"config": "agent_configs/support.json", "id": "agent_1"}]}),
        &[("agent_configs/support.json", "Support")],
    );

    let out = cli(dir, &server.uri(), &["agents", "push"]);
    assert!(out.status.success(), "expected exit 0:\n{}", stdout(&out));
}

#[tokio::test]
async fn agents_push_api_error_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"detail": "server error"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_agents_project(
        dir,
        &json!({"agents": [{"config": "agent_configs/support.json", "id": "agent_1"}]}),
        &[("agent_configs/support.json", "Support")],
    );

    let out = cli(dir, &server.uri(), &["agents", "push"]);
    assert!(
        !out.status.success(),
        "expected non-zero exit:\n{}",
        stdout(&out)
    );
}

#[tokio::test]
async fn agents_push_partial_failure_pushes_others() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"detail": "server error"})))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_2"})))
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_agents_project(
        dir,
        &json!({"agents": [
            {"config": "agent_configs/support.json", "id": "agent_1"},
            {"config": "agent_configs/billing.json", "id": "agent_2"}
        ]}),
        &[
            ("agent_configs/support.json", "Support"),
            ("agent_configs/billing.json", "Billing"),
        ],
    );

    let out = cli(dir, &server.uri(), &["agents", "push"]);
    assert!(
        !out.status.success(),
        "expected non-zero exit on partial failure:\n{}",
        stdout(&out)
    );

    let requests = server.received_requests().await.unwrap();
    let agent_2_pushed = requests
        .iter()
        .any(|r| r.method.as_str() == "PATCH" && r.url.path() == "/v1/convai/agents/agent_2");
    assert!(
        agent_2_pushed,
        "agent_2 should still be pushed despite agent_1 failing"
    );
}

#[tokio::test]
async fn agents_push_branch_failure_exits_nonzero() {
    let server = MockServer::start().await;
    // Main update succeeds (once). The second PATCH (the branch push) finds no
    // matching mock and gets wiremock's default 404, which the CLI treats as an
    // API error — exactly what we need.
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_1"})))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_agents_project(
        dir,
        &json!({"agents": [{
            "config": "agent_configs/support.json",
            "id": "agent_1",
            "branch_id": "agtbrch_main",
            "branches": {
                "beta": {
                    "config": "agent_configs/support.beta.json",
                    "branch_id": "agtbrch_beta"
                }
            }
        }]}),
        &[
            ("agent_configs/support.json", "Support"),
            ("agent_configs/support.beta.json", "Support Beta"),
        ],
    );

    let out = cli(dir, &server.uri(), &["agents", "push"]);
    assert!(
        !out.status.success(),
        "expected non-zero exit on branch failure:\n{}",
        stdout(&out)
    );
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
            ResponseTemplate::new(200)
                .set_body_json(json!({"tool_id": "tool_1", "tool_config": tool_config()})),
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
    assert!(
        !out.status.success(),
        "expected non-zero exit:\n{}",
        stdout(&out)
    );
}

#[tokio::test]
async fn tools_push_create_no_id_exits_nonzero() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convai/tools"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"tool_config": tool_config()})),
        )
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let dir = dir.path();
    setup_tools_project(
        dir,
        &json!({"tools": [{"type": "webhook", "config": "tool_configs/lookup.json"}]}),
    );

    let out = cli(dir, &server.uri(), &["tools", "push"]);
    assert!(
        !out.status.success(),
        "expected non-zero exit when API returns no ID:\n{}",
        stdout(&out)
    );
}

// ── tests push ────────────────────────────────────────────────────

fn test_config() -> Value {
    json!({
        "name": "greeting check",
        "success_condition": "agent greets the user"
    })
}

fn setup_tests_project(dir: &Path, tests_json: &Value) {
    write(
        dir,
        "test_configs/greeting.json",
        &test_config().to_string(),
    );
    write(dir, "tests.json", &tests_json.to_string());
}

#[tokio::test]
async fn tests_push_success_exits_zero() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v1/convai/agent-testing/test_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"test_id": "test_1"})))
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
    assert!(
        !out.status.success(),
        "expected non-zero exit:\n{}",
        stdout(&out)
    );
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
    assert!(
        !out.status.success(),
        "expected non-zero exit when API returns no ID:\n{}",
        stdout(&out)
    );
}
