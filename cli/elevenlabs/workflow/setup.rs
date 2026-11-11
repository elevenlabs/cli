//! `elevenlabs setup`: install the ElevenLabs plugin (its skills and the
//! hosted MCP server) into the coding agents on this machine, through each
//! agent's own plugin command. Cursor has no such command, so it gets the
//! link to its marketplace page. With none of them found, the skills are
//! copied into `~/.agents/skills`, where other agents look for them.
//!
//! `onboard setup` runs the same install when the developer accepts the offer
//! at the end of onboarding.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use serde::Serialize;

/// The ElevenLabs skills, the whole repo in one download.
pub(super) const SKILLS_ARCHIVE: &str =
    "https://github.com/elevenlabs/skills/archive/refs/heads/main.tar.gz";

const CURSOR_PAGE: &str = "https://cursor.com/marketplace/elevenlabs";
/// Left in every skill folder `setup` writes, so a later run replaces only those.
const MARKER: &str = ".elevenlabs-setup";

/// A coding agent found on this machine, with the program that runs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Agent {
    ClaudeCode(PathBuf),
    Codex(PathBuf),
    Cursor,
}

impl Agent {
    pub fn name(&self) -> &'static str {
        match self {
            Agent::ClaudeCode(_) => "Claude Code",
            Agent::Codex(_) => "Codex",
            Agent::Cursor => "Cursor",
        }
    }

    /// The commands that install the plugin, in order, each with whether its
    /// result decides the agent's. Refreshing the catalog fails harmlessly
    /// before the first install, so it never does.
    fn steps(&self) -> &'static [(&'static [&'static str], bool)] {
        match self {
            Agent::ClaudeCode(_) => &[
                (&["plugin", "marketplace", "update", "elevenlabs"], false),
                (
                    &[
                        "plugin",
                        "install",
                        "elevenlabs",
                        "--marketplace",
                        "elevenlabs/plugin",
                        "--scope",
                        "user",
                    ],
                    true,
                ),
                (&["plugin", "update", "elevenlabs@elevenlabs"], true),
            ],
            Agent::Codex(_) => &[
                (&["plugin", "marketplace", "add", "elevenlabs/plugin"], true),
                (&["plugin", "marketplace", "upgrade", "elevenlabs"], false),
                (&["plugin", "add", "elevenlabs@elevenlabs"], true),
            ],
            Agent::Cursor => &[],
        }
    }
}

/// How one agent, or the skills copy, came out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct Outcome {
    pub agent: &'static str,
    pub ok: bool,
    pub message: String,
}

/// How long one agent command, and the whole run, may take. The whole run
/// stays under the two minutes coding agents give a shell command, so the
/// agent that ran `onboard setup` always gets its reply.
#[derive(Debug, Clone, Copy)]
pub(super) struct Limits {
    pub command: Duration,
    pub total: Duration,
}

pub(super) const LIMITS: Limits = Limits {
    command: Duration::from_secs(30),
    total: Duration::from_secs(100),
};

/// The agents on this machine. `env` reads a variable, `on_path` finds a
/// program on PATH, `home` is the home folder.
pub(super) fn found(
    env: &dyn Fn(&str) -> Option<String>,
    on_path: &dyn Fn(&str) -> Option<PathBuf>,
    home: &Path,
) -> Vec<Agent> {
    let set = |name: &str| env(name).is_some_and(|v| !v.is_empty());
    let mut agents = Vec::new();
    // Inside Claude Code, including the desktop app, which may not put `claude`
    // on PATH. Claude Code installed by npm names `node` or `bun` there instead.
    // A relative path would be found from the project, so it doesn't count.
    let claude = env("CLAUDE_CODE_EXECPATH")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute() && p.is_file() && is_claude(p))
        .or_else(|| on_path("claude"));
    if let Some(claude) = claude {
        agents.push(Agent::ClaudeCode(claude));
    }
    if let Some(codex) = on_path("codex") {
        agents.push(Agent::Codex(codex));
    }
    if home.join(".cursor").is_dir() || set("CURSOR_AGENT") || set("CURSOR_TRACE_ID") {
        agents.push(Agent::Cursor);
    }
    agents
}

/// Whether `path` is a program called `claude`, as Windows spells it or not.
fn is_claude(path: &Path) -> bool {
    let name = path.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase());
    matches!(name.as_deref(), Some("claude" | "claude.exe"))
}

/// [`found`] for this machine, from the shell's environment.
pub(super) fn found_here(home: &Path) -> Vec<Agent> {
    let path = shell_var("PATH").map(OsString::from);
    found(
        &shell_var,
        &|name| on_path(name, path.as_deref(), cfg!(windows)),
        home,
    )
}

/// The home folder: `USERPROFILE` on Windows, `HOME` elsewhere.
pub(super) fn home_for(windows: bool, env: &dyn Fn(&str) -> Option<String>) -> Option<PathBuf> {
    env(if windows { "USERPROFILE" } else { "HOME" })
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

pub(super) fn home() -> Option<PathBuf> {
    home_for(cfg!(windows), &shell_var)
}

/// A variable as the shell set it, not as a project's `.env` (which the
/// runtime loads) may have changed it: a project must never choose the
/// program this starts.
fn shell_var(name: &str) -> Option<String> {
    match super::onboard::shell_env() {
        Some(env) => env_var(env, name, cfg!(windows)),
        None => std::env::var(name).ok(),
    }
}

/// `name` in `env`. Windows ignores the case of a name: PowerShell and cmd
/// pass the search path as `Path`.
fn env_var(env: &HashMap<OsString, OsString>, name: &str, windows: bool) -> Option<String> {
    let value = match env.get(OsStr::new(name)) {
        Some(value) => value,
        None if windows => env.iter().find(|(k, _)| k.eq_ignore_ascii_case(name))?.1,
        None => return None,
    };
    value.to_str().map(str::to_string)
}

/// The first `name` on `path` that is a program (on Windows, with one of
/// the extensions npm and installers use). An empty or relative entry is
/// skipped: it names a folder in the project, whose programs are never run.
pub(super) fn on_path(name: &str, path: Option<&OsStr>, windows: bool) -> Option<PathBuf> {
    let names: Vec<String> = if windows {
        [".exe", ".cmd", ".bat"]
            .iter()
            .map(|ext| format!("{name}{ext}"))
            .collect()
    } else {
        vec![name.to_string()]
    };
    std::env::split_paths(path?)
        .filter(|dir| dir.is_absolute())
        .flat_map(|dir| names.iter().map(move |n| dir.join(n)).collect::<Vec<_>>())
        .find(|p| if windows { p.is_file() } else { is_program(p) })
}

#[cfg(unix)]
fn is_program(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_program(path: &Path) -> bool {
    path.is_file()
}

/// Run `program args` with stdin closed, in `home` so nothing in the project
/// can be loaded as part of it, stopping it and everything it started after
/// `limit`. `program` is a full path, so on Windows std starts a `.cmd` shim
/// through cmd with its own quoting. `Err` is why it failed: its last line of
/// output, or that it ran out of time.
pub(super) fn run_program(
    program: &Path,
    args: &[&str],
    limit: Duration,
    home: &Path,
) -> Result<(), String> {
    let (out, err) = match (tempfile::tempfile(), tempfile::tempfile()) {
        (Ok(out), Ok(err)) => (out, err),
        (Err(e), _) | (_, Err(e)) => return Err(format!("couldn't start it: {e}")),
    };
    if !home.is_dir() {
        return Err(format!("couldn't start it: the home folder {} is missing", home.display()));
    }
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(out.try_clone().map_err(|e| e.to_string())?)
        .stderr(err.try_clone().map_err(|e| e.to_string())?);
    if let Some(env) = super::onboard::shell_env() {
        command.env_clear().envs(env);
    }
    command.current_dir(home).env("PWD", home);
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    // In its own group it gets no Ctrl-C from the terminal: one sent here stops it.
    #[cfg(unix)]
    let interrupt = interrupt::Catch::start();
    let mut child = command
        .spawn()
        .map_err(|e| format!("couldn't start it: {e}"))?;
    let deadline = Instant::now() + limit;
    let status = loop {
        #[cfg(unix)]
        if let Some(signal) = interrupt.caught() {
            stop_all(&mut child);
            let _ = child.wait();
            drop(interrupt);
            std::process::exit(128 + signal);
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                stop_all(&mut child);
                let _ = child.wait();
                return Err(format!(
                    "ran out of time after {} and was stopped",
                    seconds(limit)
                ));
            }
        }
    };
    if status.success() {
        return Ok(());
    }
    let code = status
        .code()
        .map_or("a signal".to_string(), |c| c.to_string());
    let line = last_line(err).or_else(|| last_line(out));
    Err(match line {
        Some(line) => format!("exited with {code}: {line}"),
        None => format!("exited with {code}"),
    })
}

/// Stop `child` and the programs it started: the npm `codex` is a node
/// script that starts the real one, and on Windows a shim runs under cmd.
fn stop_all(child: &mut std::process::Child) {
    #[cfg(unix)]
    if let Ok(group) = i32::try_from(child.id()) {
        // The child leads its own group, so this reaches only what it started.
        unsafe { libc::kill(-group, libc::SIGKILL) };
        // A program being started as the group is killed can miss the signal,
        // so once the child is reaped, the group is killed until it is empty.
        let _ = child.wait();
        for _ in 0..100 {
            if unsafe { libc::kill(-group, libc::SIGKILL) } != 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/PID", &child.id().to_string(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let _ = child.kill();
}

/// Ctrl-C and SIGTERM, caught while an agent command runs, so `run_program`
/// can stop it before exiting as an interrupted program does.
#[cfg(unix)]
mod interrupt {
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Mutex;

    const SIGNALS: [libc::c_int; 2] = [libc::SIGINT, libc::SIGTERM];

    static CAUGHT: AtomicI32 = AtomicI32::new(0);
    /// How many commands are waiting, and the handlers from before the first.
    static WAITING: Mutex<(usize, [libc::sighandler_t; 2])> = Mutex::new((0, [0; 2]));

    extern "C" fn note(signal: libc::c_int) {
        CAUGHT.store(signal, Ordering::SeqCst);
    }

    /// Catches the signals until dropped, which puts the earlier handlers back.
    pub struct Catch;

    impl Catch {
        pub fn start() -> Catch {
            let mut waiting = WAITING.lock().unwrap_or_else(|e| e.into_inner());
            if waiting.0 == 0 {
                CAUGHT.store(0, Ordering::SeqCst);
                for (signal, previous) in SIGNALS.iter().zip(waiting.1.iter_mut()) {
                    let handler = note as extern "C" fn(libc::c_int) as libc::sighandler_t;
                    *previous = unsafe { libc::signal(*signal, handler) };
                    // A signal the parent chose to ignore stays ignored.
                    if *previous == libc::SIG_IGN {
                        unsafe { libc::signal(*signal, libc::SIG_IGN) };
                    }
                }
            }
            waiting.0 += 1;
            Catch
        }

        pub fn caught(&self) -> Option<i32> {
            Some(CAUGHT.load(Ordering::SeqCst)).filter(|&s| s != 0)
        }
    }

    impl Drop for Catch {
        fn drop(&mut self) {
            let mut waiting = WAITING.lock().unwrap_or_else(|e| e.into_inner());
            waiting.0 -= 1;
            if waiting.0 == 0 {
                for (signal, previous) in SIGNALS.iter().zip(waiting.1) {
                    unsafe { libc::signal(*signal, previous) };
                }
            }
        }
    }
}

fn seconds(d: Duration) -> String {
    if d >= Duration::from_secs(1) {
        format!("{} seconds", d.as_secs())
    } else {
        format!("{} ms", d.as_millis())
    }
}

/// How much of a program's captured output is read for its last line.
const TAIL: u64 = 64 * 1024;
/// The longest error line `setup` repeats: the coding agent reads all of it.
const LINE_MAX: usize = 300;

/// The last line with text in a program's captured output, read from its
/// last [`TAIL`] bytes only, however much it printed.
fn last_line(mut file: std::fs::File) -> Option<String> {
    use std::io::{Read, Seek, SeekFrom};
    let len = file.seek(SeekFrom::End(0)).ok()?;
    let from = len.saturating_sub(TAIL);
    file.seek(SeekFrom::Start(from)).ok()?;
    let mut bytes = Vec::new();
    file.take(TAIL).read_to_end(&mut bytes).ok()?;
    // Cut mid-file, the tail can start inside a character.
    let partial = if from > 0 {
        bytes.iter().take(3).take_while(|b| (0x80..0xc0).contains(*b)).count()
    } else {
        0
    };
    clean_last_line(&bytes[partial..])
}

/// The last line with text in `bytes`, where a line ends at `\n` or `\r` (a
/// progress bar redraws itself after `\r`), without terminal escapes or other
/// control characters, and at most [`LINE_MAX`] characters long.
fn clean_last_line(bytes: &[u8]) -> Option<String> {
    let text = without_escapes(&String::from_utf8_lossy(bytes));
    let line = text.split(['\n', '\r']).map(str::trim).rfind(|l| !l.is_empty())?;
    Some(match line.char_indices().nth(LINE_MAX - 1) {
        Some((at, _)) if line[at..].chars().nth(1).is_some() => format!("{}…", &line[..at]),
        _ => line.to_string(),
    })
}

/// `text` without escape sequences (colours, titles, links and the rest),
/// bidi formatting characters or control characters other than line ends;
/// a tab becomes a space.
fn without_escapes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\u{1b}' | '\u{9b}' | '\u{9d}' => {
                // U+009B and U+009D are one-character forms of ESC [ and ESC ].
                let kind = match c {
                    '\u{9b}' => Some('['),
                    '\u{9d}' => Some(']'),
                    _ => chars.next_if(|c| matches!(c, '[' | ']' | 'P' | 'X' | '^' | '_')),
                };
                match kind {
                    // Parameters, then one final character from @ to ~.
                    Some('[') => {
                        for c in chars.by_ref() {
                            if ('@'..='~').contains(&c) {
                                break;
                            }
                        }
                    }
                    // A string, up to BEL or ESC \.
                    Some(_) => {
                        while let Some(c) = chars.next() {
                            if c == '\u{7}' {
                                break;
                            }
                            if c == '\u{1b}' {
                                chars.next();
                                break;
                            }
                        }
                    }
                    // Characters from space to /, then one final character from 0 to ~.
                    None => {
                        while chars.next_if(|c| (' '..='/').contains(c)).is_some() {}
                        chars.next_if(|c| ('0'..='~').contains(c));
                    }
                }
            }
            '\n' | '\r' => out.push(c),
            '\t' => out.push(' '),
            '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}' => {}
            c if c.is_control() => {}
            c => out.push(c),
        }
    }
    out
}

/// Install the plugin into each agent with `run`, which gets the program,
/// its arguments and how long it may take. An agent reached after the whole
/// run's time is up counts as failed.
pub(super) fn install(
    agents: &[Agent],
    limits: Limits,
    run: &dyn Fn(&Path, &[&str], Duration) -> Result<(), String>,
) -> Vec<Outcome> {
    let started = Instant::now();
    install_timed(agents, limits, run, &|| started.elapsed())
}

/// [`install`], with `elapsed` saying how much of the whole run's time is used.
fn install_timed(
    agents: &[Agent],
    limits: Limits,
    run: &dyn Fn(&Path, &[&str], Duration) -> Result<(), String>,
    elapsed: &dyn Fn() -> Duration,
) -> Vec<Outcome> {
    agents
        .iter()
        .map(|agent| {
            let program = match agent {
                Agent::ClaudeCode(p) | Agent::Codex(p) => p,
                Agent::Cursor => {
                    return Outcome {
                        agent: agent.name(),
                        ok: true,
                        message: format!("open {CURSOR_PAGE} and select Install"),
                    }
                }
            };
            let failed = |message: String| Outcome {
                agent: agent.name(),
                ok: false,
                message,
            };
            for (args, decides) in agent.steps() {
                let left = limits.total.saturating_sub(elapsed());
                if left.is_zero() {
                    return failed(format!(
                        "ran out of time: setup stops after {}",
                        seconds(limits.total)
                    ));
                }
                let result = run(program, args, limits.command.min(left));
                if let (Err(why), true) = (result, decides) {
                    let name = program
                        .file_stem()
                        .unwrap_or(program.as_os_str())
                        .to_string_lossy();
                    return failed(format!("`{name} {}` {why}", args.join(" ")));
                }
            }
            Outcome {
                agent: agent.name(),
                ok: true,
                message: "installed the ElevenLabs plugin (skills and the hosted MCP server)"
                    .into(),
            }
        })
        .collect()
}

/// Everything `setup --yes` does: install into the agents found, or, with
/// none, copy the skills.
pub(super) fn run(agents: &[Agent], home: &Path, limits: Limits) -> Vec<Outcome> {
    run_from(agents, home, limits, SKILLS_ARCHIVE)
}

/// [`run`], downloading the skills from `skills_url`.
fn run_from(agents: &[Agent], home: &Path, limits: Limits, skills_url: &str) -> Vec<Outcome> {
    if agents.is_empty() {
        return vec![skills_from(skills_url, home)];
    }
    install(agents, limits, &|program, args, limit| {
        run_program(program, args, limit, home)
    })
}

/// What `setup` does for each agent, before it does it.
fn plan(agents: &[Agent], home: &Path) -> Vec<String> {
    if agents.is_empty() {
        return vec![format!(
            "No Claude Code, Codex or Cursor found: copy the ElevenLabs skills into {}",
            skills_root(home).display()
        )];
    }
    agents
        .iter()
        .map(|agent| match agent {
            Agent::ClaudeCode(p) | Agent::Codex(p) => format!(
                "{} ({}): install the ElevenLabs plugin (skills and the hosted MCP server)",
                agent.name(),
                p.display()
            ),
            Agent::Cursor => format!("Cursor: open {CURSOR_PAGE} and select Install"),
        })
        .collect()
}

/// One line per outcome.
pub(super) fn result_line(outcome: &Outcome) -> String {
    if outcome.ok {
        format!("{}: {}", outcome.agent, outcome.message)
    } else {
        format!("{}: failed: {}", outcome.agent, outcome.message)
    }
}

/// How to finish for an agent the plugin went into: its MCP server signs in
/// in the agent itself.
pub(super) fn finish(outcome: &Outcome) -> Option<String> {
    let how = match outcome.agent {
        "Claude Code" => "run /mcp and sign in to the ElevenLabs MCP server",
        "Codex" => "run `codex mcp login elevenlabs` to sign in to the ElevenLabs MCP server",
        _ => return None,
    };
    outcome
        .ok
        .then(|| format!("restart {}, then {how}", outcome.agent))
}

/// The installed CLI, or npx when npx started this process (it sets
/// `npm_command=exec`), so the next command runs the same, latest CLI.
pub(super) fn cli_prefix() -> &'static str {
    cli_prefix_for(std::env::var("npm_command").ok().as_deref())
}

pub(super) fn cli_prefix_for(npm_command: Option<&str>) -> &'static str {
    if npm_command == Some("exec") {
        "npx -y @elevenlabs/cli@latest"
    } else {
        "elevenlabs"
    }
}

/// The command to run `setup` again by hand.
pub(super) fn later_command() -> String {
    format!("{} setup --yes", cli_prefix())
}

/// The question that offers `setup` at the end of onboarding, or `None`
/// when only Cursor was found: it has no install command to run.
pub(super) fn question(agents: &[Agent]) -> Option<String> {
    let claude = agents.iter().any(|a| matches!(a, Agent::ClaudeCode(_)));
    let codex = agents.iter().any(|a| matches!(a, Agent::Codex(_)));
    let names = match (claude, codex) {
        (true, true) => "Claude Code and Codex",
        (true, false) => "Claude Code",
        (false, true) => "Codex",
        (false, false) if agents.is_empty() => {
            return Some("Want me to install the ElevenLabs skills for your coding agents?".into())
        }
        (false, false) => return None,
    };
    Some(format!(
        "Want me to set ElevenLabs up in {names} too? It installs the ElevenLabs plugin (skills and \
         the hosted MCP server), so new sessions can use ElevenLabs right away."
    ))
}

fn skills_root(home: &Path) -> PathBuf {
    home.join(".agents").join("skills")
}

/// Download the skills archive from `url` and copy them into `~/.agents/skills`.
fn skills_from(url: &str, home: &Path) -> Outcome {
    let result = download_skills(url).and_then(|dir| copy_skills(&dir.path().join("skills"), home));
    let root = skills_root(home);
    match result {
        Ok(copied) if copied.written.is_empty() && copied.kept.is_empty() => Outcome {
            agent: "Skills",
            ok: false,
            message: format!("the download from {url} held no skills"),
        },
        Ok(copied) => {
            let mut message = format!(
                "copied {} ElevenLabs skills into {}",
                copied.written.len(),
                root.display()
            );
            if !copied.kept.is_empty() {
                message.push_str(&format!(
                    "; left alone, since setup didn't write them: {}",
                    copied.kept.join(", ")
                ));
            }
            Outcome {
                agent: "Skills",
                ok: true,
                message,
            }
        }
        Err(why) => Outcome {
            agent: "Skills",
            ok: false,
            message: why,
        },
    }
}

/// A new temp folder, removed when dropped, with the skills archive from
/// `url` unpacked into its `skills` folder.
fn download_skills(url: &str) -> Result<tempfile::TempDir, String> {
    let dir = tempfile::tempdir().map_err(|e| format!("couldn't make a temp folder: {e}"))?;
    let archive = dir.path().join("skills.tar.gz");
    let unpacked = dir.path().join("skills");
    std::fs::create_dir(&unpacked).map_err(|e| e.to_string())?;
    // Not the API client: it refuses the redirect GitHub answers with, to
    // keep a key from following it. Nothing secret goes to GitHub.
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;
    let body = super::say::run_async(async {
        let response = client
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        response.bytes().await
    })
    .map_err(|e| format!("couldn't download the skills from {url}: {e}"))?;
    std::fs::write(&archive, &body).map_err(|e| e.to_string())?;
    let out = Command::new("tar")
        .arg("-xzf")
        .arg(&archive)
        .arg("-C")
        .arg(&unpacked)
        .arg("--strip-components=1")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("couldn't run tar: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "tar couldn't unpack the skills: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(dir)
}

/// The skill folders written, and the ones left as they were.
#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Copied {
    pub written: Vec<String>,
    pub kept: Vec<String>,
}

/// Copy each top-level folder of `unpacked` that holds a `SKILL.md` into
/// `<home>/.agents/skills/<name>`, marking each one written. A folder already
/// there is replaced only when it has the mark: anything else is the
/// developer's, and is left as it was.
pub(super) fn copy_skills(unpacked: &Path, home: &Path) -> Result<Copied, String> {
    let root = skills_root(home);
    std::fs::create_dir_all(&root).map_err(|e| format!("{}: {e}", root.display()))?;
    let mut names: Vec<String> = std::fs::read_dir(unpacked)
        .map_err(|e| e.to_string())?
        .flatten()
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter(|e| e.path().join("SKILL.md").is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let mut copied = Copied::default();
    for name in names {
        let target = root.join(&name);
        if let Ok(meta) = std::fs::symlink_metadata(&target) {
            let ours = meta.is_dir() && target.join(MARKER).is_file();
            if !ours {
                copied.kept.push(name);
                continue;
            }
            std::fs::remove_dir_all(&target).map_err(|e| format!("{}: {e}", target.display()))?;
        }
        let failed = |e: std::io::Error| format!("{}: {e}", target.display());
        std::fs::create_dir_all(&target).map_err(failed)?;
        // Marked first, so a copy that stops partway is still replaced next time.
        if let Err(e) = std::fs::write(target.join(MARKER), "") {
            // Unmarked, it would count as the developer's and never be filled.
            let _ = std::fs::remove_dir(&target);
            return Err(failed(e));
        }
        copy_dir(&unpacked.join(&name), &target).map_err(failed)?;
        copied.written.push(name);
    }
    Ok(copied)
}

/// Files and folders only: a link in the archive is not followed.
fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let dest = to.join(entry.file_name());
        if kind.is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else if kind.is_file() {
            std::fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

fn handle(matches: &clap::ArgMatches) -> Result<(), CliError> {
    let home = home()
        .ok_or_else(|| CliError::Validation("No home folder found (HOME is not set).".into()))?;
    let agents = found_here(&home);
    if !matches.get_flag("yes") || super::util::dry_run_flag(matches) {
        for line in plan(&agents, &home) {
            println!("{line}");
        }
        println!("Nothing was changed. Run `{}` to do it.", later_command());
        return Ok(());
    }
    let (text, failed) = report(&run(&agents, &home, LIMITS));
    print!("{text}");
    if failed {
        std::process::exit(1);
    }
    Ok(())
}

/// What `setup --yes` prints for `outcomes`, and whether any failed.
fn report(outcomes: &[Outcome]) -> (String, bool) {
    let mut text = String::new();
    for outcome in outcomes {
        text += &format!("{}\n", result_line(outcome));
    }
    let failed = outcomes.iter().any(|o| !o.ok);
    if failed {
        text += &format!("To try again, run `{}`.\n", later_command());
    }
    let finish: Vec<String> = outcomes.iter().filter_map(finish).collect();
    if !finish.is_empty() {
        text += "\nTo finish:\n";
        for step in finish {
            text += &format!("- {step}\n");
        }
    }
    (text, failed)
}

pub fn register(app: CliApp) -> CliApp {
    app.command(
        clap::Command::new("setup")
            .about("Install the ElevenLabs plugin (skills and the hosted MCP server) into your coding agents")
            .arg(
                clap::Arg::new("yes")
                    .long("yes")
                    .action(clap::ArgAction::SetTrue)
                    .help("Install; without it, setup only says what it would do"),
            ),
        Box::new(|matches, _ctx| handle(matches)),
    )
}

#[cfg(test)]
mod tests;
