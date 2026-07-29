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

// ── Path containment ────────────────────────────────────────────────
//
// The `config` values in the index files are paths, and those files are meant
// to be committed and cloned — so they are untrusted input. Without a
// containment check a hostile `agents.json` / `tools.json` / `tests.json` turns
// `push` into "read any local JSON and upload it", `pull` into "overwrite any
// file", and `delete` into "unlink any file". A symlink planted inside the
// project achieves the same without touching the index at all.
//
// Everything the workflow reads, writes, or deletes therefore goes through
// [`resolve_in_project`] first, and writes/reads use `O_NOFOLLOW` so the kernel
// refuses a final-component symlink.

/// The project directory that config paths must stay inside.
fn project_root() -> Result<PathBuf, CliError> {
    std::env::current_dir()
        .and_then(|cwd| cwd.canonicalize())
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not determine project directory: {e}")))
}

/// Resolve `.` and `..` without touching the filesystem, so a path that does
/// not exist yet can still be checked for containment.
fn lexically_normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Resolve a config path from an index file (or a CLI flag) and refuse anything
/// that escapes the project directory.
///
/// Catches both `../` traversal and symlinked directories: the deepest existing
/// ancestor is canonicalized, so a symlinked `test_configs/shared` pointing
/// outside the project is rejected even though the literal path looks local.
pub fn resolve_in_project(path_str: &str) -> Result<PathBuf, CliError> {
    if path_str.trim().is_empty() {
        return Err(CliError::Validation("Config path is empty".to_string()));
    }
    if path_str.chars().any(char::is_control) {
        return Err(CliError::Validation(format!(
            "Config path contains control characters: {path_str:?}"
        )));
    }

    let root = project_root()?;
    let candidate = {
        let raw = Path::new(path_str);
        if raw.is_absolute() {
            raw.to_path_buf()
        } else {
            root.join(raw)
        }
    };
    let normalized = lexically_normalize(&candidate);

    if !normalized.starts_with(&root) {
        return Err(escaped(path_str, &root));
    }

    // Canonicalize the deepest ancestor that exists; if a symlink redirects
    // any component out of the project, this is where it shows up.
    let mut ancestor = normalized.as_path();
    let real = loop {
        match ancestor.canonicalize() {
            Ok(real) => break real,
            Err(_) => match ancestor.parent() {
                Some(parent) => ancestor = parent,
                // Walked past the root without finding anything real.
                None => return Err(escaped(path_str, &root)),
            },
        }
    };
    if !real.starts_with(&root) {
        return Err(escaped(path_str, &root));
    }

    Ok(normalized)
}

fn escaped(path_str: &str, root: &Path) -> CliError {
    CliError::Validation(format!(
        "Refusing to use config path '{path_str}': it resolves outside the project directory {}. \
         Config paths in agents.json / tools.json / tests.json must stay inside the project.",
        root.display()
    ))
}

/// Open a file for writing, refusing to follow a symlink at the final
/// component. Without this a symlink planted in the project redirects the write
/// to its target — including creating the target when the link dangles.
fn create_no_follow(path: &Path) -> Result<std::fs::File, CliError> {
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(not(unix))]
    {
        // No O_NOFOLLOW equivalent; check explicitly (racy, but better than nothing).
        if let Ok(meta) = std::fs::symlink_metadata(path) {
            if meta.file_type().is_symlink() {
                return Err(symlink_refused(path));
            }
        }
    }
    opts.open(path).map_err(|e| {
        // ELOOP is what O_NOFOLLOW returns for a symlink.
        #[cfg(unix)]
        if e.raw_os_error() == Some(libc::ELOOP) {
            return symlink_refused(path);
        }
        CliError::Other(anyhow::anyhow!(
            "Could not write configuration file to {}: {e}",
            path.display()
        ))
    })
}

fn symlink_refused(path: &Path) -> CliError {
    CliError::Validation(format!(
        "Refusing to write through the symlink at {} — a symlinked config path can redirect \
         writes outside the project.",
        path.display()
    ))
}

/// Read a config file named by an index file, with containment + symlink checks.
pub fn read_value_in_project(path_str: &str) -> Result<Value, CliError> {
    let path = resolve_in_project(path_str)?;
    if let Ok(meta) = std::fs::symlink_metadata(&path) {
        if meta.file_type().is_symlink() {
            return Err(CliError::Validation(format!(
                "Refusing to read through the symlink at {}",
                path.display()
            )));
        }
    }
    read_value(&path)
}

/// Write a config file named by an index file, with containment + symlink checks.
pub fn write_json_in_project<T: Serialize>(path_str: &str, value: &T) -> Result<(), CliError> {
    write_json(&resolve_in_project(path_str)?, value)
}

/// Delete a config file named by an index file, with containment + symlink
/// checks. Returns whether a file was removed.
pub fn remove_in_project(path_str: &str) -> Result<bool, CliError> {
    let path = resolve_in_project(path_str)?;
    match std::fs::symlink_metadata(&path) {
        Err(_) => Ok(false),
        Ok(meta) if meta.file_type().is_symlink() => Err(CliError::Validation(format!(
            "Refusing to delete through the symlink at {}",
            path.display()
        ))),
        Ok(meta) if !meta.file_type().is_file() => Err(CliError::Validation(format!(
            "Refusing to delete {} — not a regular file",
            path.display()
        ))),
        Ok(_) => Ok(std::fs::remove_file(&path).is_ok()),
    }
}

/// Config files can hold webhook headers and other sensitive values, so keep
/// them owner-only (v0 used the same posture for its credential file).
fn restrict_permissions(file: &std::fs::File) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = file.set_permissions(std::fs::Permissions::from_mode(0o600));
    }
    #[cfg(not(unix))]
    let _ = file;
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
///
/// Refuses to follow a symlink at the final component and writes owner-only:
/// this is the single primitive every workflow write goes through, so the
/// guarantee holds even for paths that don't come from an index file (`init`
/// writing `agents.json`, for instance).
pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CliError> {
    use std::io::Write;
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
    let mut file = create_no_follow(path)?;
    restrict_permissions(&file);
    file.write_all(rendered.as_bytes()).map_err(|e| {
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

// ── Config discovery ────────────────────────────────────────────────

/// Recursively collect every `.json` file under `dir`, sorted, with paths
/// normalized to forward slashes so index files stay portable across
/// platforms. Returns empty when the directory doesn't exist. Ports v0's
/// `discoverJsonFiles`.
pub fn discover_json_files(dir: &str) -> Vec<String> {
    let root = Path::new(dir);
    if !root.exists() {
        return Vec::new();
    }
    let mut found = Vec::new();
    walk_json(root, &mut found);
    found.sort();
    found
}

fn walk_json(dir: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        // Use symlink_metadata, not is_dir()/is_file(): following a symlinked
        // directory here would let a repo point discovery at files outside the
        // project, which `tests push` would then upload to the API.
        let Ok(meta) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            walk_json(&path, found);
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            // Normalize to posix separators for portable index entries.
            let as_posix = path
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");
            found.push(as_posix);
        }
    }
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

    // ── Path containment (security regressions) ─────────────────────
    //
    // These lock in fixes for three exploits that were reproduced against the
    // pre-fix binary: a hostile index file could delete/overwrite/read any
    // file, and a symlink planted in the project redirected writes outside it.

    fn in_temp_project<T>(body: impl FnOnce(&Path) -> T) -> T {
        // resolve_in_project is relative to the CWD, so run inside a temp dir.
        // Serialized via a mutex because CWD is process-global.
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().canonicalize().expect("canonicalize");
        let previous = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(&root).expect("chdir");
        let result = body(&root);
        let _ = std::env::set_current_dir(previous);
        result
    }

    #[test]
    fn traversal_out_of_the_project_is_refused() {
        in_temp_project(|_root| {
            for escape in ["../victim.txt", "../../etc/passwd", "a/../../outside.json"] {
                assert!(
                    resolve_in_project(escape).is_err(),
                    "{escape} should be refused"
                );
            }
        });
    }

    #[test]
    fn absolute_paths_outside_the_project_are_refused() {
        in_temp_project(|_root| {
            assert!(resolve_in_project("/etc/passwd").is_err());
        });
    }

    #[test]
    fn ordinary_project_relative_paths_are_allowed() {
        in_temp_project(|root| {
            let resolved = resolve_in_project("agent_configs/My-Agent.json").expect("allowed");
            assert!(resolved.starts_with(root));
            // A path that doesn't exist yet must still resolve (pull writes new files).
            assert!(resolve_in_project("agent_configs/nested/new.json").is_ok());
        });
    }

    #[test]
    fn a_symlinked_directory_cannot_smuggle_a_path_out_of_the_project() {
        in_temp_project(|_root| {
            let outside = tempfile::tempdir().expect("outside");
            std::fs::create_dir_all("test_configs").expect("mkdir");
            #[cfg(unix)]
            std::os::unix::fs::symlink(outside.path(), "test_configs/shared").expect("symlink");
            #[cfg(unix)]
            assert!(
                resolve_in_project("test_configs/shared/x.json").is_err(),
                "a symlinked directory must not escape containment"
            );
        });
    }

    #[test]
    fn writes_refuse_to_follow_a_symlink() {
        in_temp_project(|_root| {
            let outside = tempfile::tempdir().expect("outside");
            let target = outside.path().join("planted.json");
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&target, "agents.json").expect("symlink");
                let err = write_json(Path::new("agents.json"), &serde_json::json!({}));
                assert!(err.is_err(), "writing through a symlink must be refused");
                assert!(!target.exists(), "the symlink target must not be created");
            }
        });
    }

    #[test]
    fn deletes_refuse_to_escape_or_follow_a_symlink() {
        in_temp_project(|_root| {
            let outside = tempfile::tempdir().expect("outside");
            let victim = outside.path().join("victim.txt");
            std::fs::write(&victim, "keep me").expect("write victim");

            // Direct traversal.
            assert!(remove_in_project("../victim.txt").is_err());
            // Via a symlink inside the project.
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&victim, "link.json").expect("symlink");
                assert!(remove_in_project("link.json").is_err());
            }
            assert!(victim.exists(), "victim must survive both attempts");
        });
    }

    #[test]
    fn discovery_skips_symlinks() {
        in_temp_project(|_root| {
            std::fs::create_dir_all("test_configs").expect("mkdir");
            std::fs::write("test_configs/real.json", "{}").expect("write");
            let outside = tempfile::tempdir().expect("outside");
            std::fs::write(outside.path().join("private.json"), "{}").expect("write");
            #[cfg(unix)]
            std::os::unix::fs::symlink(outside.path(), "test_configs/shared").expect("symlink");

            let found = discover_json_files("test_configs");
            assert_eq!(found.len(), 1, "only the real file should be discovered: {found:?}");
            assert!(found[0].ends_with("real.json"));
        });
    }

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
