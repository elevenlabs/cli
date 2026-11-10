//! `elevenlabs setup`: install the ElevenLabs plugin (its skills and the
//! hosted MCP server) into the coding agents on this machine, through each
//! agent's own plugin command. Cursor has no such command, so it gets the
//! link to its marketplace page. With none of them found, the skills are
//! copied into `~/.agents/skills`, where other agents look for them.
//!
//! `onboard setup` runs the same install when the developer accepts the offer
//! at the end of onboarding.

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
    // Inside Claude Code, including the desktop app, which may not put `claude` on PATH.
    let claude = env("CLAUDE_CODE_EXECPATH")
        .map(PathBuf::from)
        .filter(|p| p.is_file())
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
        Some(env) => env
            .get(OsStr::new(name))
            .and_then(|v| v.to_str())
            .map(str::to_string),
        None => std::env::var(name).ok(),
    }
}

/// The first `name` on `path` that is a program (on Windows, with one of
/// the extensions npm and installers use).
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

/// The program and arguments that run `program args`. Never through a shell
/// on Unix; on Windows through `cmd /C`, since npm installs agents as `.cmd`
/// shims that can't be started by name.
pub(super) fn argv(program: &Path, args: &[&str], windows: bool) -> (OsString, Vec<OsString>) {
    let args = args.iter().map(OsString::from);
    if windows {
        let mut all = vec![OsString::from("/C"), program.as_os_str().to_owned()];
        all.extend(args);
        (OsString::from("cmd"), all)
    } else {
        (program.as_os_str().to_owned(), args.collect())
    }
}

/// Run `program args` with stdin closed, stopping it after `limit`. `Err` is
/// why it failed: its last line of output, or that it ran out of time.
pub(super) fn run_program(program: &Path, args: &[&str], limit: Duration) -> Result<(), String> {
    let (exe, argv) = argv(program, args, cfg!(windows));
    let (out, err) = match (tempfile::tempfile(), tempfile::tempfile()) {
        (Ok(out), Ok(err)) => (out, err),
        (Err(e), _) | (_, Err(e)) => return Err(format!("couldn't start it: {e}")),
    };
    let mut command = Command::new(exe);
    command
        .args(argv)
        .stdin(Stdio::null())
        .stdout(out.try_clone().map_err(|e| e.to_string())?)
        .stderr(err.try_clone().map_err(|e| e.to_string())?);
    if let Some(env) = super::onboard::shell_env() {
        command.env_clear().envs(env);
    }
    let mut child = command
        .spawn()
        .map_err(|e| format!("couldn't start it: {e}"))?;
    let deadline = Instant::now() + limit;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
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

fn seconds(d: Duration) -> String {
    if d >= Duration::from_secs(1) {
        format!("{} seconds", d.as_secs())
    } else {
        format!("{} ms", d.as_millis())
    }
}

/// The last line with text in a program's captured output.
fn last_line(mut file: std::fs::File) -> Option<String> {
    use std::io::{Read, Seek};
    file.seek(std::io::SeekFrom::Start(0)).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    String::from_utf8_lossy(&bytes)
        .lines()
        .map(str::trim)
        .rfind(|l| !l.is_empty())
        .map(str::to_string)
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
                let left = limits.total.saturating_sub(started.elapsed());
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
    if agents.is_empty() {
        return vec![skills(home)];
    }
    install(agents, limits, &run_program)
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

/// Download the skills and copy them into `~/.agents/skills`.
fn skills(home: &Path) -> Outcome {
    let result = download_skills().and_then(|dir| copy_skills(&dir.path().join("skills"), home));
    let root = skills_root(home);
    match result {
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

/// A new temp folder, removed when dropped, with the skills archive unpacked
/// into its `skills` folder.
fn download_skills() -> Result<tempfile::TempDir, String> {
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
            .get(SKILLS_ARCHIVE)
            .send()
            .await?
            .error_for_status()?;
        response.bytes().await
    })
    .map_err(|e| format!("couldn't download the skills from {SKILLS_ARCHIVE}: {e}"))?;
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
        copy_dir(&unpacked.join(&name), &target)
            .map_err(|e| format!("{}: {e}", target.display()))?;
        std::fs::write(target.join(MARKER), "").map_err(|e| e.to_string())?;
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
    if !matches.get_flag("yes") {
        for line in plan(&agents, &home) {
            println!("{line}");
        }
        println!("Nothing was changed. Run `{}` to do it.", later_command());
        return Ok(());
    }
    let outcomes = run(&agents, &home, LIMITS);
    for outcome in &outcomes {
        println!("{}", result_line(outcome));
    }
    let finish: Vec<String> = outcomes.iter().filter_map(finish).collect();
    if !finish.is_empty() {
        println!("\nTo finish:");
        for step in finish {
            println!("- {step}");
        }
    }
    if outcomes.iter().any(|o| !o.ok) {
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
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;

    use super::*;

    fn env_of(vars: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |name| vars.get(name).cloned()
    }

    fn path_with(programs: &[&str]) -> impl Fn(&str) -> Option<PathBuf> {
        let programs: Vec<String> = programs.iter().map(|p| p.to_string()).collect();
        move |name| {
            programs
                .iter()
                .any(|p| p == name)
                .then(|| PathBuf::from(format!("/bin/{name}")))
        }
    }

    #[test]
    fn agents_are_found_by_program_variable_and_folder() {
        let home = tempfile::tempdir().unwrap();
        let none = env_of(&[]);
        assert_eq!(found(&none, &path_with(&[]), home.path()), vec![]);
        assert_eq!(
            found(&none, &path_with(&["claude", "codex"]), home.path()),
            vec![
                Agent::ClaudeCode("/bin/claude".into()),
                Agent::Codex("/bin/codex".into())
            ]
        );
        assert_eq!(
            found(&none, &path_with(&["codex"]), home.path()),
            vec![Agent::Codex("/bin/codex".into())]
        );
        for var in ["CURSOR_AGENT", "CURSOR_TRACE_ID"] {
            assert_eq!(
                found(&env_of(&[(var, "1")]), &path_with(&[]), home.path()),
                vec![Agent::Cursor],
                "{var}"
            );
        }
        assert_eq!(
            found(
                &env_of(&[("CURSOR_AGENT", "")]),
                &path_with(&[]),
                home.path()
            ),
            vec![]
        );
        std::fs::create_dir(home.path().join(".cursor")).unwrap();
        assert_eq!(
            found(&none, &path_with(&["claude"]), home.path()),
            vec![Agent::ClaudeCode("/bin/claude".into()), Agent::Cursor]
        );
    }

    #[test]
    fn claude_code_is_the_program_it_names_when_that_exists() {
        let home = tempfile::tempdir().unwrap();
        let program = home.path().join("claude-desktop-bundled");
        std::fs::write(&program, "").unwrap();
        let inside = env_of(&[("CLAUDE_CODE_EXECPATH", program.to_str().unwrap())]);
        assert_eq!(
            found(&inside, &path_with(&["claude"]), home.path()),
            vec![Agent::ClaudeCode(program.clone())]
        );
        assert_eq!(
            found(&inside, &path_with(&[]), home.path()),
            vec![Agent::ClaudeCode(program)]
        );
        let gone = env_of(&[("CLAUDE_CODE_EXECPATH", "/nowhere/claude")]);
        assert_eq!(found(&gone, &path_with(&[]), home.path()), vec![]);
        assert_eq!(
            found(&gone, &path_with(&["claude"]), home.path()),
            vec![Agent::ClaudeCode("/bin/claude".into())]
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_program_on_path_is_an_executable_file() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let empty = tempfile::tempdir().unwrap();
        let program = dir.path().join("codex");
        std::fs::write(&program, "").unwrap();
        let path = std::env::join_paths([empty.path(), dir.path()]).unwrap();
        assert_eq!(on_path("codex", Some(&path), false), None, "not executable");
        std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(on_path("codex", Some(&path), false), Some(program));
        assert_eq!(on_path("claude", Some(&path), false), None);
        assert_eq!(on_path("codex", None, false), None);
    }

    #[test]
    fn on_windows_a_program_on_path_is_an_exe_or_a_cmd_shim() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("codex.cmd"), "").unwrap();
        let path = std::env::join_paths([dir.path()]).unwrap();
        assert_eq!(
            on_path("codex", Some(&path), true),
            Some(dir.path().join("codex.cmd"))
        );
    }

    #[test]
    fn the_home_folder_is_userprofile_on_windows_and_home_elsewhere() {
        let env = env_of(&[("HOME", "/home/dev"), ("USERPROFILE", "C:\\Users\\dev")]);
        assert_eq!(home_for(false, &env), Some(PathBuf::from("/home/dev")));
        assert_eq!(home_for(true, &env), Some(PathBuf::from("C:\\Users\\dev")));
        assert_eq!(home_for(true, &env_of(&[("HOME", "/home/dev")])), None);
        assert_eq!(home_for(false, &env_of(&[("HOME", "")])), None);
    }

    #[test]
    fn agents_start_directly_on_unix_and_through_cmd_on_windows() {
        let args = ["plugin", "add", "elevenlabs@elevenlabs"];
        let (exe, argv) = argv(Path::new("/usr/local/bin/codex"), &args, false);
        assert_eq!(exe, "/usr/local/bin/codex");
        assert_eq!(argv, ["plugin", "add", "elevenlabs@elevenlabs"]);
        let shim = Path::new("C:\\Users\\dev\\AppData\\Roaming\\npm\\codex.cmd");
        let (exe, argv) = super::argv(shim, &args, true);
        assert_eq!(exe, "cmd");
        assert_eq!(
            argv,
            [
                "/C",
                "C:\\Users\\dev\\AppData\\Roaming\\npm\\codex.cmd",
                "plugin",
                "add",
                "elevenlabs@elevenlabs"
            ]
        );
    }

    const LIMITS_FOR_TESTS: Limits = Limits {
        command: Duration::from_secs(30),
        total: Duration::from_secs(100),
    };

    /// Runs nothing; records each command and fails the ones `fails` names.
    struct Recorder {
        ran: RefCell<Vec<String>>,
        fails: Vec<&'static str>,
    }

    impl Recorder {
        fn new(fails: &[&'static str]) -> Self {
            Self {
                ran: RefCell::new(Vec::new()),
                fails: fails.to_vec(),
            }
        }

        fn run(&self, program: &Path, args: &[&str], limit: Duration) -> Result<(), String> {
            assert!(limit <= Duration::from_secs(30));
            let line = format!("{} {}", program.display(), args.join(" "));
            self.ran.borrow_mut().push(line.clone());
            match self.fails.iter().any(|f| line.contains(f)) {
                true => Err("exited with 1: boom".into()),
                false => Ok(()),
            }
        }

        fn ran(&self) -> Vec<String> {
            self.ran.borrow().clone()
        }
    }

    fn all_agents() -> Vec<Agent> {
        vec![
            Agent::ClaudeCode("claude".into()),
            Agent::Codex("codex".into()),
            Agent::Cursor,
        ]
    }

    #[test]
    fn install_runs_each_agents_own_commands_in_order() {
        let recorder = Recorder::new(&[]);
        let outcomes = install(&all_agents(), LIMITS_FOR_TESTS, &|p, a, l| {
            recorder.run(p, a, l)
        });
        assert_eq!(
            recorder.ran(),
            [
                "claude plugin marketplace update elevenlabs",
                "claude plugin install elevenlabs --marketplace elevenlabs/plugin --scope user",
                "claude plugin update elevenlabs@elevenlabs",
                "codex plugin marketplace add elevenlabs/plugin",
                "codex plugin marketplace upgrade elevenlabs",
                "codex plugin add elevenlabs@elevenlabs",
            ]
        );
        assert!(outcomes.iter().all(|o| o.ok), "{outcomes:?}");
        assert_eq!(
            outcomes.iter().map(result_line).collect::<Vec<_>>(),
            [
                "Claude Code: installed the ElevenLabs plugin (skills and the hosted MCP server)",
                "Codex: installed the ElevenLabs plugin (skills and the hosted MCP server)",
                "Cursor: open https://cursor.com/marketplace/elevenlabs and select Install",
            ]
        );
        assert_eq!(
            outcomes.iter().filter_map(finish).collect::<Vec<_>>(),
            [
                "restart Claude Code, then run /mcp and sign in to the ElevenLabs MCP server",
                "restart Codex, then run `codex mcp login elevenlabs` to sign in to the ElevenLabs MCP server",
            ]
        );
    }

    #[test]
    fn only_the_install_commands_decide_an_agents_result() {
        let refresh_fails = Recorder::new(&["marketplace update", "marketplace upgrade"]);
        let outcomes = install(&all_agents(), LIMITS_FOR_TESTS, &|p, a, l| {
            refresh_fails.run(p, a, l)
        });
        assert!(outcomes.iter().all(|o| o.ok), "{outcomes:?}");
        assert_eq!(refresh_fails.ran().len(), 6);

        for (fails, agent) in [
            ("plugin install", "Claude Code"),
            ("plugin update", "Claude Code"),
            ("marketplace add", "Codex"),
            ("plugin add", "Codex"),
        ] {
            let recorder = Recorder::new(&[fails]);
            let outcomes = install(&all_agents(), LIMITS_FOR_TESTS, &|p, a, l| {
                recorder.run(p, a, l)
            });
            let failed: Vec<_> = outcomes.iter().filter(|o| !o.ok).collect();
            assert_eq!(failed.len(), 1, "{fails}: {outcomes:?}");
            assert_eq!(failed[0].agent, agent);
            assert!(failed[0].message.contains(fails), "{failed:?}");
            assert!(
                result_line(failed[0]).starts_with(&format!("{agent}: failed: `"))
                    && result_line(failed[0]).ends_with("` exited with 1: boom"),
                "{}",
                result_line(failed[0])
            );
            assert_eq!(
                finish(failed[0]),
                None,
                "no finishing steps for a failed agent"
            );
        }
    }

    #[test]
    fn agents_not_reached_in_time_count_as_failed() {
        let recorder = Recorder::new(&[]);
        let slow = |p: &Path, a: &[&str], l: Duration| {
            std::thread::sleep(Duration::from_millis(30));
            recorder.run(p, a, l)
        };
        let limits = Limits {
            command: Duration::from_millis(500),
            total: Duration::from_millis(50),
        };
        let outcomes = install(&all_agents(), limits, &slow);
        assert_eq!(recorder.ran().len(), 2, "{:?}", recorder.ran());
        assert!(
            !outcomes[0].ok && outcomes[0].message.contains("ran out of time"),
            "{outcomes:?}"
        );
        assert!(
            !outcomes[1].ok && outcomes[1].message.contains("ran out of time"),
            "{outcomes:?}"
        );
        assert!(outcomes[2].ok, "Cursor runs nothing");
    }

    #[cfg(unix)]
    fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join(name);
        std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[cfg(unix)]
    #[test]
    fn a_command_past_its_limit_is_stopped_and_fails() {
        let dir = tempfile::tempdir().unwrap();
        let hangs = script(dir.path(), "claude", "exec sleep 30");
        let started = Instant::now();
        let err =
            run_program(&hangs, &["plugin", "install"], Duration::from_millis(200)).unwrap_err();
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
        assert_eq!(err, "ran out of time after 200 ms and was stopped");

        let outcomes = install(
            &[Agent::ClaudeCode(hangs), Agent::Cursor],
            Limits {
                command: Duration::from_millis(200),
                total: Duration::from_millis(300),
            },
            &run_program,
        );
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "{:?}",
            started.elapsed()
        );
        assert!(!outcomes[0].ok, "{outcomes:?}");
        assert!(
            outcomes[0].message.contains("ran out of time"),
            "{outcomes:?}"
        );
        assert!(outcomes[1].ok);
    }

    #[cfg(unix)]
    #[test]
    fn a_failing_command_reports_its_last_line_and_stdin_is_closed() {
        let dir = tempfile::tempdir().unwrap();
        let fails = script(
            dir.path(),
            "codex",
            "echo working; echo 'first' >&2; echo 'no such plugin' >&2; exit 3",
        );
        assert_eq!(
            run_program(&fails, &[], Duration::from_secs(10)),
            Err("exited with 3: no such plugin".into())
        );
        let reads = script(dir.path(), "reads", "read line && exit 1; exit 0");
        assert_eq!(run_program(&reads, &[], Duration::from_secs(10)), Ok(()));
    }

    fn skill(dir: &Path, name: &str, text: &str) {
        std::fs::create_dir_all(dir.join(name).join("references")).unwrap();
        std::fs::write(dir.join(name).join("SKILL.md"), text).unwrap();
        std::fs::write(dir.join(name).join("references").join("a.md"), "ref").unwrap();
    }

    #[test]
    fn the_skills_go_into_agents_skills_marked_as_setups() {
        let (unpacked, home) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        skill(unpacked.path(), "text-to-speech", "new tts");
        skill(unpacked.path(), "music", "new music");
        skill(unpacked.path(), "dubbing", "new dubbing");
        std::fs::create_dir_all(unpacked.path().join("evals").join("cases")).unwrap();
        std::fs::write(unpacked.path().join("README.md"), "readme").unwrap();
        let root = home.path().join(".agents").join("skills");

        // One of the developer's own, and one setup wrote before.
        std::fs::create_dir_all(root.join("music")).unwrap();
        std::fs::write(root.join("music").join("SKILL.md"), "mine").unwrap();
        std::fs::create_dir_all(root.join("dubbing")).unwrap();
        std::fs::write(root.join("dubbing").join("SKILL.md"), "old dubbing").unwrap();
        std::fs::write(root.join("dubbing").join("stale.md"), "old").unwrap();
        std::fs::write(root.join("dubbing").join(MARKER), "").unwrap();

        let copied = copy_skills(unpacked.path(), home.path()).unwrap();
        assert_eq!(
            copied,
            Copied {
                written: vec!["dubbing".into(), "text-to-speech".into()],
                kept: vec!["music".into()],
            }
        );
        let read = |p: &[&str]| {
            std::fs::read_to_string(p.iter().fold(root.clone(), |a, b| a.join(b))).unwrap()
        };
        assert_eq!(read(&["text-to-speech", "SKILL.md"]), "new tts");
        assert_eq!(read(&["text-to-speech", "references", "a.md"]), "ref");
        assert_eq!(read(&["text-to-speech", MARKER]), "");
        assert_eq!(read(&["dubbing", "SKILL.md"]), "new dubbing");
        assert!(
            !root.join("dubbing").join("stale.md").exists(),
            "replaced whole"
        );
        assert_eq!(read(&["music", "SKILL.md"]), "mine");
        assert!(!root.join("music").join(MARKER).exists());
        assert_eq!(std::fs::read_dir(root.join("music")).unwrap().count(), 1);
        assert!(!root.join("evals").exists() && !root.join("README.md").exists());

        // Again: everything setup wrote is replaced, the developer's still left.
        let again = copy_skills(unpacked.path(), home.path()).unwrap();
        assert_eq!(again, copied);
    }

    #[test]
    fn the_offer_names_the_agents_that_can_take_the_plugin() {
        let claude = Agent::ClaudeCode("claude".into());
        let codex = Agent::Codex("codex".into());
        let plugin = |names: &str| {
            Some(format!(
                "Want me to set ElevenLabs up in {names} too? It installs the ElevenLabs plugin (skills and the \
                 hosted MCP server), so new sessions can use ElevenLabs right away."
            ))
        };
        assert_eq!(question(&[claude.clone()]), plugin("Claude Code"));
        assert_eq!(question(&[codex.clone(), Agent::Cursor]), plugin("Codex"));
        assert_eq!(
            question(&[claude, codex, Agent::Cursor]),
            plugin("Claude Code and Codex")
        );
        assert_eq!(
            question(&[]),
            Some("Want me to install the ElevenLabs skills for your coding agents?".into())
        );
        assert_eq!(
            question(&[Agent::Cursor]),
            None,
            "Cursor has nothing to run"
        );
    }

    #[test]
    fn the_later_command_runs_the_cli_the_way_this_run_was_started() {
        assert_eq!(
            cli_prefix_for(Some("exec")),
            "npx -y @elevenlabs/cli@latest"
        );
        assert_eq!(cli_prefix_for(Some("install")), "elevenlabs");
        assert_eq!(cli_prefix_for(None), "elevenlabs");
        assert_eq!(
            format!("{} setup --yes", cli_prefix_for(Some("exec"))),
            "npx -y @elevenlabs/cli@latest setup --yes"
        );
        assert_eq!(
            format!("{} setup --yes", cli_prefix_for(None)),
            "elevenlabs setup --yes"
        );
    }

    #[test]
    fn the_plan_says_what_would_happen_for_each_agent() {
        let home = Path::new("/home/dev");
        assert_eq!(
            plan(&[Agent::ClaudeCode("/bin/claude".into()), Agent::Cursor], home),
            [
                "Claude Code (/bin/claude): install the ElevenLabs plugin (skills and the hosted MCP server)",
                "Cursor: open https://cursor.com/marketplace/elevenlabs and select Install",
            ]
        );
        assert_eq!(plan(&[], home).len(), 1);
        assert!(plan(&[], home)[0].contains(&skills_root(home).display().to_string()));
    }
}
