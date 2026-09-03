//! Thin API layer for the workflow commands — ports v0's
//! `src/shared/elevenlabs-api.ts`.
//!
//! All agent/tool/test **config** traffic goes through raw JSON
//! (`serde_json::Value`) via the CLI's authenticated executor, so configs
//! round-trip losslessly (the generated typed models drop unmodeled
//! fields — see the migration plan). Auth, base URL, retries, and TLS are
//! inherited from the CLI automatically.

#![allow(dead_code)]

use elevenlabs_sdk::RequestOptions;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use reqwest::Method;
use serde_json::{json, Value};

/// Upper bound on pages any list helper will fetch. A hostile or buggy API
/// that keeps returning `has_more` with the same cursor would otherwise pin the
/// CLI in an infinite request loop with unbounded memory growth. The framework
/// applies the same idea to its own `--page-all` (default 10 pages); these
/// helpers need their own limit because they don't go through it.
const MAX_PAGES: usize = 100;

/// Decide whether to fetch another page, and with which cursor.
///
/// Split out from the list helpers so the termination guarantees are testable
/// without an HTTP server: a missing/false `has_more`, an absent `next_cursor`,
/// or a cursor we have already followed all stop the loop.
fn advance_cursor(resp: &Value, seen: &mut std::collections::HashSet<String>) -> Option<String> {
    if !resp
        .get("has_more")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return None;
    }
    let next = resp.get("next_cursor").and_then(Value::as_str)?;
    // A cursor we have already followed means the API is looping.
    if !seen.insert(next.to_string()) {
        return None;
    }
    Some(next.to_string())
}

/// Tag the outgoing `User-Agent` with the command that is running, for the
/// lifetime of one CLI invocation.
///
/// Why the User-Agent and not a header: `RequestOptions.additional_headers`
/// never reaches the wire. `HttpClient::send_request` only calls
/// `apply_custom_headers` on its non-executor branch, and this CLI always uses
/// the executor — which is also why the `X-Source` below is silently dropped.
/// The `User-Agent` is the one identifier that does arrive, and `HttpConfig`
/// reads its suffix env var at client-build time, so setting it here is picked
/// up by every request the command makes.
///
/// The tag is **always a static string** chosen by the caller. It must never be
/// derived from argv: positional tokens carry user data (`agents test my-agent`
/// would put the agent's name in the User-Agent and from there into request
/// logs).
///
/// Appends rather than replaces, so a consumer's own `ELEVENLABS_VIA` survives.
/// Restores the previous value on drop, so one command cannot leak its tag into
/// another (`agents push` shells out to nothing today, but `--dry-run` paths and
/// tests share a process).
pub struct CommandScope {
    key: String,
    previous: Option<String>,
}

/// Env var the framework reads for the User-Agent suffix. The segment tracks
/// the configured flag name (`userAgentSuffixFlag: via` → `_VIA`), so a rename
/// upstream does not silently detach this.
fn suffix_env_key() -> String {
    format!(
        "ELEVENLABS{}",
        fern_cli_sdk::user_agent::suffix_env_segment()
    )
}

pub fn command_scope(command: &'static str) -> CommandScope {
    let key = suffix_env_key();
    let previous = std::env::var(&key).ok();
    let tag = format!("cmd/{command}");
    let combined = match previous.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(existing) => format!("{existing} {tag}"),
        None => tag,
    };
    // Single-threaded: one command is dispatched per process, and the request
    // futures run on a current-thread runtime via `sdk::block_on`.
    std::env::set_var(&key, combined);
    CommandScope { key, previous }
}

impl Drop for CommandScope {
    fn drop(&mut self) {
        match &self.previous {
            Some(v) => std::env::set_var(&self.key, v),
            None => std::env::remove_var(&self.key),
        }
    }
}

/// `X-Source` tag v0 attached to every request.
const X_SOURCE: &str = "agents-cli";

fn request_options() -> Option<RequestOptions> {
    let mut opts = RequestOptions::new();
    opts.additional_headers
        .insert("X-Source".to_string(), X_SOURCE.to_string());
    // The generated `<resource> <method>` commands get this header from the
    // framework's global-parameter injection; hand-written commands go
    // through the SDK executor instead, which that injection does not
    // reach, so add it here. Already sanitised and percent-encoded.
    if let Some(encoded) = super::intent::resolved_encoded() {
        opts.additional_headers
            .insert(super::intent::INTENT_HEADER.to_string(), encoded);
    }
    Some(opts)
}

/// Pull a human-readable message out of the API's error body.
///
/// ElevenLabs returns several shapes: `{"detail": "..."}`, `{"detail":
/// {"message": ..., "status": ...}}`, FastAPI's `{"detail": [{"msg": ...}]}`,
/// and bare `{"message": ..., "status": ...}`. Fall back to the whole body so
/// an unrecognized shape is still shown rather than swallowed.
fn api_error_message(body: &Value) -> String {
    let detail = body.get("detail");
    if let Some(s) = detail.and_then(Value::as_str) {
        return s.to_string();
    }
    if let Some(msg) = detail
        .and_then(|d| d.get("message"))
        .and_then(Value::as_str)
        .or_else(|| body.get("message").and_then(Value::as_str))
    {
        return msg.to_string();
    }
    if let Some(msgs) = detail.and_then(Value::as_array) {
        let joined: Vec<String> = msgs
            .iter()
            .filter_map(|m| m.get("msg").and_then(Value::as_str).map(String::from))
            .collect();
        if !joined.is_empty() {
            return joined.join("; ");
        }
    }
    body.to_string()
}

/// Perform a raw JSON request through the CLI's authenticated executor,
/// surfacing non-2xx responses as errors.
///
/// Body and response are untyped [`Value`], so agent/tool/test configs
/// round-trip losslessly. Paths are relative (no leading slash), matching
/// the generated SDK's convention.
///
/// The status check is ours on purpose. The generated SDK's `execute_request`
/// does not reliably reject error responses: `parse_response` only fails when
/// the body is *empty*, and the executor-backed path this CLI uses skips
/// `execute_with_retries`, which is the only other place status is inspected.
/// So a non-2xx with a JSON body deserializes straight into `Ok`. Left alone,
/// a 401 surfaced as "No agents found in your ElevenLabs workspace" with exit
/// code 0 — a wrong answer reported as success. `execute_request_raw` hands
/// back the status alongside the body, so we can judge it here.
pub(super) fn raw_request(
    ctx: &AppContext,
    method: Method,
    path: &str,
    body: Option<Value>,
    query: Option<Vec<(String, String)>>,
) -> Result<Value, CliError> {
    let client = crate::sdk::client(ctx);
    let raw = crate::sdk::block_on(client.agents.http_client.execute_request_raw::<Value>(
        method,
        path,
        body,
        query,
        request_options(),
    ))?;
    if raw.status_code >= 400 {
        let message = api_error_message(&raw.body);
        return Err(CliError::Api {
            code: raw.status_code,
            message,
            reason: format!("http_{}", raw.status_code),
            // The body goes in `details` rather than being folded into
            // `message`: anything `api_error_message` could not summarise still
            // belongs in the JSON envelope, and stringifying it into the
            // message would nest an escaped JSON document inside a JSON field.
            details: Some(raw.body),
            // No advice to add — the framework supplies its own for the cases
            // where it has some (credential source on a 401, for instance).
            help: None,
        });
    }
    Ok(raw.body)
}

// ── Config cleaning ─────────────────────────────────────────────────

/// Remove the deprecated `agent.prompt.tools` field when `tool_ids`
/// (or `toolIds`) is present — the API returns both but accepts only one.
/// Ports v0's `cleanConversationConfigForApi`.
pub fn clean_conversation_config(conversation_config: &mut Value) {
    if let Some(agent) = conversation_config
        .get_mut("agent")
        .and_then(Value::as_object_mut)
    {
        if let Some(prompt) = agent.get_mut("prompt").and_then(Value::as_object_mut) {
            if prompt.contains_key("tool_ids") || prompt.contains_key("toolIds") {
                prompt.remove("tools");
            }
        }
    }
}

/// Build the create/update request body from an on-disk agent config,
/// sending the fields verbatim (raw wire JSON).
fn build_agent_body(config: &Value, version_description: Option<&str>) -> Value {
    let mut body = serde_json::Map::new();

    if let Some(name) = config.get("name") {
        body.insert("name".to_string(), name.clone());
    }

    // conversation_config is always sent (defaults to {}), cleaned.
    let mut cc = config
        .get("conversation_config")
        .cloned()
        .unwrap_or_else(|| json!({}));
    clean_conversation_config(&mut cc);
    body.insert("conversation_config".to_string(), cc);

    for key in ["platform_settings", "workflow", "tags"] {
        if let Some(v) = config.get(key) {
            body.insert(key.to_string(), v.clone());
        }
    }

    if let Some(desc) = version_description {
        body.insert("version_description".to_string(), json!(desc));
    }

    Value::Object(body)
}

// ── Agent API ───────────────────────────────────────────────────────

/// Result of an agent update — mirrors the fields v0 threads back into
/// `agents.json`.
pub struct UpdateResult {
    pub agent_id: String,
    pub version_id: Option<String>,
    pub branch_id: Option<String>,
}

/// Create a new agent (raw JSON). Returns the new `agent_id`.
pub fn create_agent(ctx: &AppContext, config: &Value) -> Result<String, CliError> {
    let body = build_agent_body(config, None);
    let resp = raw_request(
        ctx,
        Method::POST,
        "v1/convai/agents/create",
        Some(body),
        None,
    )?;
    resp.get("agent_id")
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| {
            CliError::Other(anyhow::anyhow!(
                "Agent create response did not contain an agent_id: {resp}"
            ))
        })
}

/// Update an existing agent (raw JSON), optionally targeting a branch.
pub fn update_agent(
    ctx: &AppContext,
    agent_id: &str,
    config: &Value,
    version_description: Option<&str>,
    branch_id: Option<&str>,
) -> Result<UpdateResult, CliError> {
    let body = build_agent_body(config, version_description);
    let query = branch_id.map(|b| vec![("branch_id".to_string(), b.to_string())]);
    let resp = raw_request(
        ctx,
        Method::PATCH,
        &format!("v1/convai/agents/{agent_id}"),
        Some(body),
        query,
    )?;
    Ok(UpdateResult {
        agent_id: resp
            .get("agent_id")
            .and_then(Value::as_str)
            .unwrap_or(agent_id)
            .to_string(),
        version_id: resp
            .get("version_id")
            .and_then(Value::as_str)
            .map(String::from),
        branch_id: resp
            .get("branch_id")
            .and_then(Value::as_str)
            .map(String::from),
    })
}

/// Fetch an agent's full config (raw JSON), optionally from a branch.
pub fn get_agent(
    ctx: &AppContext,
    agent_id: &str,
    branch_id: Option<&str>,
) -> Result<Value, CliError> {
    let query = branch_id.map(|b| vec![("branch_id".to_string(), b.to_string())]);
    raw_request(
        ctx,
        Method::GET,
        &format!("v1/convai/agents/{agent_id}"),
        None,
        query,
    )
}

/// List every agent's metadata, paginating to completion.
pub fn list_agents(ctx: &AppContext, search: Option<&str>) -> Result<Vec<Value>, CliError> {
    let mut all = Vec::new();
    let mut cursor: Option<String> = None;
    let mut seen_cursors: std::collections::HashSet<String> = std::collections::HashSet::new();
    for page in 0..MAX_PAGES {
        let mut query = vec![("page_size".to_string(), "100".to_string())];
        if let Some(c) = &cursor {
            query.push(("cursor".to_string(), c.clone()));
        }
        if let Some(s) = search {
            query.push(("search".to_string(), s.to_string()));
        }
        let resp = raw_request(ctx, Method::GET, "v1/convai/agents", None, Some(query))?;
        if let Some(agents) = resp.get("agents").and_then(Value::as_array) {
            all.extend(agents.iter().cloned());
        }
        match advance_cursor(&resp, &mut seen_cursors) {
            Some(next) => cursor = Some(next),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            eprintln!("Warning: stopped after {MAX_PAGES} pages; results may be incomplete.");
        }
    }
    Ok(all)
}

/// Delete an agent. Uses the typed SDK method (empty response body).
pub fn delete_agent(ctx: &AppContext, agent_id: &str) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    crate::sdk::block_on(client.agents.delete(agent_id, request_options()))
}

/// List an agent's branches (raw JSON summaries).
pub fn list_branches(
    ctx: &AppContext,
    agent_id: &str,
    include_archived: bool,
) -> Result<Vec<Value>, CliError> {
    let query = vec![("include_archived".to_string(), include_archived.to_string())];
    let resp = raw_request(
        ctx,
        Method::GET,
        &format!("v1/convai/agents/{agent_id}/branches"),
        None,
        Some(query),
    )?;
    Ok(resp
        .get("results")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

/// Resolve a branch name or `agtbrch_`-prefixed id to a branch id.
/// Ports v0's `resolveBranchId`.
pub fn resolve_branch_id(
    ctx: &AppContext,
    agent_id: &str,
    branch_name_or_id: &str,
) -> Result<String, CliError> {
    if branch_name_or_id.starts_with("agtbrch_") {
        return Ok(branch_name_or_id.to_string());
    }
    let branches = list_branches(ctx, agent_id, true)?;
    for branch in &branches {
        if branch.get("name").and_then(Value::as_str) == Some(branch_name_or_id) {
            if let Some(id) = branch.get("id").and_then(Value::as_str) {
                return Ok(id.to_string());
            }
        }
    }
    Err(CliError::Validation(format!(
        "Branch '{branch_name_or_id}' not found for agent '{agent_id}'. \
         Use 'elevenlabs agents branches list --agent {agent_id}' to see available branches."
    )))
}

// ── Tool API ────────────────────────────────────────────────────────

/// Create a tool (raw JSON). Returns the full response, which contains
/// the new `id` and the persisted `tool_config`.
pub fn create_tool(ctx: &AppContext, config: &Value) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::POST,
        "v1/convai/tools",
        Some(json!({ "tool_config": config })),
        None,
    )
}

/// Update a tool (raw JSON). Returns the full response (contains
/// `tool_config` for verification).
pub fn update_tool(ctx: &AppContext, tool_id: &str, config: &Value) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::PATCH,
        &format!("v1/convai/tools/{tool_id}"),
        Some(json!({ "tool_config": config })),
        None,
    )
}

/// Fetch a tool (raw JSON). The response wraps the config in `tool_config`.
pub fn get_tool(ctx: &AppContext, tool_id: &str) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::GET,
        &format!("v1/convai/tools/{tool_id}"),
        None,
        None,
    )
}

/// List every tool (raw JSON summaries, each wrapping `tool_config`).
pub fn list_tools(ctx: &AppContext) -> Result<Vec<Value>, CliError> {
    let resp = raw_request(ctx, Method::GET, "v1/convai/tools", None, None)?;
    Ok(resp
        .get("tools")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

/// Delete a tool. The endpoint returns a JSON body, so the raw path works.
pub fn delete_tool(ctx: &AppContext, tool_id: &str) -> Result<(), CliError> {
    raw_request(
        ctx,
        Method::DELETE,
        &format!("v1/convai/tools/{tool_id}"),
        None,
        None,
    )?;
    Ok(())
}

// ── Test API ────────────────────────────────────────────────────────

/// Create a test (raw JSON). Returns the full response, containing the new `id`.
pub fn create_test(ctx: &AppContext, config: &Value) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::POST,
        "v1/convai/agent-testing/create",
        Some(config.clone()),
        None,
    )
}

/// Update a test (raw JSON).
pub fn update_test(ctx: &AppContext, test_id: &str, config: &Value) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::PUT,
        &format!("v1/convai/agent-testing/{test_id}"),
        Some(config.clone()),
        None,
    )
}

/// Fetch a test's full config (raw JSON).
pub fn get_test(ctx: &AppContext, test_id: &str) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::GET,
        &format!("v1/convai/agent-testing/{test_id}"),
        None,
        None,
    )
}

/// List every test, paginating to completion.
pub fn list_tests(ctx: &AppContext) -> Result<Vec<Value>, CliError> {
    let mut all = Vec::new();
    let mut cursor: Option<String> = None;
    let mut seen_cursors: std::collections::HashSet<String> = std::collections::HashSet::new();
    for page in 0..MAX_PAGES {
        let mut query = vec![("page_size".to_string(), "100".to_string())];
        if let Some(c) = &cursor {
            query.push(("cursor".to_string(), c.clone()));
        }
        let resp = raw_request(
            ctx,
            Method::GET,
            "v1/convai/agent-testing",
            None,
            Some(query),
        )?;
        if let Some(tests) = resp.get("tests").and_then(Value::as_array) {
            all.extend(tests.iter().cloned());
        }
        match advance_cursor(&resp, &mut seen_cursors) {
            Some(next) => cursor = Some(next),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            eprintln!("Warning: stopped after {MAX_PAGES} pages; results may be incomplete.");
        }
    }
    Ok(all)
}

/// Delete a test.
pub fn delete_test(ctx: &AppContext, test_id: &str) -> Result<(), CliError> {
    raw_request(
        ctx,
        Method::DELETE,
        &format!("v1/convai/agent-testing/{test_id}"),
        None,
        None,
    )?;
    Ok(())
}

// ── Test-running API (for `agents test`) ────────────────────────────

/// Run the given tests on an agent. Returns the invocation (raw JSON).
pub fn run_tests_on_agent(
    ctx: &AppContext,
    agent_id: &str,
    test_ids: &[String],
    agent_config_override: Option<&Value>,
) -> Result<Value, CliError> {
    let tests: Vec<Value> = test_ids.iter().map(|id| json!({ "test_id": id })).collect();
    let mut body = serde_json::Map::new();
    body.insert("tests".to_string(), Value::Array(tests));
    if let Some(over) = agent_config_override {
        body.insert("agent_config_override".to_string(), over.clone());
    }
    raw_request(
        ctx,
        Method::POST,
        &format!("v1/convai/agents/{agent_id}/run-tests"),
        Some(Value::Object(body)),
        None,
    )
}

/// Poll a single test invocation's current state (raw JSON).
pub fn get_test_invocation(ctx: &AppContext, invocation_id: &str) -> Result<Value, CliError> {
    raw_request(
        ctx,
        Method::GET,
        &format!("v1/convai/test-invocations/{invocation_id}"),
        None,
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // `command_scope` mutates process-wide env, so these must not interleave.
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_env<T>(body: impl FnOnce(&str) -> T) -> T {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let key = suffix_env_key();
        let saved = std::env::var(&key).ok();
        std::env::remove_var(&key);
        let out = body(&key);
        match saved {
            Some(v) => std::env::set_var(&key, v),
            None => std::env::remove_var(&key),
        }
        out
    }

    #[test]
    fn scope_tags_the_user_agent_suffix_and_restores_on_drop() {
        with_env(|key| {
            {
                let _scope = command_scope("agents.push");
                assert_eq!(std::env::var(key).as_deref(), Ok("cmd/agents.push"));
            }
            // Restored to absent, so one command cannot leak its tag into the next.
            assert!(std::env::var(key).is_err());
        });
    }

    #[test]
    fn scope_appends_rather_than_clobbering_a_consumer_token() {
        with_env(|key| {
            std::env::set_var(key, "partner-app/3.1");
            {
                let _scope = command_scope("tools.pull");
                assert_eq!(
                    std::env::var(key).as_deref(),
                    Ok("partner-app/3.1 cmd/tools.pull")
                );
            }
            assert_eq!(std::env::var(key).as_deref(), Ok("partner-app/3.1"));
        });
    }

    #[test]
    fn scope_ignores_a_blank_existing_value() {
        with_env(|key| {
            std::env::set_var(key, "   ");
            let _scope = command_scope("tests.add");
            assert_eq!(std::env::var(key).as_deref(), Ok("cmd/tests.add"));
        });
    }

    #[test]
    fn suffix_env_key_tracks_the_configured_flag_name() {
        // Guards against the flag being renamed upstream (userAgentSuffixFlag)
        // without this following it — the tag would silently stop being read.
        let key = suffix_env_key();
        assert!(key.starts_with("ELEVENLABS"), "unexpected key: {key}");
        assert_eq!(
            key,
            format!(
                "ELEVENLABS{}",
                fern_cli_sdk::user_agent::suffix_env_segment()
            )
        );
    }

    #[test]
    fn api_error_message_handles_every_shape_the_api_returns() {
        // {"detail": "..."}
        assert_eq!(
            api_error_message(&json!({"detail": "Invalid API key"})),
            "Invalid API key"
        );
        // {"detail": {"message": ..., "status": ...}}
        assert_eq!(
            api_error_message(&json!({"detail": {"message": "bad model", "status": "invalid"}})),
            "bad model"
        );
        // bare {"message": ..., "status": ...}
        assert_eq!(
            api_error_message(&json!({"message": "Internal Server error", "status": "ise"})),
            "Internal Server error"
        );
        // FastAPI validation list
        assert_eq!(
            api_error_message(&json!({"detail": [{"msg": "field required"}, {"msg": "bad type"}]})),
            "field required; bad type"
        );
    }

    #[test]
    fn api_error_message_falls_back_to_the_whole_body() {
        // An unrecognized shape must still be shown, not swallowed.
        let body = json!({"weird": {"nested": true}});
        assert_eq!(api_error_message(&body), body.to_string());
    }

    #[test]
    fn clean_removes_tools_when_tool_ids_present() {
        let mut cc = json!({
            "agent": { "prompt": { "tool_ids": ["t1"], "tools": [{"name":"x"}] } }
        });
        clean_conversation_config(&mut cc);
        assert!(cc["agent"]["prompt"].get("tools").is_none());
        assert!(cc["agent"]["prompt"].get("tool_ids").is_some());
    }

    #[test]
    fn clean_keeps_tools_when_no_tool_ids() {
        let mut cc = json!({ "agent": { "prompt": { "tools": [{"name":"x"}] } } });
        clean_conversation_config(&mut cc);
        assert!(cc["agent"]["prompt"].get("tools").is_some());
    }

    #[test]
    fn build_body_defaults_conversation_config() {
        let body = build_agent_body(&json!({ "name": "A" }), None);
        assert_eq!(body["name"], json!("A"));
        assert_eq!(body["conversation_config"], json!({}));
    }

    fn seen() -> std::collections::HashSet<String> {
        std::collections::HashSet::new()
    }

    #[test]
    fn stops_when_has_more_is_absent_or_false() {
        assert_eq!(advance_cursor(&json!({}), &mut seen()), None);
        assert_eq!(
            advance_cursor(&json!({"has_more": false}), &mut seen()),
            None
        );
    }

    #[test]
    fn stops_when_has_more_is_a_non_bool() {
        // A hostile API cannot keep the loop alive with a truthy-looking value.
        assert_eq!(
            advance_cursor(&json!({"has_more": "yes"}), &mut seen()),
            None
        );
        assert_eq!(advance_cursor(&json!({"has_more": 1}), &mut seen()), None);
    }

    #[test]
    fn stops_when_next_cursor_is_missing_despite_has_more() {
        let resp = json!({"has_more": true});
        assert_eq!(advance_cursor(&resp, &mut seen()), None);
    }

    #[test]
    fn follows_a_fresh_cursor() {
        let mut s = seen();
        let resp = json!({"has_more": true, "next_cursor": "abc"});
        assert_eq!(advance_cursor(&resp, &mut s), Some("abc".to_string()));
        assert!(s.contains("abc"));
    }

    #[test]
    fn stops_on_a_repeated_cursor() {
        let mut s = seen();
        let resp = json!({"has_more": true, "next_cursor": "loop"});
        assert_eq!(advance_cursor(&resp, &mut s), Some("loop".to_string()));
        // Same cursor again: the API is looping, so refuse to follow it.
        assert_eq!(advance_cursor(&resp, &mut s), None);
    }

    #[test]
    fn page_cap_is_bounded() {
        assert!(MAX_PAGES > 0 && MAX_PAGES <= 1000);
    }
}
