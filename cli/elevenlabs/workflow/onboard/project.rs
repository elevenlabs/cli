//! The project the agent is working in: its stack, its env file, any key it
//! already has, and whether the SDK is wired in. Every step reads it the same
//! way, so they agree on the env file without passing state between them.

use std::path::{Path, PathBuf};

use fern_cli_sdk::error::CliError;

use super::browser;
use super::env_file::{self, API_KEY_ENV, DEFAULT_ENV_FILE};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum KeySource {
    /// The project's env file.
    EnvFile,
    /// A `.env` in the folder or above it, which loaders read before the env file.
    Dotenv(PathBuf),
    /// A variable set in the shell, which wins over any file.
    Shell,
}

pub(super) struct Key {
    pub value: String,
    pub source: KeySource,
}

impl Key {
    /// The file holding the key, which a replacement goes into; `None` for the shell.
    pub fn file<'a>(&'a self, project: &'a Project) -> Option<&'a Path> {
        match &self.source {
            KeySource::EnvFile => Some(&project.env_file),
            KeySource::Dotenv(path) => Some(path),
            KeySource::Shell => None,
        }
    }

    pub fn source_name(&self) -> &'static str {
        match self.source {
            KeySource::EnvFile => "env_file",
            KeySource::Dotenv(_) => "dotenv",
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

    /// The project in `dir`, with `--env-file` when one was given.
    pub fn at(dir: PathBuf, env_file: Option<String>) -> Self {
        let stack = stack_of(&dir);
        let env_display = env_file.unwrap_or_else(|| {
            match stack {
                // python-dotenv's load_dotenv() reads `.env`; JS tooling reads `.env.local`.
                Stack::Python => ".env",
                _ => DEFAULT_ENV_FILE,
            }
            .into()
        });
        Self {
            env_file: dir.join(&env_display),
            env_display,
            dir,
            stack,
            no_browser: false,
        }
    }

    /// The next command's arguments: `step`, plus the flags of this run that it
    /// takes (`--no-browser`), so following `run` never loses one, even
    /// through `check`. The env file is always named, so every later step uses
    /// the same file even if the project's stack (and so the default) changes.
    pub fn next(&self, step: &str) -> String {
        let mut cmd = step.to_string();
        if !step.contains("--env-file") {
            cmd.push_str(&format!(" --env-file {}", shell_arg(&self.env_display)));
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
    /// any other file relative to the project (a `.env` up the tree as
    /// `../../.env`), so it can go in a command run from the project.
    pub fn show(&self, path: &Path) -> String {
        if same_file(path, &self.env_file) {
            return self.env_display.clone();
        }
        if let Ok(inside) = path.strip_prefix(&self.dir) {
            return inside.display().to_string();
        }
        let up = path
            .parent()
            .and_then(|parent| self.dir.strip_prefix(parent).ok())
            .map(|below| below.components().count());
        match (up, path.file_name()) {
            (Some(n), Some(name)) => format!("{}{}", "../".repeat(n), name.to_string_lossy()),
            _ => path.display().to_string(),
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

    /// The key the app would use: a variable set in the shell first, then the
    /// env file (which the CLI tells the app to load, and which Next.js and
    /// Vite read before `.env`), then a `.env` in the folder or above it. The
    /// shell's value is read from before the runtime loaded `.env`.
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
        let line = env_file::read(&self.env_file)
            .ok()
            .and_then(|c| env_file::file_key(&c));
        // An empty line is still the value the app loads, so `.env` doesn't count then.
        let has_line = line.is_some();
        let from_file = line.filter(|k| !k.is_empty());
        let shell = shell.filter(|v| !v.trim().is_empty());
        let dotenv = env_file::nearest_dotenv(&self.dir)
            .filter(|(path, _)| !has_line && !same_file(path, &self.env_file))
            .and_then(|(path, key)| Some((path, key.filter(|k| !k.is_empty())?)));
        let key = |value, source| Some(Key { value, source });
        match (shell, from_file, dotenv) {
            // Exported from the file (direnv, mise): the file is what to change.
            (Some(v), Some(f), _) if v == f => key(f, KeySource::EnvFile),
            (Some(v), None, Some((path, k))) if v == k => key(k, KeySource::Dotenv(path)),
            (Some(v), _, _) => key(v, KeySource::Shell),
            (None, Some(f), _) => key(f, KeySource::EnvFile),
            (None, None, Some((path, k))) => key(k, KeySource::Dotenv(path)),
            (None, None, None) => None,
        }
    }

    /// The command that adds the SDK. A Python one also names it in a manifest,
    /// which is how `check` sees it.
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

    /// How the app gets the key from `file` (as [`Self::show`] names it) into
    /// its environment. A file above the app is never read by the framework.
    pub fn loading(&self, file: &str) -> String {
        if file.starts_with("../") {
            return match self.stack {
                Stack::Python => "load it with python-dotenv (`load_dotenv()` finds it up the tree)".into(),
                _ => format!(
                    "Next.js and Vite only read env files in the app's folder, so load it explicitly: \
                     `node --env-file={file}` or `dotenv.config({{ path: \"{file}\" }})`"
                ),
            };
        }
        match self.stack {
            Stack::Python => format!("load it with python-dotenv (`load_dotenv(\"{file}\")`)"),
            _ => format!(
                "Next.js and Vite read {file} on their own; plain Node needs `node --env-file={file}` or `dotenv.config({{ path: \"{file}\" }})`"
            ),
        }
    }

    /// Some manifest in the project declares an ElevenLabs SDK (any
    /// `@elevenlabs/` package but the CLI, or Python's `elevenlabs`), in the
    /// app's folder or one below it, as in a monorepo.
    pub fn sdk_declared(&self) -> bool {
        self.any_file(declares_sdk)
    }

    /// Some source file imports an ElevenLabs SDK.
    pub fn sdk_imported(&self) -> bool {
        self.any_file(imports_sdk)
    }

    /// Whether `test` holds for a file in the project, skipping hidden folders
    /// and installed dependencies.
    fn any_file(&self, test: fn(&Path) -> bool) -> bool {
        let mut stack = vec![self.dir.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let Ok(kind) = entry.file_type() else {
                    continue;
                };
                if kind.is_dir() {
                    let sample = dir == self.dir && SAMPLE_DIRS.contains(&&*name);
                    if !name.starts_with('.') && !SKIPPED_DIRS.contains(&&*name) && !sample {
                        stack.push(entry.path());
                    }
                } else if kind.is_file() && test(&entry.path()) {
                    return true;
                }
            }
        }
        false
    }
}

fn declares_sdk(path: &Path) -> bool {
    let read = || std::fs::read_to_string(path).unwrap_or_default();
    match path.file_name().and_then(|n| n.to_str()) {
        Some("package.json") => serde_json::from_str::<serde_json::Value>(&read()).is_ok_and(|p| {
            ["dependencies", "devDependencies"].iter().any(|d| {
                p[d].as_object().is_some_and(|deps| {
                    deps.keys()
                        .any(|name| name.starts_with("@elevenlabs/") && name != "@elevenlabs/cli")
                })
            })
        }),
        Some("requirements.txt" | "pyproject.toml" | "Pipfile" | "setup.py" | "setup.cfg") => {
            names_python_sdk(&read())
        }
        _ => false,
    }
}

/// `elevenlabs` as a whole requirement name: at the start of a line or after
/// a space or quote, and not followed by more of a name (`elevenlabs-foo`).
fn names_python_sdk(text: &str) -> bool {
    text.match_indices("elevenlabs").any(|(i, m)| {
        let before = text[..i].chars().next_back();
        let after = text[i + m.len()..].chars().next();
        before.is_none_or(|c| c.is_whitespace() || c == '"' || c == '\'')
            && after.is_none_or(|c| !(c.is_alphanumeric() || c == '-' || c == '_' || c == '.'))
    })
}

/// Installed dependencies, build output and vendored code.
const SKIPPED_DIRS: &[&str] = &[
    "node_modules",
    "venv",
    "__pycache__",
    "dist",
    "build",
    "target",
    "vendor",
    "third_party",
];

/// Sample projects beside the app, at its top level only: deeper down, a
/// folder of that name is the app's own code (`app/examples/`).
const SAMPLE_DIRS: &[&str] = &["examples", "example"];

fn imports_sdk(path: &Path) -> bool {
    let source = matches!(
        path.extension().and_then(|e| e.to_str()),
        Some(
            "js" | "mjs"
                | "cjs"
                | "ts"
                | "mts"
                | "cts"
                | "jsx"
                | "tsx"
                | "vue"
                | "svelte"
                | "astro"
                | "py"
        )
    );
    source
        && std::fs::read_to_string(path).is_ok_and(|s| {
            imports_js_sdk(&s)
                || s.lines().any(|l| {
                    let l = l.trim_start();
                    l.starts_with("from elevenlabs") || l.starts_with("import elevenlabs")
                })
        })
}

/// A quoted `@elevenlabs/<package>` import of any package but the CLI.
pub(super) fn imports_js_sdk(source: &str) -> bool {
    source.match_indices("@elevenlabs/").any(|(i, m)| {
        let name: String = source[i + m.len()..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || "-_.".contains(*c))
            .collect();
        source[..i].ends_with(['"', '\'']) && !name.is_empty() && name != "cli"
    })
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
