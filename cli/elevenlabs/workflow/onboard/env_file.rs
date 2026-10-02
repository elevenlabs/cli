//! The env file the key goes into, and keeping it out of git.

use std::path::Path;
use std::process::Command;

use fern_cli_sdk::error::CliError;

pub(super) const API_KEY_ENV: &str = "ELEVENLABS_API_KEY";
pub(super) const DEFAULT_ENV_FILE: &str = ".env.local";
pub(super) const PLACEHOLDER_NOTE: &str =
    "# Paste your ElevenLabs API key after the = below. Create one at \
     https://elevenlabs.io/app/settings/api-keys if you need to. Never paste it into a chat.";

/// The line without a leading `export`, when it is a key line.
fn key_line(line: &str) -> Option<&str> {
    let line = line
        .trim_start()
        .strip_prefix('\u{feff}')
        .unwrap_or(line.trim_start());
    let line = match line.strip_prefix("export") {
        Some(rest) if rest.starts_with(char::is_whitespace) => rest.trim_start(),
        _ => line,
    };
    line.strip_prefix(API_KEY_ENV)
        .filter(|rest| rest.trim_start().starts_with('='))
        .map(|_| line)
}

/// `Some("")` for an empty placeholder line, `None` when there is no line.
/// The last line wins, as it does for dotenv loaders.
pub(super) fn file_key(contents: &str) -> Option<String> {
    let line = contents.lines().filter_map(key_line).next_back()?;
    let value = line.split_once('=')?.1;
    let value = value.split(" #").next().unwrap_or(value).trim();
    Some(value.trim_matches(|c| c == '"' || c == '\'').to_string())
}

/// `contents` with the (last) key line set to `value`, keeping its `export`
/// and the file's line endings; appended when there is none.
pub(super) fn with_key(contents: &str, value: &str) -> String {
    let newline = if contents.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut lines: Vec<String> = contents.lines().map(str::to_string).collect();
    match lines.iter().rposition(|l| key_line(l).is_some()) {
        Some(i) => {
            let export = if lines[i].trim_start().starts_with("export") {
                "export "
            } else {
                ""
            };
            lines[i] = format!("{export}{API_KEY_ENV}={value}");
        }
        None => {
            if value.is_empty() {
                lines.push(PLACEHOLDER_NOTE.to_string());
            }
            lines.push(format!("{API_KEY_ENV}={value}"));
        }
    }
    lines.join(newline) + newline
}

/// Only a missing file counts as empty; never overwrite a file we could not read.
pub(super) fn read(path: &Path) -> Result<String, CliError> {
    match std::fs::read_to_string(path) {
        Ok(c) => Ok(c),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(CliError::Other(anyhow::anyhow!(
            "read {}: {e}",
            path.display()
        ))),
    }
}

/// Write the whole file owner-only and atomically: a temp file next to it,
/// then a rename, so a crash never leaves it truncated. Rename replaces a
/// symlink rather than following it.
pub(super) fn write(path: &Path, contents: &str) -> Result<(), CliError> {
    let (dir, name) = split(path);
    let tmp = dir.join(format!(".{name}.{}.tmp", std::process::id()));
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let result = (|| {
        let mut file = opts.open(&tmp)?;
        std::io::Write::write_all(&mut file, contents.as_bytes())?;
        file.sync_all()?;
        std::fs::rename(&tmp, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    result.map_err(|e| CliError::Other(anyhow::anyhow!("write {}: {e}", path.display())))
}

/// The file's directory and name, for git and for the temp file.
fn split(path: &Path) -> (std::path::PathBuf, String) {
    let dir = match path.parent() {
        Some(d) if !d.as_os_str().is_empty() => d.to_path_buf(),
        _ => Path::new(".").to_path_buf(),
    };
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| DEFAULT_ENV_FILE.to_string());
    (dir, name)
}

pub(super) fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

/// The `.env` a dotenv loader would read from `cwd`: the nearest one in the
/// directory or its ancestors, as `dotenvy::dotenv()` searches. `None` when
/// there is none or it sets no key.
pub(super) fn dotenv_with_key(cwd: &Path) -> Option<(std::path::PathBuf, String)> {
    cwd.ancestors()
        .map(|dir| dir.join(".env"))
        .find(|p| p.is_file())
        .and_then(|p| {
            let key = std::fs::read_to_string(&p)
                .ok()
                .and_then(|c| file_key(&c))?;
            (!key.is_empty()).then_some((p, key))
        })
}

pub(super) fn is_tracked(path: &Path) -> bool {
    git(path, &["ls-files", "--error-unmatch"]) == Some(true)
}

/// Add the file's name to the `.gitignore` in its directory unless git already
/// ignores it. `false` outside a repository.
pub(super) fn ensure_ignored(path: &Path) -> Result<bool, CliError> {
    if git(path, &["check-ignore", "-q"]) != Some(false) {
        return Ok(false);
    }
    let (dir, name) = split(path);
    let gitignore_path = dir.join(".gitignore");
    let mut gitignore = match std::fs::read_to_string(&gitignore_path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => {
            return Err(CliError::Other(anyhow::anyhow!(
                "read {}: {e}",
                gitignore_path.display()
            )))
        }
    };
    if !gitignore.is_empty() && !gitignore.ends_with('\n') {
        gitignore.push('\n');
    }
    let note = "# Local secrets written by `elevenlabs onboard`\n";
    if !gitignore.contains(note) {
        gitignore.push_str(note);
    }
    gitignore.push_str(&format!("{name}\n"));
    // Never write through a symlinked .gitignore; the repository owns that file.
    if is_symlink(&gitignore_path) {
        return Ok(false);
    }
    std::fs::write(&gitignore_path, gitignore)
        .map_err(|e| CliError::Other(anyhow::anyhow!("write {}: {e}", gitignore_path.display())))?;
    // Report what git now says, not what was written.
    Ok(git(path, &["check-ignore", "-q"]) == Some(true))
}

/// git's yes (0) or no (1) about the file, asked from its own directory so the
/// right repository answers; `None` for anything else, such as no repository.
fn git(path: &Path, args: &[&str]) -> Option<bool> {
    let (dir, name) = split(path);
    let out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .args(args)
        .arg("--")
        .arg(&name)
        .output()
        .ok()?;
    match out.status.code()? {
        0 => Some(true),
        1 => Some(false),
        _ => None,
    }
}
