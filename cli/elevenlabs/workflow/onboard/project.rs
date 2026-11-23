//! The project the agent is working in: its stack, its env file and any key
//! it already has. Every step reads it the same way, so they agree on the env
//! file without passing state between them. The CLI never reads the project's
//! code: the coding agent knows the project, the CLI only places the key.

use std::path::{Path, PathBuf};

use fern_cli_sdk::error::CliError;

use super::browser;
use super::env_file::{self, API_KEY_ENV, DEFAULT_ENV_FILE, LOCAL_ENV_FILE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Stack {
    Node,
    Python,
    Other,
}

pub(super) struct Project {
    pub dir: PathBuf,
    pub stack: Stack,
    pub env_file: PathBuf,
    /// The env file as the developer would write it (relative when it was given so).
    pub env_display: String,
    /// `--no-browser`: the developer opens the link themselves (port forwarding).
    pub no_browser: bool,
}

/// Where the key the SDK would use comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum KeySource {
    /// The project's env file.
    EnvFile,
    /// A variable set in the shell, which wins over any file.
    Shell,
}

pub(super) struct Key {
    pub value: String,
    pub source: KeySource,
}

impl Key {
    /// The file holding the key, which a replacement goes into; `None` for the shell.
    pub fn file<'a>(&self, project: &'a Project) -> Option<&'a Path> {
        match self.source {
            KeySource::EnvFile => Some(&project.env_file),
            KeySource::Shell => None,
        }
    }

    pub fn source_name(&self) -> &'static str {
        match self.source {
            KeySource::EnvFile => "env_file",
            KeySource::Shell => "shell",
        }
    }
}

/// A flag's value when the step defines it.
fn arg<T: Clone + Send + Sync + 'static>(matches: &clap::ArgMatches, id: &str) -> Option<T> {
    matches.try_get_one::<T>(id).ok().flatten().cloned()
}

impl Project {
    pub fn open(matches: &clap::ArgMatches) -> Result<Self, CliError> {
        let dir = std::env::current_dir()
            .map_err(|e| CliError::Other(anyhow::anyhow!("read the working directory: {e}")))?;
        let mut project = Self::at(dir, arg::<String>(matches, "env-file"));
        project.no_browser = arg::<bool>(matches, "no-browser").unwrap_or(false);
        Ok(project)
    }

    /// The project in `dir`, with `--env-path` when one was given.
    pub fn at(dir: PathBuf, env_file: Option<String>) -> Self {
        let stack = stack_of(&dir);
        let env_display = env_file.unwrap_or_else(|| default_env_file(&dir).into());
        Self {
            env_file: dir.join(&env_display),
            env_display,
            dir,
            stack,
            no_browser: false,
        }
    }

    /// The next command's arguments: `step`, plus the flags of this run that it
    /// takes (`--no-browser`), so following `run` never loses one. The env file
    /// is always named, so every later step uses the same file even if the
    /// project's stack (and so the default) changes.
    pub fn next(&self, step: &str) -> String {
        let mut cmd = step.to_string();
        if !step.contains("--env-path") {
            cmd.push_str(&format!(" --env-path {}", shell_arg(&self.env_display)));
        }
        if self.no_browser {
            cmd.push_str(" --no-browser");
        }
        cmd
    }

    /// No browser can reach this machine, so keys are added by hand. Even with
    /// `--no-browser` and a forwarded port, the page would copy the key on the
    /// developer's machine, not this one.
    pub fn remote(&self) -> bool {
        !browser::reachable()
    }

    /// How to name `path` to the developer: the env file as they know it, and
    /// any other file relative to the project.
    pub fn show(&self, path: &Path) -> String {
        if same_file(path, &self.env_file) {
            return self.env_display.clone();
        }
        match path.strip_prefix(&self.dir) {
            Ok(inside) => inside.display().to_string(),
            Err(_) => path.display().to_string(),
        }
    }

    /// Why the env file can't hold a key, and how to fix it so the same step then works.
    pub fn env_file_problem(&self) -> Option<String> {
        let name = &self.env_display;
        if let Some(parent) = self.env_file.parent() {
            if !parent.is_dir() {
                return Some(format!(
                    "The folder for {name} doesn't exist. Create {}, then try again.",
                    parent.display()
                ));
            }
        }
        if env_file::is_symlink(&self.env_file) {
            return Some(format!(
                "{name} is a symlink, and the key must go in a regular file. Replace it with a regular file, then try again."
            ));
        }
        if env_file::is_tracked(&self.env_file) {
            return Some(format!(
                "git tracks {name}, so a key in it could be committed. Untrack it with `git rm --cached {}` (the file stays on disk), then try again. \
                 If it ever held a key that was pushed, replace that key too.",
                shell_arg(name)
            ));
        }
        None
    }

    /// The key the app would use: a variable set in the shell first, which
    /// wins over any file, then the env file. The shell's value is read from
    /// before the runtime loaded `.env`.
    pub fn key(&self) -> Option<Key> {
        let shell = match super::shell_env() {
            Some(env) => env
                .get(std::ffi::OsStr::new(API_KEY_ENV))
                .and_then(|v| v.to_str())
                .map(str::to_string),
            None => std::env::var(API_KEY_ENV).ok(),
        };
        self.key_given(shell)
    }

    /// [`Self::key`] with the shell's value passed in.
    pub fn key_given(&self, shell: Option<String>) -> Option<Key> {
        let from_file = env_file::read(&self.env_file)
            .ok()
            .and_then(|c| env_file::file_key(&c))
            .filter(|k| !k.is_empty());
        let shell = shell.filter(|v| !v.trim().is_empty());
        let key = |value, source| Some(Key { value, source });
        match (shell, from_file) {
            // Exported from the file (direnv, mise): the file is what to change.
            (Some(v), Some(f)) if v == f => key(f, KeySource::EnvFile),
            (Some(v), _) => key(v, KeySource::Shell),
            (None, Some(f)) => key(f, KeySource::EnvFile),
            (None, None) => None,
        }
    }

    /// The command that adds the SDK.
    pub fn sdk_install(&self) -> &'static str {
        match self.stack {
            Stack::Python => "pip install elevenlabs (and add elevenlabs to the project's dependencies)",
            Stack::Node => "npm install @elevenlabs/elevenlabs-js",
            Stack::Other => {
                "npm install @elevenlabs/elevenlabs-js (or pip install elevenlabs, and add elevenlabs to requirements.txt)"
            }
        }
    }

    /// What the next sign-in does, as the developer will see it.
    pub fn approval(&self) -> &'static str {
        if self.no_browser {
            "I'll give you a link to the approval page"
        } else {
            "I'll open the approval page"
        }
    }

    /// Where the key is and what the app needs to get it: the goal, not a
    /// recipe for each framework. `file` is the key's file as [`Self::show`]
    /// names it; `None` when the key is set in the shell.
    pub fn load_key(&self, file: Option<&str>) -> String {
        let found = match file {
            Some(file) => format!(
                "It is in {file}, so make sure the app loads that file into its environment before the \
                 SDK is imported: many frameworks only read `.env` on their own, and some read env files \
                 into their own object instead of the process environment."
            ),
            None => "It is set in your shell, so the app gets it from the environment.".into(),
        };
        format!(
            "The SDK reads {API_KEY_ENV} from the environment. {found} Keep the key in server-side \
             code: never in the source, and never in anything that runs in the browser."
        )
    }
}

/// The env file when none was named: `.env`, the file dotenv, python-dotenv
/// and the common frameworks load on their own. `.env.local` in two cases:
/// the project's key is already there, so there is one key and it stays in
/// the file the app loads; or git tracks `.env`, so a key never goes into a
/// file the team commits.
fn default_env_file(dir: &Path) -> &'static str {
    let has_key = |name: &str| {
        env_file::read(&dir.join(name))
            .ok()
            .and_then(|c| env_file::file_key(&c))
            .is_some()
    };
    let local_only = !has_key(DEFAULT_ENV_FILE)
        && (has_key(LOCAL_ENV_FILE) || env_file::is_tracked(&dir.join(DEFAULT_ENV_FILE)));
    if local_only {
        LOCAL_ENV_FILE
    } else {
        DEFAULT_ENV_FILE
    }
}

fn stack_of(dir: &Path) -> Stack {
    let has = |f: &str| dir.join(f).exists();
    if has("package.json") {
        Stack::Node
    } else if ["pyproject.toml", "requirements.txt", "Pipfile", "setup.py"]
        .iter()
        .any(|f| has(f))
    {
        Stack::Python
    } else {
        Stack::Other
    }
}

/// `arg` as one shell word: as is when it is plain, else single-quoted.
pub(super) fn shell_arg(arg: &str) -> String {
    if !arg.is_empty()
        && arg
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "._-/".contains(c))
    {
        arg.to_string()
    } else {
        format!("'{}'", arg.replace('\'', r"'\''"))
    }
}

/// Whether two paths name the same file, by canonical path when both exist.
pub(super) fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

pub(super) fn last4(key: &str) -> String {
    key.chars()
        .skip(key.chars().count().saturating_sub(4))
        .collect()
}
