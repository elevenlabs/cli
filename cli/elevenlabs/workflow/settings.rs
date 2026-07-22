//! User-level settings stored in `~/.elevenlabs/config.json` — currently
//! just data residency. Ports the residency half of v0's
//! `src/shared/config.ts`.
//!
//! API-key storage is deliberately NOT handled here: v1 delegates
//! credentials to the framework's keyring/env (`ELEVENLABS_API_KEY`), so
//! this file only ever holds non-sensitive settings.

#![allow(dead_code)]

use std::path::PathBuf;

use fern_cli_sdk::error::CliError;
use serde_json::Value;

/// Accepted residency values (matches v0's `LOCATIONS`).
pub const RESIDENCY_VALUES: &[&str] =
    &["us", "global", "eu-residency", "in-residency", "sg-residency"];

pub const DEFAULT_RESIDENCY: &str = "global";

fn config_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".elevenlabs"))
}

fn config_file() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("config.json"))
}

/// Read the configured residency, defaulting to `global` on any problem
/// (missing file, unreadable, malformed) — matching v0's lenient default.
pub fn read_residency() -> String {
    let Some(path) = config_file() else {
        return DEFAULT_RESIDENCY.to_string();
    };
    let Ok(data) = std::fs::read_to_string(&path) else {
        return DEFAULT_RESIDENCY.to_string();
    };
    serde_json::from_str::<Value>(&data)
        .ok()
        .and_then(|v| {
            v.get("residency")
                .and_then(Value::as_str)
                .map(String::from)
        })
        .unwrap_or_else(|| DEFAULT_RESIDENCY.to_string())
}

/// Persist the residency, preserving any other keys and never writing an
/// API key into the config file (mirrors v0's `saveConfig`).
pub fn write_residency(residency: &str) -> Result<(), CliError> {
    let dir = config_dir()
        .ok_or_else(|| CliError::Other(anyhow::anyhow!("Could not determine home directory")))?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not create {}: {e}", dir.display())))?;
    let path = dir.join("config.json");

    let mut obj = std::fs::read_to_string(&path)
        .ok()
        .and_then(|d| serde_json::from_str::<Value>(&d).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default();
    obj.insert("residency".to_string(), Value::String(residency.to_string()));
    obj.remove("api_key");

    let rendered = super::project::to_pretty_string(&Value::Object(obj))?;
    std::fs::write(&path, rendered)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not write {}: {e}", path.display())))
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
