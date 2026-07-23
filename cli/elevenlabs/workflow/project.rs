//! Local project model for the "agents-as-code" workflow.
//!
//! Ports v0's `src/shared/utils.ts` plus the `agents.json` / `tools.json`
//! / `tests.json` index-file schemas. Design notes:
//!
//! * Entity **configs** are stored as raw wire JSON (`serde_json::Value`)
//!   and pushed verbatim, so they round-trip losslessly (see the
//!   round-trip rationale in the migration plan — the generated typed
//!   models drop unmodeled fields).
//! * The **index files** map entity name / id / branch metadata to the
//!   on-disk config file path. Their `config` field is a *path*, not an
//!   inline config — matching v0 exactly, so existing projects keep
//!   working.
//! * Files are written pretty-printed with a 4-space indent to match v0's
//!   `JSON.stringify(value, null, 4)` on-disk format.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use fern_cli_sdk::error::CliError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Index-file locations ────────────────────────────────────────────

pub const AGENTS_FILE: &str = "agents.json";
pub const TOOLS_FILE: &str = "tools.json";
pub const TESTS_FILE: &str = "tests.json";

pub const AGENT_CONFIGS_DIR: &str = "agent_configs";
pub const TOOL_CONFIGS_DIR: &str = "tool_configs";
pub const TEST_CONFIGS_DIR: &str = "test_configs";

// ── Index-file schemas ──────────────────────────────────────────────

/// `agents.json` — the agent registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentsConfig {
    #[serde(default)]
    pub agents: Vec<AgentDefinition>,
}

/// One entry in `agents.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefinition {
    /// Path to the agent's config file, relative to the project root.
    pub config: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Per-branch configs, keyed by branch name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches: Option<BTreeMap<String, BranchDefinition>>,
}

/// A branch entry under an [`AgentDefinition`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchDefinition {
    /// Path to this branch's config file, relative to the project root.
    pub config: String,
    pub branch_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// `tools.json` — the tool registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolsConfig {
    #[serde(default)]
    pub tools: Vec<ToolDefinition>,
}

/// One entry in `tools.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// `"webhook"` or `"client"`.
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Path to the tool's config file, relative to the project root.
    pub config: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// `tests.json` — the test registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestsConfig {
    #[serde(default)]
    pub tests: Vec<TestDefinition>,
}

/// One entry in `tests.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDefinition {
    /// Path to the test's config file, relative to the project root.
    pub config: String,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub test_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ── JSON IO ─────────────────────────────────────────────────────────

/// Read and deserialize a JSON file, with v0-style error messages.
pub fn read_json<T>(path: &Path) -> Result<T, CliError>
where
    T: serde::de::DeserializeOwned,
{
    let data = std::fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            CliError::Validation(format!(
                "Configuration file not found at {}",
                path.display()
            ))
        } else {
            CliError::Other(anyhow::anyhow!("Could not read {}: {e}", path.display()))
        }
    })?;
    serde_json::from_str(&data).map_err(|e| {
        CliError::Validation(format!(
            "Invalid JSON in configuration file {}: {e}",
            path.display()
        ))
    })
}

/// Read a config file as an untyped [`Value`] (lossless).
pub fn read_value(path: &Path) -> Result<Value, CliError> {
    read_json::<Value>(path)
}

/// Serialize with a 4-space indent to match v0's on-disk format.
pub fn to_pretty_string<T: Serialize>(value: &T) -> Result<String, CliError> {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    value
        .serialize(&mut ser)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not serialize JSON: {e}")))?;
    String::from_utf8(buf)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Serialized JSON was not valid UTF-8: {e}")))
}

/// Write a value as pretty JSON, creating parent directories as needed.
pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CliError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| {
                CliError::Other(anyhow::anyhow!(
                    "Could not create directory {}: {e}",
                    parent.display()
                ))
            })?;
        }
    }
    let rendered = to_pretty_string(value)?;
    std::fs::write(path, rendered).map_err(|e| {
        CliError::Other(anyhow::anyhow!(
            "Could not write configuration file to {}: {e}",
            path.display()
        ))
    })
}

// ── Filename generation ─────────────────────────────────────────────

/// Sanitize a user-provided entity name into a filesystem-safe stem.
///
/// Ports v0's `sanitizeFilename`. This is the primary defense against
/// path traversal: separators and other unsafe characters are replaced
/// with hyphens, leading dots are neutralized, and the length is capped.
pub fn sanitize_filename(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "unnamed".to_string();
    }

    let starts_with_dot = trimmed.starts_with('.');

    // Replace filesystem-unsafe characters and whitespace with hyphens.
    let mut sanitized: String = trimmed
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            c if c.is_whitespace() => '-',
            c => c,
        })
        .collect();

    // Collapse runs of hyphens into one.
    while sanitized.contains("--") {
        sanitized = sanitized.replace("--", "-");
    }

    // Strip leading/trailing dots and hyphens.
    let stripped = sanitized.trim_matches(|c| c == '.' || c == '-').to_string();
    if stripped.is_empty() {
        return "unnamed".to_string();
    }

    let mut result = if starts_with_dot {
        format!("_{stripped}")
    } else {
        stripped
    };

    // Cap length to avoid absurdly long filenames.
    if result.chars().count() > 100 {
        result = result.chars().take(100).collect();
    }

    result
}

/// Build a collision-free path `<dir>/<sanitized><ext>`, appending
/// `-1`, `-2`, … if files already exist. Ports v0's
/// `generateUniqueFilename`.
pub fn generate_unique_filename(dir: &str, entity_name: &str, ext: &str) -> PathBuf {
    let sanitized = sanitize_filename(entity_name);
    let base = PathBuf::from(dir);

    let candidate = base.join(format!("{sanitized}{ext}"));
    if !candidate.exists() {
        return candidate;
    }

    let mut counter = 1;
    loop {
        let candidate = base.join(format!("{sanitized}-{counter}{ext}"));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

// ── Registry load/save convenience ──────────────────────────────────

pub fn agents_path() -> PathBuf {
    PathBuf::from(AGENTS_FILE)
}
pub fn tools_path() -> PathBuf {
    PathBuf::from(TOOLS_FILE)
}
pub fn tests_path() -> PathBuf {
    PathBuf::from(TESTS_FILE)
}

pub fn load_agents() -> Result<AgentsConfig, CliError> {
    read_json(&agents_path())
}
pub fn save_agents(config: &AgentsConfig) -> Result<(), CliError> {
    write_json(&agents_path(), config)
}

pub fn load_tools() -> Result<ToolsConfig, CliError> {
    read_json(&tools_path())
}
pub fn save_tools(config: &ToolsConfig) -> Result<(), CliError> {
    write_json(&tools_path(), config)
}

pub fn load_tests() -> Result<TestsConfig, CliError> {
    read_json(&tests_path())
}
pub fn save_tests(config: &TestsConfig) -> Result<(), CliError> {
    write_json(&tests_path(), config)
}

// ── Interactive prompt ──────────────────────────────────────────────

/// Ask a `y/N` question on stdin. Ports v0's `promptForConfirmation`:
/// only an explicit `y`/`yes` (case-insensitive) counts as yes, so a
/// non-interactive/empty stdin safely defaults to no.
pub fn prompt_confirm(message: &str) -> Result<bool, CliError> {
    use std::io::Write;
    print!("{message} (y/N): ");
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not read confirmation input: {e}")))?;
    let answer = input.trim().to_lowercase();
    Ok(answer == "y" || answer == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_replaces_separators_and_traversal() {
        // Leading dot → "_" prefix (matches v0); crucially, no path
        // separators survive, so traversal is neutralized either way.
        assert_eq!(sanitize_filename("../../etc/passwd"), "_etc-passwd");
        // Interior dots are kept (harmless — no separators remain).
        assert_eq!(sanitize_filename("foo/../bar"), "foo-..-bar");
        assert_eq!(sanitize_filename("a/b\\c:d"), "a-b-c-d");
        assert_eq!(sanitize_filename("My Agent"), "My-Agent");
        assert_eq!(sanitize_filename("  spaced  out  "), "spaced-out");
    }

    #[test]
    fn sanitize_handles_dots_and_empties() {
        assert_eq!(sanitize_filename(""), "unnamed");
        assert_eq!(sanitize_filename("   "), "unnamed");
        assert_eq!(sanitize_filename("..."), "unnamed");
        assert_eq!(sanitize_filename(".hidden"), "_hidden");
        assert_eq!(sanitize_filename("--dashes--"), "dashes");
    }

    #[test]
    fn sanitize_caps_length() {
        let long = "a".repeat(250);
        assert_eq!(sanitize_filename(&long).chars().count(), 100);
    }

    #[test]
    fn pretty_string_uses_four_space_indent() {
        let v = serde_json::json!({ "a": { "b": 1 } });
        let s = to_pretty_string(&v).unwrap();
        assert!(s.contains("\n    \"a\""), "expected 4-space indent, got:\n{s}");
    }
}
