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

/// `X-Source` tag v0 attached to every request.
const X_SOURCE: &str = "agents-cli";

fn request_options() -> Option<RequestOptions> {
    let mut opts = RequestOptions::new();
    opts.additional_headers
        .insert("X-Source".to_string(), X_SOURCE.to_string());
    Some(opts)
}

/// Perform a raw JSON request through the CLI's authenticated executor.
///
/// Body and response are untyped [`Value`], so agent/tool/test configs
/// round-trip losslessly. Paths are relative (no leading slash), matching
/// the generated SDK's convention.
fn raw_request(
    ctx: &AppContext,
    method: Method,
    path: &str,
    body: Option<Value>,
    query: Option<Vec<(String, String)>>,
) -> Result<Value, CliError> {
    let client = crate::sdk::client(ctx);
    crate::sdk::block_on(
        client
            .agents
            .http_client
            .execute_request::<Value>(method, path, body, query, request_options()),
    )
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
    loop {
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
        if !resp.get("has_more").and_then(Value::as_bool).unwrap_or(false) {
            break;
        }
        cursor = resp
            .get("next_cursor")
            .and_then(Value::as_str)
            .map(String::from);
        if cursor.is_none() {
            break;
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
}
