//! Push-time verification: read an agent back after a push and warn about
//! any locally-specified field the API did not persist. Ports v0's
//! `src/shared/verify.ts`.
//!
//! Unlike v0, v1 pushes raw wire JSON (no SDK serializer in the path and
//! no snake/camel conversion), so this compares the pushed config against
//! the live config directly. Verification never fails a push — problems
//! are reported as warnings only.

#![allow(dead_code)]

use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Map, Value};

use super::api;

/// Collect the key paths in `expected` that are missing from `actual`.
///
/// Presence-only subset check: objects are traversed recursively, while
/// arrays and scalars are treated as leaves (a present key counts; values
/// are not compared). Extra keys in `actual` are ignored, since the API
/// merges updates and fills defaults. Ports v0's `findMissingPaths`.
pub fn find_missing_paths(expected: &Value, actual: &Value, base_path: &str) -> Vec<String> {
    let Some(expected_obj) = expected.as_object() else {
        return Vec::new();
    };
    let mut missing = Vec::new();
    for (key, expected_value) in expected_obj {
        let path = if base_path.is_empty() {
            key.clone()
        } else {
            format!("{base_path}.{key}")
        };
        match actual.as_object().and_then(|o| o.get(key)) {
            None => missing.push(path),
            Some(actual_value) => {
                if expected_value.is_object() {
                    missing.extend(find_missing_paths(expected_value, actual_value, &path));
                }
            }
        }
    }
    missing
}

fn report_missing_fields(label: &str, missing: &[String], pull_command: &str) {
    if missing.is_empty() {
        return;
    }
    let shown = missing.len().min(10);
    println!(
        "  ⚠ {label}: {} field(s) in the local config were not persisted by the API:",
        missing.len()
    );
    for path in &missing[..shown] {
        println!("      - {path}");
    }
    if missing.len() > shown {
        println!("      ... and {} more", missing.len() - shown);
    }
    println!("    Run '{pull_command}' to inspect the live config.");
}

/// Read an agent back after a push and warn about any locally-specified
/// field the API did not persist. Never returns an error — verification
/// failures are surfaced as warnings.
pub fn verify_agent_push(
    ctx: &AppContext,
    label: &str,
    agent_id: &str,
    pushed_config: &Value,
    branch_id: Option<&str>,
) {
    let live = match api::get_agent(ctx, agent_id, branch_id) {
        Ok(live) => live,
        Err(e) => {
            println!("  ⚠ {label}: could not verify the pushed config: {e}");
            return;
        }
    };

    // Mirror the push payload: conversation_config is always sent (default
    // {}) with the deprecated `tools` removed when `tool_ids` is present,
    // so its absence live is intentional.
    let mut expected = Map::new();
    let mut conversation_config = pushed_config
        .get("conversation_config")
        .cloned()
        .unwrap_or_else(|| json!({}));
    api::clean_conversation_config(&mut conversation_config);
    expected.insert("conversation_config".to_string(), conversation_config);

    if let Some(platform_settings) = pushed_config.get("platform_settings") {
        expected.insert("platform_settings".to_string(), platform_settings.clone());
    }
    if let Some(workflow) = pushed_config.get("workflow") {
        if !workflow.is_null() {
            expected.insert("workflow".to_string(), workflow.clone());
        }
    }

    let missing = find_missing_paths(&Value::Object(expected), &live, "");
    report_missing_fields(label, &missing, "elevenlabs agents pull");
}

/// Verify a pushed tool config against the config the API echoed back in
/// the create/update response (`tool_config`). No extra API call needed.
/// Ports v0's `verifyToolPush`.
pub fn verify_tool_push(label: &str, pushed_config: &Value, response: &Value) {
    match response.get("tool_config") {
        Some(live_config) => {
            let missing = find_missing_paths(pushed_config, live_config, "");
            report_missing_fields(label, &missing, "elevenlabs tools pull");
        }
        None => println!(
            "  ⚠ {label}: could not verify the pushed config (no tool_config in the API response)"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_paths_reports_absent_keys_only() {
        let expected = json!({ "a": { "b": 1, "c": 2 }, "d": 3 });
        let actual = json!({ "a": { "b": 9 }, "e": 5 });
        let mut missing = find_missing_paths(&expected, &actual, "");
        missing.sort();
        assert_eq!(missing, vec!["a.c".to_string(), "d".to_string()]);
    }

    #[test]
    fn arrays_and_scalars_are_leaves() {
        // Present array key counts as present even if contents differ.
        let expected = json!({ "tags": ["x", "y"], "n": 1 });
        let actual = json!({ "tags": [], "n": 2 });
        assert!(find_missing_paths(&expected, &actual, "").is_empty());
    }

    #[test]
    fn nothing_missing_when_actual_superset() {
        let expected = json!({ "a": 1 });
        let actual = json!({ "a": 1, "extra": 2 });
        assert!(find_missing_paths(&expected, &actual, "").is_empty());
    }
}
