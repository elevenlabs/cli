//! End-to-end tests for cursor pagination through the public CLI.
//!
//! The mock server keeps these independent of an ElevenLabs account or API
//! key. Each test verifies that `--page-all` uses the cursor field names from
//! the operation's OpenAPI definition when requesting the second page.

use std::path::Path;
use std::process::{Command, Output};

use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn cli(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .env("HOME", home)
        .env("ELEVENLABS_API_KEY", "test-key")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env_remove("ELEVENLABS_BASE_URL")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

#[tokio::test]
async fn page_all_replaces_starting_next_page_token_with_response_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param("next_page_token", "start-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-first-page"}],
            "has_more": true,
            "next_page_token": "voice-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param("next_page_token", "voice-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-second-page"}],
            "has_more": false,
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "voices",
            "search",
            "--next-page-token",
            "start-token",
            "--page-size",
            "1",
            "--page-all",
            "--page-limit",
            "2",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        out.status.success(),
        "CLI should fetch both voice pages: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(requests.len(), 2, "both voice pages should be requested");
    let second_page_tokens: Vec<_> = requests[1]
        .url
        .query_pairs()
        .filter(|(name, _)| name == "next_page_token")
        .map(|(_, value)| value.to_string())
        .collect();
    assert_eq!(second_page_tokens, ["voice-page-two"]);
    assert!(String::from_utf8_lossy(&out.stdout).contains("voice-first-page"));
    assert!(String::from_utf8_lossy(&out.stdout).contains("voice-second-page"));
}

#[tokio::test]
async fn page_all_reports_missing_cursor_when_more_pages_are_declared() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-first-page"}],
            "has_more": true,
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "voices",
            "search",
            "--page-all",
            "--page-limit",
            "2",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        !out.status.success(),
        "incomplete pagination must not report success"
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("cursor"),
        "missing-cursor diagnostic: stderr={} stdout={}",
        String::from_utf8_lossy(&out.stderr),
        String::from_utf8_lossy(&out.stdout)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn page_all_follows_cursor_from_the_next_cursor_response_field() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/agents"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "agents": [{"agent_id": "agent-first-page"}],
            "has_more": true,
            "next_cursor": "agent-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/agents"))
        .and(query_param("cursor", "agent-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "agents": [{"agent_id": "agent-second-page"}],
            "has_more": false,
            "next_cursor": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "agents",
            "list",
            "--page-size",
            "1",
            "--page-all",
            "--page-limit",
            "2",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        out.status.success(),
        "CLI should fetch both agent pages: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(requests.len(), 2, "both agent pages should be requested");
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-first-page"));
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-second-page"));
}

#[tokio::test]
async fn page_all_uses_history_cursor_and_stops_when_has_more_is_false() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/history"))
        .and(query_param_is_missing("start_after_history_item_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "history": [{"history_item_id": "history-first-page"}],
            "has_more": true,
            "last_history_item_id": "history-item-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/history"))
        .and(query_param(
            "start_after_history_item_id",
            "history-item-two",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "history": [{"history_item_id": "history-second-page"}],
            "has_more": false,
            "last_history_item_id": "history-item-two"
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "history",
            "list",
            "--page-size",
            "1",
            "--page-all",
            "--page-limit",
            "3",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        out.status.success(),
        "CLI should fetch both history pages: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(requests.len(), 2, "both history pages should be requested");
    assert!(String::from_utf8_lossy(&out.stdout).contains("history-first-page"));
    assert!(String::from_utf8_lossy(&out.stdout).contains("history-second-page"));
}

#[tokio::test]
async fn page_all_follows_cursor_when_response_has_multiple_result_arrays() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/knowledge-base/doc-1/dependent-agents"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "agents": [{"id": "agent-first-page", "type": "unknown"}],
            "branches": [],
            "has_more": true,
            "next_cursor": "dependent-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/knowledge-base/doc-1/dependent-agents"))
        .and(query_param("cursor", "dependent-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "agents": [{"id": "agent-second-page", "type": "unknown"}],
            "branches": [],
            "has_more": false,
            "next_cursor": "stale-cursor"
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "agents",
            "knowledge-base",
            "documents",
            "get_agents",
            "--documentation-id",
            "doc-1",
            "--page-all",
            "--page-limit",
            "3",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        out.status.success(),
        "CLI should fetch both dependent-agent pages: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(requests.len(), 2, "has_more=false should stop at page two");
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-first-page"));
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-second-page"));
}

#[tokio::test]
async fn page_all_follows_cursor_when_results_have_a_union_type() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/secrets/secret-1/dependencies/agents"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "dependencies": [{"id": "agent-first-page", "type": "unknown"}],
            "next_cursor": "dependency-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/secrets/secret-1/dependencies/agents"))
        .and(query_param("cursor", "dependency-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "dependencies": [{"id": "agent-second-page", "type": "unknown"}],
            "next_cursor": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base_url = server.uri();
    let out = cli(
        home.path(),
        &[
            "agents",
            "secrets",
            "get_dependencies",
            "--secret-id",
            "secret-1",
            "--resource-type",
            "agents",
            "--page-all",
            "--page-limit",
            "3",
            "--page-delay",
            "0",
            "--no-pager",
            "--base-url",
            &base_url,
        ],
    );

    assert!(
        out.status.success(),
        "CLI should fetch both dependency pages: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let requests = server.received_requests().await.expect("recorded requests");
    assert_eq!(
        requests.len(),
        2,
        "both dependency pages should be requested"
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-first-page"));
    assert!(String::from_utf8_lossy(&out.stdout).contains("agent-second-page"));
}
