//! User-level settings stored in `~/.elevenlabs/config.json` — data
//! residency plus the `say` command's defaults. Ports the residency half
//! of v0's `src/shared/config.ts`.
//!
//! API-key storage is deliberately NOT handled here: v1 delegates
//! credentials to the framework's keyring/env (`ELEVENLABS_API_KEY`), so
//! this file only ever holds non-sensitive settings.

#![allow(dead_code)]

use std::path::PathBuf;

use fern_cli_sdk::error::CliError;
use serde_json::Value;

/// Accepted residency values (matches v0's `LOCATIONS`).
pub const RESIDENCY_VALUES: &[&str] = &[
    "us",
    "global",
    "eu-residency",
    "in-residency",
    "sg-residency",
];

pub const DEFAULT_RESIDENCY: &str = "global";

fn config_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".elevenlabs"))
}

fn config_file() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.json"))
}

// ── Generic get/set ─────────────────────────────────────────────────

/// Read a nested string setting, e.g. `["say", "voice_id"]`.
///
/// Returns `None` on any problem — missing file, unreadable, malformed,
/// missing key, or a non-string value. Callers supply their own default,
/// matching v0's lenient posture: a hand-mangled config degrades to
/// defaults rather than breaking every command.
pub fn read_setting(path: &[&str]) -> Option<String> {
    let file = config_file()?;
    let data = std::fs::read_to_string(&file).ok()?;
    let root: Value = serde_json::from_str(&data).ok()?;
    let mut cursor = &root;
    for key in path {
        cursor = cursor.get(key)?;
    }
    cursor.as_str().map(String::from)
}

/// Persist a nested string setting, creating intermediate objects as
/// needed. `None` removes the key.
pub fn write_setting(path: &[&str], value: Option<&str>) -> Result<(), CliError> {
    let dir = config_dir()
        .ok_or_else(|| CliError::Other(anyhow::anyhow!("Could not determine home directory")))?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not create {}: {e}", dir.display())))?;
    // Owner-only, matching v0's posture for ~/.elevenlabs. Nothing sensitive
    // lives here today (api_key is stripped below), but the directory is the
    // natural home for anything that later does.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
    }
    let file = dir.join("config.json");

    let existing = std::fs::read_to_string(&file)
        .ok()
        .and_then(|d| serde_json::from_str::<Value>(&d).ok());

    super::project::write_json(&file, &merge_setting(existing, path, value))
}

/// Apply one setting to the existing config, preserving unrelated keys and
/// dropping any `api_key` — credentials belong in the framework's keyring,
/// never in this file. Split out from [`write_setting`] so the precedence
/// and stripping rules are testable without touching a real home directory.
fn merge_setting(existing: Option<Value>, path: &[&str], value: Option<&str>) -> Value {
    let mut root = existing
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();
    root.remove("api_key");
    set_in(&mut root, path, value);
    Value::Object(root)
}

fn set_in(obj: &mut serde_json::Map<String, Value>, path: &[&str], value: Option<&str>) {
    match path {
        [] => {}
        [leaf] => match value {
            Some(v) => {
                obj.insert((*leaf).to_string(), Value::String(v.to_string()));
            }
            None => {
                obj.remove(*leaf);
            }
        },
        [head, rest @ ..] => {
            let entry = obj
                .entry((*head).to_string())
                .or_insert_with(|| Value::Object(Default::default()));
            // A non-object here means the file was hand-edited to something
            // else; replace it rather than silently dropping the write.
            if !entry.is_object() {
                *entry = Value::Object(Default::default());
            }
            let nested = entry
                .as_object_mut()
                .expect("entry was just coerced to an object");
            set_in(nested, rest, value);
            // Unsetting the last key in a section leaves no empty debris.
            if nested.is_empty() {
                obj.remove(*head);
            }
        }
    }
}

// ── Residency ───────────────────────────────────────────────────────

/// Read the configured residency, defaulting to `global` on any problem
/// (missing file, unreadable, malformed) — matching v0's lenient default.
pub fn read_residency() -> String {
    read_setting(&["residency"]).unwrap_or_else(|| DEFAULT_RESIDENCY.to_string())
}

/// Persist the residency, preserving any other keys and never writing an
/// API key into the config file (mirrors v0's `saveConfig`).
pub fn write_residency(residency: &str) -> Result<(), CliError> {
    write_setting(&["residency"], Some(residency))
}

/// Map a residency to its API base URL. Ports v0's `getApiBaseUrl`.
pub fn base_url_for(residency: &str) -> &'static str {
    match residency {
        "eu-residency" => "https://api.eu.residency.elevenlabs.io",
        "in-residency" => "https://api.in.residency.elevenlabs.io",
        "sg-residency" => "https://api.sg.residency.elevenlabs.io",
        "us" => "https://api.us.elevenlabs.io",
        // "global" and anything unrecognized
        _ => "https://api.elevenlabs.io",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn every_region_maps_to_its_host() {
        // Ports v0's residency.test.ts.
        assert_eq!(base_url_for("global"), "https://api.elevenlabs.io");
        assert_eq!(base_url_for("us"), "https://api.us.elevenlabs.io");
        assert_eq!(
            base_url_for("eu-residency"),
            "https://api.eu.residency.elevenlabs.io"
        );
        assert_eq!(
            base_url_for("in-residency"),
            "https://api.in.residency.elevenlabs.io"
        );
        assert_eq!(
            base_url_for("sg-residency"),
            "https://api.sg.residency.elevenlabs.io"
        );
    }

    #[test]
    fn an_unrecognized_region_falls_back_to_the_default_host() {
        assert_eq!(base_url_for("mars"), base_url_for(DEFAULT_RESIDENCY));
    }

    #[test]
    fn the_default_region_is_offered_as_a_choice() {
        assert!(RESIDENCY_VALUES.contains(&DEFAULT_RESIDENCY));
        // Every advertised region must map somewhere.
        for region in RESIDENCY_VALUES {
            assert!(base_url_for(region).starts_with("https://"));
        }
    }

    #[test]
    fn merging_preserves_unrelated_keys() {
        let existing = json!({ "residency": "us", "other": 1 });
        let merged = merge_setting(Some(existing), &["residency"], Some("eu-residency"));
        assert_eq!(merged["residency"], json!("eu-residency"));
        assert_eq!(merged["other"], json!(1));
    }

    #[test]
    fn merging_never_persists_an_api_key() {
        let existing = json!({ "api_key": "sk-secret", "residency": "us" });
        let merged = merge_setting(Some(existing), &["residency"], Some("global"));
        assert!(
            merged.get("api_key").is_none(),
            "api_key must never be written to the config file"
        );
    }

    #[test]
    fn merging_handles_a_missing_or_malformed_config() {
        assert_eq!(
            merge_setting(None, &["residency"], Some("us"))["residency"],
            json!("us")
        );
        // A non-object config (e.g. hand-edited to a list) is replaced, not crashed on.
        assert_eq!(
            merge_setting(Some(json!([1, 2])), &["residency"], Some("us"))["residency"],
            json!("us")
        );
    }

    #[test]
    fn a_nested_setting_does_not_disturb_its_siblings() {
        let existing = json!({ "residency": "us", "say": { "model_id": "m" } });
        let merged = merge_setting(Some(existing), &["say", "voice_id"], Some("v"));
        assert_eq!(merged["residency"], json!("us"));
        assert_eq!(merged["say"]["model_id"], json!("m"));
        assert_eq!(merged["say"]["voice_id"], json!("v"));
    }

    #[test]
    fn a_nested_setting_creates_its_section() {
        let merged = merge_setting(None, &["say", "voice_id"], Some("v"));
        assert_eq!(merged["say"]["voice_id"], json!("v"));
    }

    #[test]
    fn a_nested_section_that_is_not_an_object_is_replaced() {
        let existing = json!({ "say": "nonsense" });
        let merged = merge_setting(Some(existing), &["say", "voice_id"], Some("v"));
        assert_eq!(merged["say"]["voice_id"], json!("v"));
    }

    #[test]
    fn unsetting_removes_the_key_and_prunes_an_empty_section() {
        let existing = json!({ "residency": "us", "say": { "voice_id": "v" } });
        let merged = merge_setting(Some(existing), &["say", "voice_id"], None);
        assert_eq!(merged["residency"], json!("us"));
        assert!(
            merged.get("say").is_none(),
            "an emptied section should not linger in the file"
        );
    }

    #[test]
    fn unsetting_keeps_a_section_that_still_has_other_keys() {
        let existing = json!({ "say": { "voice_id": "v", "model_id": "m" } });
        let merged = merge_setting(Some(existing), &["say", "voice_id"], None);
        assert_eq!(merged["say"]["model_id"], json!("m"));
        assert!(merged["say"].get("voice_id").is_none());
    }
}
