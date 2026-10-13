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
/// The last line wins, as it does for the app's dotenv loaders.
pub(super) fn file_key(contents: &str) -> Option<String> {
    file_keys(contents).pop()
}

/// The value of every key line, in order. Lines are found as [`with_key`]
/// finds them, so reading and writing always agree on which line is the key;
/// only the value is parsed by the runtime's loader (quotes, comments).
fn file_keys(contents: &str) -> Vec<String> {
    let lines: Vec<&str> = contents.lines().collect();
    lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| key_line(l).map(|line| line_value(line, &lines[..i])))
        .collect()
}

/// A key line's value as dotenv loaders read it, or the text after `=` when
/// the loader rejects the line. A `$` is filled in from the earlier lines
/// that set other variables and parse on their own.
fn line_value(line: &str, earlier: &[&str]) -> String {
    let mut text = String::new();
    if line.contains('$') {
        // Trimmed as the key line is: a trailing space would fail the line alone.
        let trimmed = earlier
            .iter()
            .map(|l| l.trim_start_matches('\u{feff}').trim_end());
        let parses = |l: &&str| dotenvy::from_read_iter(l.as_bytes()).all(|r| r.is_ok());
        for l in trimmed.filter(|l| key_line(l).is_none()).filter(parses) {
            text.push_str(l);
            text.push('\n');
        }
    }
    text.push_str(line.trim_end());
    dotenvy::from_read_iter(text.as_bytes())
        .flatten()
        .filter(|(key, _)| key == API_KEY_ENV)
        .map(|(_, value)| value)
        .last()
        .unwrap_or_else(|| {
            line.split_once('=')
                .map_or("", |(_, v)| v)
                .trim()
                .to_string()
        })
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
/// symlink rather than following it. Also used for the sign-in session's files.
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

/// Make a regular file owner-only; `true` when that changed something.
/// A symlink is left alone: its target belongs to whoever made it.
pub(super) fn make_private(path: &Path) -> Result<bool, CliError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let Ok(meta) = std::fs::symlink_metadata(path) else {
            return Ok(false);
        };
        if !meta.is_file() || meta.permissions().mode() & 0o077 == 0 {
            return Ok(false);
        }
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| CliError::Other(anyhow::anyhow!("chmod {}: {e}", path.display())))?;
        Ok(true)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(false)
    }
}

pub(super) fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

/// The `.env` a dotenv loader would read from `cwd` (the nearest one in the
/// directory or its ancestors, as `dotenvy::dotenv()` searches) and its key,
/// as [`file_key`] reads it. `None` when there is no such file.
pub(super) fn nearest_dotenv(cwd: &Path) -> Option<(std::path::PathBuf, Option<String>)> {
    let path = cwd
        .ancestors()
        .map(|dir| dir.join(".env"))
        .find(|p| p.is_file())?;
    let key = file_key(&std::fs::read_to_string(&path).ok()?);
    Some((path, key))
}

pub(super) fn is_tracked(path: &Path) -> bool {
    git(path, &["ls-files", "--error-unmatch"]) == Some(true)
}

/// Make git ignore the file, adding its name to the `.gitignore` in its
/// directory unless git already ignores it. `Ok(true)` when it added the line;
/// `Ok(false)` when nothing was needed (already ignored, or not a repository).
/// An error when git still doesn't ignore the file afterwards.
pub(super) fn ensure_ignored(path: &Path) -> Result<bool, CliError> {
    if git(path, &["check-ignore", "-q"]) != Some(false) {
        return Ok(false);
    }
    let (dir, name) = split(path);
    let gitignore_path = dir.join(".gitignore");
    let not_ignored = || {
        CliError::Validation(format!(
            "git doesn't ignore {name}, so the key could be committed. Add `/{}` to {}.",
            gitignore_pattern(&name),
            gitignore_path.display()
        ))
    };
    // Never write through a symlinked .gitignore; the repository owns that file.
    if is_symlink(&gitignore_path) {
        return Err(not_ignored());
    }
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
    gitignore.push_str(&format!("/{}\n", gitignore_pattern(&name)));
    std::fs::write(&gitignore_path, gitignore)
        .map_err(|e| CliError::Other(anyhow::anyhow!("write {}: {e}", gitignore_path.display())))?;
    // Report what git now says, not what was written.
    match git(path, &["check-ignore", "-q"]) {
        Some(true) => Ok(true),
        _ => Err(not_ignored()),
    }
}

/// `name` as a `.gitignore` pattern that matches only itself.
fn gitignore_pattern(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if "\\[]*?".contains(c) || (i == 0 && "!#".contains(c)) {
            out.push('\\');
        }
        out.push(c);
    }
    // Trailing spaces are dropped unless escaped.
    if out.ends_with(' ') {
        out.pop();
        out.push_str("\\ ");
    }
    out
}

/// git, with only the environment the shell gave this process. The runtime
/// loads the nearest `.env` at startup, and a repository must not configure
/// the git we run for it: `GIT_CONFIG_*`, `GIT_SSH_COMMAND`, `XDG_CONFIG_HOME`
/// and the like can make it run any command. The repository's own config is
/// read too, and its `core.fsmonitor` would run a command on every `ls-files`
/// and `check-ignore`, so it is switched off.
pub(super) fn git_command() -> Command {
    let mut cmd = Command::new("git");
    cmd.args(["-c", "core.fsmonitor="]);
    for key in super::added_since_startup() {
        cmd.env_remove(key);
    }
    cmd
}

/// git's yes (0) or no (1) about the file, asked from its own directory so the
/// right repository answers; `None` for anything else, such as no repository.
fn git(path: &Path, args: &[&str]) -> Option<bool> {
    let (dir, name) = split(path);
    let out = git_command()
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
