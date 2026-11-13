//! `elevenlabs setup`: install the ElevenLabs plugin (its skills and the
//! hosted MCP server) into the coding agents on this machine, through each
//! agent's own plugin command. Cursor has no such command, so it gets the
//! link to its marketplace page. With none of them found, it says how to
//! install the skills.
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

const CURSOR_PAGE: &str = "https://cursor.com/marketplace/elevenlabs";

/// A coding agent found on this machine, with the program that runs it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Agent {
    ClaudeCode(PathBuf),
    Codex(PathBuf),
    Cursor,
}

/// One command that installs the plugin.
struct Step {
    args: &'static [&'static str],
    /// Whether its result decides the agent's. A command that prepares the
    /// marketplace never does: a refresh fails while the plugin is installed
    /// and working when GitHub can't be reached, and an `add` is refused when
    /// a marketplace of that name came from elsewhere. Without a marketplace,
    /// the install fails anyway.
    decides: bool,
}

impl Agent {
    pub fn name(&self) -> &'static str {
        match self {
            Agent::ClaudeCode(_) => "Claude Code",
            Agent::Codex(_) => "Codex",
            Agent::Cursor => "Cursor",
        }
    }

    /// The commands that install the plugin, in order. These work on every
    /// version of each agent.
    fn steps(&self) -> &'static [Step] {
        match self {
            Agent::ClaudeCode(_) => &[
                Step { decides: false, args: &["plugin", "marketplace", "add", "elevenlabs/plugin"] },
                Step { decides: false, args: &["plugin", "marketplace", "update", "elevenlabs"] },
                Step { decides: true, args: &["plugin", "install", "elevenlabs@elevenlabs"] },
                Step { decides: true, args: &["plugin", "update", "elevenlabs@elevenlabs"] },
            ],
            Agent::Codex(_) => &[
                Step { decides: false, args: &["plugin", "marketplace", "add", "elevenlabs/plugin"] },
                Step { decides: false, args: &["plugin", "marketplace", "upgrade", "elevenlabs"] },
                Step { decides: true, args: &["plugin", "add", "elevenlabs@elevenlabs"] },
            ],
            Agent::Cursor => &[],
        }
    }

    /// How to finish once the plugin is in: its MCP server signs in in the
    /// agent itself.
    fn finish(&self) -> Option<&'static str> {
        match self {
            Agent::ClaudeCode(_) => {
                Some("restart Claude Code, then run /mcp and sign in to the ElevenLabs MCP server")
            }
            Agent::Codex(_) => Some(
                "restart Codex, then run `codex mcp login elevenlabs` in a terminal to sign in to the ElevenLabs MCP \
                 server",
            ),
            Agent::Cursor => None,
        }
    }
}

/// How one agent came out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct Outcome {
    pub agent: &'static str,
    pub ok: bool,
    pub message: String,
    /// How to finish, for an agent the plugin went into.
    #[serde(skip)]
    pub finish: Option<&'static str>,
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
/// output, or that it ran out of time. During [`run`], a Ctrl-C or SIGTERM
/// stops the program and exits this process as an interrupted one does.
pub(super) fn run_program(
    program: &Path,
    args: &[&str],
    limit: Duration,
    home: &Path,
) -> Result<(), String> {
    #[cfg(unix)]
    interrupt::exit_if_caught();
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
    // In its own group it gets no Ctrl-C from the terminal: `run` catches it.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut command, 0);
    let mut child = command
        .spawn()
        .map_err(|e| format!("couldn't start it: {e}"))?;
    let deadline = Instant::now() + limit;
    let status = loop {
        #[cfg(unix)]
        if let Some(signal) = interrupt::caught() {
            stop_all(&mut child);
            std::process::exit(128 + signal);
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                stop_all(&mut child);
                return Err(format!("ran out of time after {limit:?} and was stopped"));
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

/// Stop `child` and the programs it started, and wait for it: the npm
/// `codex` is a node script that starts the real one, and on Windows a shim
/// runs under cmd.
fn stop_all(child: &mut std::process::Child) {
    // The child leads its own group, so this reaches only what it started.
    #[cfg(unix)]
    let group = i32::try_from(child.id()).ok();
    #[cfg(unix)]
    if let Some(group) = group {
        unsafe { libc::kill(-group, libc::SIGKILL) };
    }
    #[cfg(windows)]
    let _ = Command::new("taskkill")
        .args(["/PID", &child.id().to_string(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let _ = child.kill();
    let _ = child.wait();
    // A program being started as the group is killed can miss the signal,
    // so once the child is reaped, the group is killed until it is empty.
    #[cfg(unix)]
    if let Some(group) = group {
        for _ in 0..100 {
            if unsafe { libc::kill(-group, libc::SIGKILL) } != 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}

/// Ctrl-C and SIGTERM, caught while [`run`] runs the agent commands, so the
/// one running can be stopped before this process exits as an interrupted
/// program does.
#[cfg(unix)]
mod interrupt {
    use std::sync::atomic::{AtomicI32, Ordering};

    const SIGNALS: [libc::c_int; 2] = [libc::SIGINT, libc::SIGTERM];

    static CAUGHT: AtomicI32 = AtomicI32::new(0);

    extern "C" fn note(signal: libc::c_int) {
        CAUGHT.store(signal, Ordering::SeqCst);
    }

    /// Catches the signals, holding the handlers from before.
    pub struct Catch([libc::sighandler_t; 2]);

    impl Catch {
        pub fn start() -> Catch {
            CAUGHT.store(0, Ordering::SeqCst);
            let handler = note as extern "C" fn(libc::c_int) as libc::sighandler_t;
            Catch(SIGNALS.map(|signal| {
                let previous = unsafe { libc::signal(signal, handler) };
                // A signal the parent chose to ignore stays ignored.
                if previous == libc::SIG_IGN {
                    unsafe { libc::signal(signal, libc::SIG_IGN) };
                }
                previous
            }))
        }

        /// Put the earlier handlers back, then exit if a signal came meanwhile.
        pub fn finish(self) {
            for (signal, previous) in SIGNALS.iter().zip(self.0) {
                unsafe { libc::signal(*signal, previous) };
            }
            exit_if_caught();
        }
    }

    pub fn caught() -> Option<i32> {
        Some(CAUGHT.load(Ordering::SeqCst)).filter(|&s| s != 0)
    }

    pub fn exit_if_caught() {
        if let Some(signal) = caught() {
            std::process::exit(128 + signal);
        }
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
    file.seek(SeekFrom::Start(len.saturating_sub(TAIL))).ok()?;
    let mut bytes = Vec::new();
    file.take(TAIL).read_to_end(&mut bytes).ok()?;
    clean_last_line(&bytes)
}

/// The last line with text in `bytes`, where a line ends at `\n` or `\r` (a
/// progress bar redraws itself after `\r`), without terminal escapes, bidi
/// formatting characters or control characters (a tab becomes a space), and
/// at most [`LINE_MAX`] characters long.
fn clean_last_line(bytes: &[u8]) -> Option<String> {
    // U+009B and U+009D are one-character forms of ESC [ and ESC ], which
    // the parser sees only as their two-character forms.
    let text = String::from_utf8_lossy(bytes)
        .replace('\u{9b}', "\u{1b}[")
        .replace('\u{9d}', "\u{1b}]");
    let text: String = anstream::adapter::strip_str(&text)
        .to_string()
        .chars()
        .filter_map(|c| match c {
            '\t' => Some(' '),
            '\n' | '\r' => Some(c),
            '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}' => None,
            c if c.is_control() => None,
            c => Some(c),
        })
        .collect();
    let line = text.split(['\n', '\r']).map(str::trim).rfind(|l| !l.is_empty())?;
    Some(match line.char_indices().nth(LINE_MAX - 1) {
        Some((at, _)) if line[at..].chars().nth(1).is_some() => format!("{}…", &line[..at]),
        _ => line.to_string(),
    })
}

/// Install the plugin into each agent with `run`, which gets the program,
/// its arguments and how long it may take; `elapsed` says how much of the
/// whole run's time is used. An agent reached after that time is up counts
/// as failed, and one that fails shows its first failed command.
pub(super) fn install(
    agents: &[Agent],
    limits: Limits,
    run: &dyn Fn(&Path, &[&str], Duration) -> Result<(), String>,
    elapsed: &dyn Fn() -> Duration,
) -> Vec<Outcome> {
    agents
        .iter()
        .map(|agent| {
            let outcome = |ok: bool, message: String| Outcome {
                agent: agent.name(),
                ok,
                message,
                finish: agent.finish().filter(|_| ok),
            };
            let program = match agent {
                Agent::ClaudeCode(p) | Agent::Codex(p) => p,
                Agent::Cursor => return outcome(true, format!("open {CURSOR_PAGE} and select Install")),
            };
            let name = program.file_stem().unwrap_or(program.as_os_str()).to_string_lossy();
            let mut first_failed = None;
            for step in agent.steps() {
                let left = limits.total.saturating_sub(elapsed());
                if left.is_zero() {
                    let message = format!("ran out of time: setup stops after {:?}", limits.total);
                    return outcome(false, message);
                }
                if let Err(why) = run(program, step.args, limits.command.min(left)) {
                    let failed = first_failed
                        .get_or_insert_with(|| format!("`{name} {}` {why}", step.args.join(" ")));
                    if step.decides {
                        return outcome(false, failed.clone());
                    }
                }
            }
            let message = "installed the ElevenLabs plugin (skills and the hosted MCP server)";
            outcome(true, message.into())
        })
        .collect()
}

/// Everything `setup --yes` does: install into the agents found. A Ctrl-C
/// or SIGTERM meanwhile stops the command running and exits.
pub(super) fn run(agents: &[Agent], home: &Path, limits: Limits) -> Vec<Outcome> {
    #[cfg(unix)]
    let catch = interrupt::Catch::start();
    let started = Instant::now();
    let outcomes = install(
        agents,
        limits,
        &|program, args, limit| run_program(program, args, limit, home),
        &|| started.elapsed(),
    );
    #[cfg(unix)]
    catch.finish();
    outcomes
}

/// What `setup` does for each agent, before it does it.
fn plan(agents: &[Agent]) -> Vec<String> {
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

/// What `setup --yes` and `onboard setup` say for `outcomes`, a sentence or
/// two per line, and whether any failed. With no agent found, how to get the
/// skills.
pub(super) fn report(outcomes: &[Outcome]) -> (Vec<String>, bool) {
    if outcomes.is_empty() {
        return (
            vec![format!(
                "No Claude Code, Codex or Cursor found. If you use Claude Code in the Claude desktop app, \
                 ask it to run \"{}\" for you. For other coding agents, install the ElevenLabs skills \
                 with: npx skills add elevenlabs/skills",
                later_command()
            )],
            false,
        );
    }
    let mut lines: Vec<String> = outcomes
        .iter()
        .map(|o| {
            let failed = if o.ok { "" } else { "failed: " };
            let mut line = format!("{}: {failed}{}.", o.agent, o.message);
            if let Some(finish) = o.finish {
                line.push_str(&format!(" To finish, {finish}."));
            }
            line
        })
        .collect();
    let failed = outcomes.iter().any(|o| !o.ok);
    if failed {
        lines.push(format!("To try again, run `{}`.", later_command()));
    }
    (lines, failed)
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
/// when neither Claude Code nor Codex was found: nothing would be installed.
pub(super) fn question(agents: &[Agent]) -> Option<String> {
    let claude = agents.iter().any(|a| matches!(a, Agent::ClaudeCode(_)));
    let codex = agents.iter().any(|a| matches!(a, Agent::Codex(_)));
    let names = match (claude, codex) {
        (true, true) => "Claude Code and Codex",
        (true, false) => "Claude Code",
        (false, true) => "Codex",
        (false, false) => return None,
    };
    Some(format!(
        "Want me to set ElevenLabs up in {names} too? It installs the ElevenLabs plugin (skills and \
         the hosted MCP server), so new sessions can use ElevenLabs right away."
    ))
}

fn handle(matches: &clap::ArgMatches) -> Result<(), CliError> {
    let home = home()
        .ok_or_else(|| CliError::Validation("No home folder found (HOME is not set).".into()))?;
    let agents = found_here(&home);
    let install = matches.get_flag("yes") && !super::util::dry_run_flag(matches);
    if !install && !agents.is_empty() {
        for line in plan(&agents) {
            println!("{line}");
        }
        println!("Nothing was changed. Run `{}` to do it.", later_command());
        return Ok(());
    }
    let (lines, failed) = report(&run(&agents, &home, LIMITS));
    for line in lines {
        println!("{line}");
    }
    if failed {
        std::process::exit(1);
    }
    Ok(())
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
