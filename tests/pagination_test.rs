//! `--page-all` against a local mock of the paginated ElevenLabs endpoints.
//!
//! Each test checks that the CLI follows the field names declared on that
//! operation, and that a failed or truncated page is a non-zero exit rather
//! than a successful partial listing.

use std::path::Path;
use std::process::{Command, Output};

use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn cli(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .env("HOME", home)
        .env("TMPDIR", home)
        .env("ELEVENLABS_API_KEY", "test-key")
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("APPDATA")
        .env_remove("ELEVENLABS_BASE_URL")
        .env_remove("ELEVENLABS_OUTPUT")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

fn combined(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn assert_success(out: &Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} failed (exit {:?})\n{}",
        out.status.code(),
        combined(out)
    );
}

fn page_flags<'a>(base_url: &'a str, limit: &'a str) -> Vec<&'a str> {
    vec![
        "--page-all",
        "--page-limit",
        limit,
        "--page-delay",
        "0",
        "--no-pager",
        "--no-retry",
        "--format",
        "json",
        "--base-url",
        base_url,
    ]
}

#[tokio::test]
async fn voices_search_follows_next_page_token_and_replaces_a_starting_token() {
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
    let base = server.uri();
    let mut args = vec![
        "voices",
        "search",
        "--next-page-token",
        "start-token",
        "--page-size",
        "1",
    ];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "voices search");

    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2, "both voice pages should be requested");
    let second: Vec<_> = requests[1]
        .url
        .query_pairs()
        .filter(|(name, _)| name == "next_page_token")
        .map(|(_, value)| value.to_string())
        .collect();
    assert_eq!(second, ["voice-page-two"]);
    let body = combined(&out);
    assert!(body.contains("voice-first-page"), "{body}");
    assert!(body.contains("voice-second-page"), "{body}");
}

#[tokio::test]
async fn voices_search_errors_when_has_more_has_no_token() {
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
    let base = server.uri();
    let mut args = vec!["voices", "search"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);

    assert!(
        !out.status.success(),
        "a truncated listing must not exit 0:\n{}",
        combined(&out)
    );
    let body = combined(&out);
    assert!(body.contains("has_more=true"), "{body}");
    assert!(body.contains("next_page_token"), "{body}");
    assert!(body.contains("incomplete"), "{body}");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn voices_search_errors_when_a_later_page_fails() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param_is_missing("next_page_token"))
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
        .respond_with(ResponseTemplate::new(500).set_body_string("{\"detail\":\"boom\"}"))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["voices", "search", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);

    assert!(
        !out.status.success(),
        "a failed later page must not exit 0:\n{}",
        combined(&out)
    );
    let body = combined(&out);
    assert!(
        body.contains("incomplete"),
        "the error should say the listing is incomplete:\n{body}"
    );
    assert!(
        !body.contains("voice-first-page"),
        "a failed later page must not be reported as a successful listing:\n{body}"
    );
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn page_limit_stops_before_the_last_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param_is_missing("next_page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-1"}],
            "has_more": true,
            "next_page_token": "tok-2"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param("next_page_token", "tok-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-2"}],
            "has_more": true,
            "next_page_token": "tok-3"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/voices"))
        .and(query_param("next_page_token", "tok-3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "voice-3"}],
            "has_more": false,
            "next_page_token": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["voices", "search"];
    args.extend(page_flags(&base, "2"));
    let out = cli(home.path(), &args);
    assert_success(&out, "page limit");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2, "--page-limit 2 must not fetch page 3");
    let body = combined(&out);
    assert!(
        body.contains("voice-1") && body.contains("voice-2"),
        "{body}"
    );
    assert!(!body.contains("voice-3"), "{body}");
}

#[tokio::test]
async fn agents_list_follows_next_cursor_and_stops_when_has_more_is_false() {
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
            "next_cursor": "stale-cursor"
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["agents", "list", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "agents list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(
        requests.len(),
        2,
        "has_more=false must stop even if a cursor is present"
    );
    let body = combined(&out);
    assert!(
        body.contains("agent-first-page") && body.contains("agent-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn conversations_list_follows_next_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/conversations"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "conversations": [{"conversation_id": "conv-first-page"}],
            "has_more": true,
            "next_cursor": "conv-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/conversations"))
        .and(query_param("cursor", "conv-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "conversations": [{"conversation_id": "conv-second-page"}],
            "has_more": false,
            "next_cursor": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["agents", "conversations", "list", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "conversations list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("conv-first-page") && body.contains("conv-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn history_list_uses_last_history_item_id() {
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
    let base = server.uri();
    let mut args = vec!["history", "list", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "history list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("history-first-page") && body.contains("history-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn dubbing_list_follows_next_cursor_without_has_more() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/dubbing"))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "dubs": [{"dubbing_id": "dub-first-page"}],
            "next_cursor": "dub-page-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/dubbing"))
        .and(query_param("cursor", "dub-page-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "dubs": [{"dubbing_id": "dub-second-page"}],
            "next_cursor": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["dubbing", "list", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "dubbing list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("dub-first-page") && body.contains("dub-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn batch_calls_follow_next_doc() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/batch-calling/workspace"))
        .and(query_param_is_missing("last_doc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "batch_calls": [{"id": "batch-first-page"}],
            "has_more": true,
            "next_doc": "doc-two"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/batch-calling/workspace"))
        .and(query_param("last_doc", "doc-two"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "batch_calls": [{"id": "batch-second-page"}],
            "has_more": false,
            "next_doc": null
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["agents", "batch-calls", "list"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "batch-calls list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("batch-first-page") && body.contains("batch-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn shared_voices_increment_page_from_zero() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/shared-voices"))
        .and(query_param_is_missing("page"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "shared-first-page"}],
            "has_more": true
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/shared-voices"))
        .and(query_param("page", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "voices": [{"voice_id": "shared-second-page"}],
            "has_more": false
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["voices", "get_shared", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "voices get_shared");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("shared-first-page") && body.contains("shared-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn audio_isolation_increments_page_from_one() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/audio-isolation/history"))
        .and(query_param_is_missing("page"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "iso-first-page"}],
            "has_more": true
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/audio-isolation/history"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "items": [{"id": "iso-second-page"}],
            "has_more": false
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["audio-isolation", "list", "--page-size", "1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "audio-isolation list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2);
    let body = combined(&out);
    assert!(
        body.contains("iso-first-page") && body.contains("iso-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn deployments_follow_meta_total() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/agents/agent-1/deployments"))
        .and(query_param_is_missing("page"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [{"id": "dep-first-page"}],
            "meta": {"total": 2, "page": 1, "page_size": 1}
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/convai/agents/agent-1/deployments"))
        .and(query_param("page", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": [{"id": "dep-second-page"}],
            "meta": {"total": 2, "page": 2, "page_size": 1}
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["agents", "deployments", "list", "--agent-id", "agent-1"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "deployments list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(
        requests.len(),
        2,
        "page 2 covers meta.total, so page 3 must not be fetched"
    );
    let body = combined(&out);
    assert!(
        body.contains("dep-first-page") && body.contains("dep-second-page"),
        "{body}"
    );
}

#[tokio::test]
async fn orders_advance_offset_by_the_page_length() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/productions/orders"))
        .and(query_param_is_missing("offset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "orders": [
                {"order_id": "order-1", "name": "one", "state": "open"},
                {"order_id": "order-2", "name": "two", "state": "open"}
            ]
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/productions/orders"))
        .and(query_param("offset", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "orders": [
                {"order_id": "order-3", "name": "three", "state": "open"}
            ]
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let base = server.uri();
    let mut args = vec!["productions", "orders", "list", "--page-size", "2"];
    args.extend(page_flags(&base, "5"));
    let out = cli(home.path(), &args);
    assert_success(&out, "orders list");
    let requests = server.received_requests().await.expect("requests");
    assert_eq!(requests.len(), 2, "a short page ends offset pagination");
    let body = combined(&out);
    assert!(
        body.contains("order-1") && body.contains("order-2") && body.contains("order-3"),
        "{body}"
    );
}
