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
fn claude_code_is_the_program_it_names_when_that_is_claude() {
    let home = tempfile::tempdir().unwrap();
    for name in ["claude", "Claude.exe", "CLAUDE"] {
        let dir = tempfile::tempdir().unwrap();
        let program = dir.path().join(name);
        std::fs::write(&program, "").unwrap();
        let inside = env_of(&[("CLAUDE_CODE_EXECPATH", program.to_str().unwrap())]);
        assert_eq!(
            found(&inside, &path_with(&["claude"]), home.path()),
            vec![Agent::ClaudeCode(program.clone())],
            "{name}"
        );
        assert_eq!(
            found(&inside, &path_with(&[]), home.path()),
            vec![Agent::ClaudeCode(program)],
            "{name}"
        );
    }
    let gone = env_of(&[("CLAUDE_CODE_EXECPATH", "/nowhere/claude")]);
    assert_eq!(found(&gone, &path_with(&[]), home.path()), vec![]);
    assert_eq!(
        found(&gone, &path_with(&["claude"]), home.path()),
        vec![Agent::ClaudeCode("/bin/claude".into())]
    );
}

/// Claude Code installed by npm names the runtime that runs it, which
/// would load the project's own `plugin.js` given `plugin install`.
#[test]
fn a_runtime_named_by_claude_code_is_never_started() {
    let home = tempfile::tempdir().unwrap();
    for name in ["node", "bun", "node.exe", "claude-desktop", "claude.cmd"] {
        let dir = tempfile::tempdir().unwrap();
        let program = dir.path().join(name);
        std::fs::write(&program, "").unwrap();
        let inside = env_of(&[("CLAUDE_CODE_EXECPATH", program.to_str().unwrap())]);
        assert_eq!(
            found(&inside, &path_with(&["claude"]), home.path()),
            vec![Agent::ClaudeCode("/bin/claude".into())],
            "{name}: claude on PATH instead"
        );
        assert_eq!(
            found(&inside, &path_with(&[]), home.path()),
            vec![],
            "{name}"
        );
    }
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
fn on_windows_a_variable_is_found_whatever_the_case_of_its_name() {
    let env: HashMap<OsString, OsString> = [("Path", "C:\\bin"), ("HOME", ""), ("Cursor_Agent", "1")]
        .iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();
    assert_eq!(env_var(&env, "PATH", true), Some("C:\\bin".into()));
    assert_eq!(env_var(&env, "PATH", false), None);
    assert_eq!(env_var(&env, "CURSOR_AGENT", true), Some("1".into()));
    assert_eq!(env_var(&env, "CURSOR_AGENT", false), None);
    for windows in [true, false] {
        assert_eq!(env_var(&env, "Path", windows), Some("C:\\bin".into()));
        assert_eq!(env_var(&env, "HOME", windows), Some(String::new()), "empty, as set");
        assert_eq!(env_var(&env, "USERPROFILE", windows), None);
    }
    let both: HashMap<OsString, OsString> = [("Path", "lower"), ("PATH", "exact")]
        .iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();
    assert_eq!(env_var(&both, "PATH", true), Some("exact".into()));
}

#[test]
fn the_home_folder_is_userprofile_on_windows_and_home_elsewhere() {
    let env = env_of(&[("HOME", "/home/dev"), ("USERPROFILE", "C:\\Users\\dev")]);
    assert_eq!(home_for(false, &env), Some(PathBuf::from("/home/dev")));
    assert_eq!(home_for(true, &env), Some(PathBuf::from("C:\\Users\\dev")));
    assert_eq!(home_for(true, &env_of(&[("HOME", "/home/dev")])), None);
    assert_eq!(home_for(false, &env_of(&[("HOME", "")])), None);
}

const LIMITS_FOR_TESTS: Limits = Limits {
    command: Duration::from_secs(30),
    total: Duration::from_secs(100),
};

/// Runs nothing; records each command and fails the ones `fails` names,
/// or, made with [`Recorder::exactly`], only that one command.
struct Recorder {
    ran: RefCell<Vec<String>>,
    fails: Vec<&'static str>,
    exactly: bool,
}

impl Recorder {
    fn new(fails: &[&'static str]) -> Self {
        Self {
            ran: RefCell::new(Vec::new()),
            fails: fails.to_vec(),
            exactly: false,
        }
    }

    fn exactly(fails: &'static str) -> Self {
        Self {
            exactly: true,
            ..Self::new(&[fails])
        }
    }

    fn run(&self, program: &Path, args: &[&str], limit: Duration) -> Result<(), String> {
        assert!(limit <= Duration::from_secs(30));
        let line = format!("{} {}", program.display(), args.join(" "));
        self.ran.borrow_mut().push(line.clone());
        let fails = |f: &&str| if self.exactly { line == *f } else { line.contains(f) };
        match self.fails.iter().any(fails) {
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
    // Each command takes 30 s of a 50 s run, without waiting for it.
    let used = std::cell::Cell::new(Duration::ZERO);
    let slow = |p: &Path, a: &[&str], l: Duration| {
        used.set(used.get() + Duration::from_secs(30));
        recorder.run(p, a, l)
    };
    let limits = Limits {
        command: Duration::from_secs(30),
        total: Duration::from_secs(50),
    };
    let outcomes = install_timed(&all_agents(), limits, &slow, &|| used.get());
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

/// A shell script at `dir/name`, written by a short-lived `sh`, never by this
/// process. Tests run as threads of one process: if one forks while another
/// holds a script open for writing, the forked child keeps that descriptor
/// until it execs, and Linux then refuses to start the script with "Text
/// file busy" (rust-lang/rust#114554). A file this process never opens can't
/// be held that way.
#[cfg(unix)]
fn script(dir: &Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    let status = Command::new("/bin/sh")
        .args(["-c", "printf '%s' \"$1\" > \"$2\" && chmod 755 \"$2\"", "sh"])
        .arg(format!("#!/bin/sh\n{body}\n"))
        .arg(&path)
        .stdin(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success(), "writing {}", path.display());
    path
}

/// `sleep <seconds>.<this process's id>`: a command line no other run of
/// these tests on this machine has, for finding what a test left running.
/// The id has a fixed width, so one marker never contains another.
#[cfg(unix)]
fn sleep_marker(seconds: u32) -> String {
    format!("sleep {seconds}.{:010}", std::process::id())
}

#[cfg(unix)]
#[test]
fn a_command_past_its_limit_is_stopped_and_fails() {
    let dir = tempfile::tempdir().unwrap();
    let hangs = script(dir.path(), "claude", "exec sleep 30");
    let started = Instant::now();
    let err = run_program(
        &hangs,
        &["plugin", "install"],
        Duration::from_millis(200),
        dir.path(),
    )
    .unwrap_err();
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
        &|p, a, l| run_program(p, a, l, dir.path()),
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
        run_program(&fails, &[], Duration::from_secs(10), dir.path()),
        Err("exited with 3: no such plugin".into())
    );
    let reads = script(dir.path(), "reads", "read line && exit 1; exit 0");
    assert_eq!(
        run_program(&reads, &[], Duration::from_secs(10), dir.path()),
        Ok(())
    );
}

/// Kills the process when the test ends, passed or not.
#[cfg(unix)]
struct KillOnDrop(i32);

#[cfg(unix)]
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        unsafe { libc::kill(self.0, libc::SIGKILL) };
    }
}

#[cfg(unix)]
#[test]
fn a_command_past_its_limit_is_stopped_with_everything_it_started() {
    let dir = tempfile::tempdir().unwrap();
    let pid = dir.path().join("pid");
    let hangs = script(
        dir.path(),
        "codex",
        &format!("sleep 300 &\necho $! > '{}'\nwait", pid.display()),
    );
    // Long enough for the stand-in to start its child on a busy machine.
    let result = run_program(&hangs, &[], Duration::from_secs(2), dir.path());
    let pid: i32 = std::fs::read_to_string(&pid).unwrap().trim().parse().unwrap();
    let _cleanup = KillOnDrop(pid);
    let err = result.unwrap_err();
    assert!(err.contains("ran out of time"), "{err}");
    let deadline = Instant::now() + Duration::from_secs(5);
    while unsafe { libc::kill(pid, 0) } == 0 {
        assert!(Instant::now() < deadline, "sleep {pid} still runs");
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
#[test]
fn agents_run_in_the_home_folder_not_the_project() {
    let (dir, home) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let wrote = dir.path().join("pwd");
    let prints = script(
        dir.path(),
        "claude",
        &format!("pwd -P > '{0}'; echo \"$PWD\" >> '{0}'", wrote.display()),
    );
    run_program(&prints, &[], Duration::from_secs(10), home.path()).unwrap();
    let home = home.path().canonicalize().unwrap();
    let lines: Vec<PathBuf> = std::fs::read_to_string(&wrote)
        .unwrap()
        .lines()
        .map(|l| Path::new(l).canonicalize().unwrap())
        .collect();
    assert_eq!(lines, [home.clone(), home]);
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

#[cfg(unix)]
#[test]
fn a_copy_that_stops_partway_is_replaced_next_time() {
    use std::os::unix::fs::PermissionsExt;
    if unsafe { libc::geteuid() } == 0 {
        // Root reads a file whatever its mode, so the copy can't be made to stop.
        return;
    }
    let (unpacked, home) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    skill(unpacked.path(), "music", "new music");
    let unreadable = unpacked.path().join("music").join("references").join("a.md");
    std::fs::set_permissions(&unreadable, std::fs::Permissions::from_mode(0o000)).unwrap();
    let target = skills_root(home.path()).join("music");

    assert!(copy_skills(unpacked.path(), home.path()).is_err());
    assert!(target.join(MARKER).is_file(), "marked before the copy");

    std::fs::set_permissions(&unreadable, std::fs::Permissions::from_mode(0o644)).unwrap();
    let copied = copy_skills(unpacked.path(), home.path()).unwrap();
    assert_eq!(copied.written, ["music"]);
    assert_eq!(
        std::fs::read_to_string(target.join("references").join("a.md")).unwrap(),
        "ref"
    );
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

// ── finding agents, as a table ──────────────────────────────────────

/// Where Claude Code is, in a row of the table below.
#[derive(Debug, Clone, Copy)]
enum ClaudeAt {
    ExecPath,
    OnPath,
    Both,
    Neither,
}

/// What says Cursor is there, in a row of the table below.
#[derive(Debug, Clone, Copy)]
enum CursorBy {
    Folder,
    AgentVariable,
    TraceVariable,
    Nothing,
}

#[cfg(unix)]
#[test]
fn every_combination_of_claude_code_codex_and_cursor_is_found() {
    let inside = tempfile::tempdir().unwrap();
    let named = script(inside.path(), "claude", "exit 0");
    let mut rows = 0;
    for claude in [ClaudeAt::ExecPath, ClaudeAt::OnPath, ClaudeAt::Both, ClaudeAt::Neither] {
        for codex in [true, false] {
            for cursor in [
                CursorBy::Folder,
                CursorBy::AgentVariable,
                CursorBy::TraceVariable,
                CursorBy::Nothing,
            ] {
                let (home, bin) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
                let mut vars = Vec::new();
                let mut want = Vec::new();
                if matches!(claude, ClaudeAt::ExecPath | ClaudeAt::Both) {
                    vars.push(("CLAUDE_CODE_EXECPATH", named.to_str().unwrap()));
                }
                let on_path_claude = matches!(claude, ClaudeAt::OnPath | ClaudeAt::Both)
                    .then(|| script(bin.path(), "claude", "exit 0"));
                match claude {
                    ClaudeAt::ExecPath | ClaudeAt::Both => want.push(Agent::ClaudeCode(named.clone())),
                    ClaudeAt::OnPath => want.push(Agent::ClaudeCode(on_path_claude.unwrap())),
                    ClaudeAt::Neither => {}
                }
                if codex {
                    want.push(Agent::Codex(script(bin.path(), "codex", "exit 0")));
                }
                match cursor {
                    CursorBy::Folder => std::fs::create_dir(home.path().join(".cursor")).unwrap(),
                    CursorBy::AgentVariable => vars.push(("CURSOR_AGENT", "1")),
                    CursorBy::TraceVariable => vars.push(("CURSOR_TRACE_ID", "a1b2")),
                    CursorBy::Nothing => {}
                }
                if !matches!(cursor, CursorBy::Nothing) {
                    want.push(Agent::Cursor);
                }
                let path = std::env::join_paths([bin.path()]).unwrap();
                let got = found(&env_of(&vars), &|n| on_path(n, Some(&path), false), home.path());
                assert_eq!(got, want, "Claude Code {claude:?}, Codex {codex}, Cursor {cursor:?}");
                rows += 1;
            }
        }
    }
    assert_eq!(rows, 32);
}

/// A path to `to` relative to this process's folder, as a PATH entry can be.
#[cfg(unix)]
fn relative_to_here(to: &Path) -> PathBuf {
    let here = std::env::current_dir().unwrap();
    let up = here.components().count() - 1;
    let mut relative: PathBuf = std::iter::repeat("..").take(up).collect();
    relative.push(to.strip_prefix("/").unwrap());
    assert!(relative.is_relative() && relative.join("x").parent().unwrap().is_dir());
    relative
}

#[cfg(unix)]
#[test]
fn awkward_path_entries_are_skipped_and_the_first_real_program_wins() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let base = tempfile::tempdir().unwrap();
    let dir = |name: &str| {
        let d = base.path().join(name);
        std::fs::create_dir_all(&d).unwrap();
        d
    };
    let first = dir("first");
    let real = script(&dir("real"), "claude", "exit 0");
    let program = |at: &Path| script(at, "claude", "exit 0");
    let path = |entries: &[&std::ffi::OsStr]| -> OsString {
        entries.iter().map(|e| e.to_os_string()).collect::<Vec<_>>().join(OsStr::new(":"))
    };
    let real_dir = real.parent().unwrap().as_os_str();

    // (row, PATH, what it finds)
    let mut rows: Vec<(&str, OsString, Option<PathBuf>)> = vec![
        ("empty entries", path(&["".as_ref(), "".as_ref(), real_dir, "".as_ref()]), Some(real.clone())),
        (
            "a missing folder first",
            path(&[base.path().join("missing").as_os_str(), real_dir]),
            Some(real.clone()),
        ),
        (
            "a relative entry, which names a folder in the project",
            path(&[relative_to_here(real.parent().unwrap()).as_os_str()]),
            None,
        ),
        ("only an empty entry", OsString::new(), None),
    ];
    let twice = program(&dir("twice"));
    rows.push((
        "duplicate entries",
        path(&[twice.parent().unwrap().as_os_str(), twice.parent().unwrap().as_os_str(), real_dir]),
        Some(twice.clone()),
    ));
    let folder = dir("folder");
    std::fs::create_dir(folder.join("claude")).unwrap();
    rows.push(("a folder named claude", path(&[folder.as_os_str(), real_dir]), Some(real.clone())));
    rows.push(("only a folder named claude", path(&[folder.as_os_str()]), None));
    let plain = dir("plain");
    std::fs::write(plain.join("claude"), "#!/bin/sh\n").unwrap();
    std::fs::set_permissions(plain.join("claude"), std::fs::Permissions::from_mode(0o644)).unwrap();
    rows.push(("a claude without the execute bit", path(&[plain.as_os_str(), real_dir]), Some(real.clone())));
    rows.push(("only a claude without the execute bit", path(&[plain.as_os_str()]), None));
    let linked = dir("linked");
    symlink(&real, linked.join("claude")).unwrap();
    rows.push(("a symlink to a program", path(&[linked.as_os_str()]), Some(linked.join("claude"))));
    let broken = dir("broken");
    symlink(base.path().join("gone"), broken.join("claude")).unwrap();
    rows.push(("a broken symlink", path(&[broken.as_os_str(), real_dir]), Some(real.clone())));
    rows.push(("only a broken symlink", path(&[broken.as_os_str()]), None));
    let odd = program(&dir("my agents (ü 日本)"));
    rows.push((
        "a folder with spaces and non-ASCII letters",
        path(&[first.as_os_str(), odd.parent().unwrap().as_os_str()]),
        Some(odd.clone()),
    ));

    for (row, path, want) in rows {
        assert_eq!(on_path("claude", Some(&path), false), want, "{row}: PATH={path:?}");
    }
}

#[cfg(unix)]
#[test]
fn claude_code_execpath_is_used_only_when_it_names_a_claude_file() {
    let home = tempfile::tempdir().unwrap();
    let base = tempfile::tempdir().unwrap();
    std::fs::create_dir(base.path().join("bin")).unwrap();
    let on_path_claude = script(&base.path().join("bin"), "claude", "exit 0");
    let bin = std::env::join_paths([on_path_claude.parent().unwrap()]).unwrap();
    let odd = base.path().join("Claude Code (ü)");
    std::fs::create_dir_all(&odd).unwrap();
    let odd = script(&odd, "claude", "exit 0");
    let named_folder = base.path().join("folder").join("claude");
    std::fs::create_dir_all(&named_folder).unwrap();
    let node = script(base.path(), "node", "exit 0");
    let bun = script(base.path(), "bun", "exit 0");
    let missing = base.path().join("gone").join("claude");
    // Real files, named from this process's folder: a program started from
    // the home folder, or on Windows from the project, would not be them.
    let relative = relative_to_here(odd.parent().unwrap()).join("claude");
    let dot_relative = Path::new(".").join(&relative);

    // (row, CLAUDE_CODE_EXECPATH, the program it names when it counts)
    let rows: [(&str, &Path, Option<&Path>); 9] = [
        ("node", &node, None),
        ("bun", &bun, None),
        ("a missing file", &missing, None),
        ("a folder named claude", &named_folder, None),
        ("an empty value", Path::new(""), None),
        ("a claude in a folder with spaces and non-ASCII letters", &odd, Some(&odd)),
        ("a relative path to a claude", &relative, None),
        ("a relative path to a claude starting with ./", &dot_relative, None),
        ("a bare claude", Path::new("claude"), None),
    ];
    for (row, value, named) in rows {
        let env = env_of(&[("CLAUDE_CODE_EXECPATH", value.to_str().unwrap())]);
        let with_path = found(&env, &|n| on_path(n, Some(&bin), false), home.path());
        let want = named.unwrap_or(&on_path_claude).to_path_buf();
        assert_eq!(with_path, vec![Agent::ClaudeCode(want)], "{row}, with claude on PATH");
        let alone = found(&env, &|_| None, home.path());
        let want: Vec<Agent> = named.map(|p| Agent::ClaudeCode(p.to_path_buf())).into_iter().collect();
        assert_eq!(alone, want, "{row}, with nothing on PATH");
    }
}

// ── installing, as a table and as a randomized run ──────────────────

/// The six commands, in the order `install` runs them.
const COMMANDS: [&str; 6] = [
    "claude plugin marketplace update elevenlabs",
    "claude plugin install elevenlabs --marketplace elevenlabs/plugin --scope user",
    "claude plugin update elevenlabs@elevenlabs",
    "codex plugin marketplace add elevenlabs/plugin",
    "codex plugin marketplace upgrade elevenlabs",
    "codex plugin add elevenlabs@elevenlabs",
];

const INSTALLED: &str = "installed the ElevenLabs plugin (skills and the hosted MCP server)";

#[test]
fn each_failing_command_stops_its_own_agent_and_no_other() {
    let claude = Agent::ClaudeCode("claude".into());
    let codex = Agent::Codex("codex".into());
    let both = [claude.clone(), codex.clone(), Agent::Cursor];
    // (agents, the command that fails, the commands that ran, each agent's result)
    let rows: [(&[Agent], usize, &[usize], &[bool]); 12] = [
        (&[claude.clone()], 0, &[0, 1, 2], &[true]),
        (&[claude.clone()], 1, &[0, 1], &[false]),
        (&[claude.clone()], 2, &[0, 1, 2], &[false]),
        (&[codex.clone()], 3, &[3], &[false]),
        (&[codex.clone()], 4, &[3, 4, 5], &[true]),
        (&[codex.clone()], 5, &[3, 4, 5], &[false]),
        (&both, 0, &[0, 1, 2, 3, 4, 5], &[true, true, true]),
        (&both, 1, &[0, 1, 3, 4, 5], &[false, true, true]),
        (&both, 2, &[0, 1, 2, 3, 4, 5], &[false, true, true]),
        (&both, 3, &[0, 1, 2, 3], &[true, false, true]),
        (&both, 4, &[0, 1, 2, 3, 4, 5], &[true, true, true]),
        (&both, 5, &[0, 1, 2, 3, 4, 5], &[true, false, true]),
    ];
    for (agents, fails, ran, ok) in rows {
        let row = format!("{:?} with `{}` failing", agents.iter().map(Agent::name).collect::<Vec<_>>(), COMMANDS[fails]);
        let recorder = Recorder::exactly(COMMANDS[fails]);
        let outcomes = install(agents, LIMITS_FOR_TESTS, &|p, a, l| recorder.run(p, a, l));
        let want: Vec<String> = ran.iter().map(|&i| COMMANDS[i].to_string()).collect();
        assert_eq!(recorder.ran(), want, "{row}");
        let want: Vec<Outcome> = agents
            .iter()
            .zip(ok)
            .map(|(agent, &ok)| Outcome {
                agent: agent.name(),
                ok,
                message: match (agent, ok) {
                    (Agent::Cursor, _) => format!("open {CURSOR_PAGE} and select Install"),
                    (_, true) => INSTALLED.into(),
                    (_, false) => format!("`{}` exited with 1: boom", COMMANDS[fails]),
                },
            })
            .collect();
        assert_eq!(outcomes, want, "{row}");
    }
}

/// xorshift64*: enough randomness for a test, with no crate.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// One command a randomized run started: which, when, and with how long.
#[derive(Debug)]
struct Call {
    line: String,
    at: Duration,
    limit: Duration,
    failed: bool,
}

#[test]
fn install_keeps_its_rules_under_2000_random_failure_patterns() {
    const SEED: u64 = 0x5EED_0F_E1E7_E2;
    const PATTERNS: usize = 2000;
    let mut rng = Rng(SEED);
    for pattern in 0..PATTERNS {
        let mut agents = vec![
            Agent::ClaudeCode("claude".into()),
            Agent::Codex("codex".into()),
            Agent::Cursor,
        ];
        for i in (1..agents.len()).rev() {
            agents.swap(i, rng.below(i as u64 + 1) as usize);
        }
        agents.truncate(rng.below(4) as usize);
        let limits = Limits {
            command: Duration::from_millis(1 + rng.below(30)),
            total: Duration::from_millis(1 + rng.below(150)),
        };
        let rng = RefCell::new(Rng(rng.next() | 1));
        let clock = std::cell::Cell::new(Duration::ZERO);
        let calls = RefCell::new(Vec::<Call>::new());
        let outcomes = install_timed(
            &agents,
            limits,
            &|program, args, limit| {
                let mut rng = rng.borrow_mut();
                // Never longer than its limit: past it, a command is stopped.
                let took = Duration::from_millis(rng.below(limit.as_millis() as u64 + 1));
                let failed = took == limit || rng.below(10) < 3;
                calls.borrow_mut().push(Call {
                    line: format!("{} {}", program.display(), args.join(" ")),
                    at: clock.get(),
                    limit,
                    failed,
                });
                clock.set(clock.get() + took);
                if failed {
                    Err("failed".into())
                } else {
                    Ok(())
                }
            },
            &|| clock.get(),
        );
        let why = format!("seed {SEED:#x}, pattern {pattern}: {agents:?} {limits:?}");
        let calls = calls.into_inner();
        assert_eq!(outcomes.len(), agents.len(), "{why}");
        assert!(clock.get() <= limits.total, "{why}: the run took {:?}", clock.get());
        let mut next = 0;
        for (agent, outcome) in agents.iter().zip(&outcomes) {
            assert_eq!(outcome.agent, agent.name(), "{why}");
            let planned: Vec<String> = COMMANDS
                .iter()
                .filter(|c| match agent {
                    Agent::ClaudeCode(_) => c.starts_with("claude "),
                    Agent::Codex(_) => c.starts_with("codex "),
                    Agent::Cursor => false,
                })
                .map(|c| c.to_string())
                .collect();
            // This agent's commands come next, a prefix of its plan in order.
            let mine: Vec<&Call> = calls[next..]
                .iter()
                .take_while(|c| planned.contains(&c.line))
                .collect();
            next += mine.len();
            let lines: Vec<&String> = mine.iter().map(|c| &c.line).collect();
            assert_eq!(lines, planned.iter().take(mine.len()).collect::<Vec<_>>(), "{why}");
            for call in &mine {
                assert!(
                    !call.limit.is_zero()
                        && call.limit <= limits.command
                        && call.at + call.limit <= limits.total,
                    "{why}: {call:?}"
                );
            }
            let decides = |line: &str| {
                agent.steps().iter().any(|(args, decides)| *decides && line.ends_with(&args.join(" ")))
            };
            let deciding_failure = mine.iter().position(|c| c.failed && decides(&c.line));
            if let Some(at) = deciding_failure {
                assert_eq!(at + 1, mine.len(), "{why}: nothing runs after a deciding command fails");
            }
            let finished = mine.len() == planned.len();
            if !finished && deciding_failure.is_none() {
                assert!(
                    outcome.message.contains("ran out of time"),
                    "{why}: stopped early only when time ran out: {outcome:?}"
                );
            }
            let every_deciding_command_worked = agent
                .steps()
                .iter()
                .filter(|(_, decides)| *decides)
                .all(|(args, _)| mine.iter().any(|c| c.line.ends_with(&args.join(" ")) && !c.failed));
            let want_ok = matches!(agent, Agent::Cursor) || every_deciding_command_worked;
            assert_eq!(outcome.ok, want_ok, "{why}: {outcome:?} after {lines:?}");
        }
        assert_eq!(next, calls.len(), "{why}: every command belongs to an agent, in their order");
    }
}

// ── the skills copy, against a local server standing in for GitHub ──

/// Everything under `root` but `<root>/.agents/skills`: each path with its
/// kind, its bytes or its link's target, and its permissions. The `.agents`
/// folder that holds it is left out too, so making it counts as no change.
#[cfg(unix)]
fn outside_the_skills(root: &Path) -> std::collections::BTreeMap<PathBuf, String> {
    let mut all = tree(root, &[skills_root(Path::new(""))]);
    if all.get(Path::new(".agents")).is_some_and(|what| what.starts_with("folder")) {
        all.remove(Path::new(".agents"));
    }
    all
}

/// Every path under `root` but those under `skip`, with what it holds.
#[cfg(unix)]
fn tree(root: &Path, skip: &[PathBuf]) -> std::collections::BTreeMap<PathBuf, String> {
    use std::os::unix::fs::PermissionsExt;
    fn walk(root: &Path, at: &Path, skip: &[PathBuf], out: &mut std::collections::BTreeMap<PathBuf, String>) {
        for entry in std::fs::read_dir(at).unwrap().flatten() {
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            if skip.contains(&relative) {
                continue;
            }
            let meta = std::fs::symlink_metadata(&path).unwrap();
            let mode = meta.permissions().mode() & 0o7777;
            let what = if meta.file_type().is_symlink() {
                format!("link to {}", std::fs::read_link(&path).unwrap().display())
            } else if meta.is_dir() {
                format!("folder {mode:o}")
            } else {
                format!("file {mode:o} {:?}", std::fs::read(&path).unwrap())
            };
            out.insert(relative, what);
            if meta.is_dir() && !meta.file_type().is_symlink() {
                walk(root, &path, skip, out);
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    walk(root, root, skip, &mut out);
    out
}

/// A `.tar.gz` of `files` (path and text; a path ending in `/` is a folder,
/// text starting with `->` a symlink) under `skills-main/`, made with the
/// system's tar as GitHub's archive is laid out.
#[cfg(unix)]
fn archive(files: &[(&str, &str)]) -> Vec<u8> {
    let dir = tempfile::tempdir().unwrap();
    let top = dir.path().join("skills-main");
    std::fs::create_dir(&top).unwrap();
    for (path, text) in files {
        let at = top.join(path);
        std::fs::create_dir_all(if path.ends_with('/') { &at } else { at.parent().unwrap() }).unwrap();
        if let Some(target) = text.strip_prefix("->") {
            std::os::unix::fs::symlink(target, &at).unwrap();
        } else if !path.ends_with('/') {
            std::fs::write(&at, text).unwrap();
        }
    }
    let out = dir.path().join("skills.tar.gz");
    let status = Command::new("tar")
        .arg("-czf")
        .arg(&out)
        .arg("-C")
        .arg(dir.path())
        .arg("skills-main")
        .status()
        .unwrap();
    assert!(status.success());
    std::fs::read(out).unwrap()
}

#[cfg(unix)]
const TWO_SKILLS: &[(&str, &str)] = &[
    ("music/SKILL.md", "# music"),
    ("music/references/api.md", "the api"),
    ("text-to-speech/SKILL.md", "# tts"),
    ("README.md", "readme"),
    ("evals/cases/a.json", "{}"),
];

/// A server standing in for GitHub that answers `GET /<name>` with each of
/// `answers`, and the URL of the first.
#[cfg(unix)]
async fn github(answers: Vec<(&str, wiremock::ResponseTemplate)>) -> (wiremock::MockServer, String) {
    let server = wiremock::MockServer::start().await;
    let first = format!("{}/{}", server.uri(), answers[0].0);
    for (name, answer) in answers {
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path(format!("/{name}")))
            .respond_with(answer)
            .mount(&server)
            .await;
    }
    (server, first)
}

#[cfg(unix)]
fn body(bytes: Vec<u8>) -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(200).set_body_bytes(bytes)
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn the_skills_download_is_copied_or_fails_with_what_went_wrong() {
    let good = archive(TWO_SKILLS);
    let truncated = good[..good.len() / 2].to_vec();
    let odd = archive(&[
        ("music/SKILL.md", "# music"),
        ("music/hosts", "->/etc/hosts"),
        ("music/up", "->../../outside"),
        ("linked", "->music"),
        ("notes", "a file where a skill folder would be"),
        ("weird/SKILL.md/", ""),
        ("weird/other.md", "no SKILL.md file"),
    ]);
    let none = archive(&[("README.md", "readme"), ("evals/a.json", "{}")]);
    let copied = |n: usize| move |root: &Path| format!("copied {n} ElevenLabs skills into {}", root.display());
    type Want = Box<dyn Fn(&str, &Path) -> (bool, String)>;
    let ok = |n: usize| -> Want { Box::new(move |_, root| (true, copied(n)(root))) };
    let starts = |ok: bool, prefix: &'static str| -> Want {
        Box::new(move |url, _| (ok, prefix.replace("{url}", url)))
    };
    // (row, how the server answers, the outcome, the skill folders left)
    let rows: Vec<(&str, Vec<(&str, wiremock::ResponseTemplate)>, Want, &[&str])> = vec![
        ("a good archive", vec![("main.tar.gz", body(good.clone()))], ok(2), &["music", "text-to-speech"]),
        (
            "a redirect to it",
            vec![
                ("archive", wiremock::ResponseTemplate::new(302).insert_header("Location", "/codeload.tar.gz")),
                ("codeload.tar.gz", body(good.clone())),
            ],
            ok(2),
            &["music", "text-to-speech"],
        ),
        (
            "HTTP 404",
            vec![("main.tar.gz", wiremock::ResponseTemplate::new(404))],
            starts(false, "couldn't download the skills from {url}: HTTP status client error (404 Not Found)"),
            &[],
        ),
        (
            "HTTP 500",
            vec![("main.tar.gz", wiremock::ResponseTemplate::new(500))],
            starts(false, "couldn't download the skills from {url}: HTTP status server error (500 Internal Server Error)"),
            &[],
        ),
        (
            "a body that is not gzip",
            vec![("main.tar.gz", body(b"<html>not here</html>".to_vec()))],
            starts(false, "tar couldn't unpack the skills: "),
            &[],
        ),
        (
            "a truncated archive",
            vec![("main.tar.gz", body(truncated))],
            starts(false, "tar couldn't unpack the skills: "),
            &[],
        ),
        (
            "an archive with no skills",
            vec![("main.tar.gz", body(none))],
            starts(false, "the download from {url} held no skills"),
            &[],
        ),
        (
            "links, a file and a SKILL.md folder where skills would be",
            vec![("main.tar.gz", body(odd))],
            ok(1),
            &["music"],
        ),
    ];
    for (row, answers, want, left) in rows {
        let (_server, url) = github(answers).await;
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join(".profile"), "mine").unwrap();
        let before = outside_the_skills(home.path());
        let outcome = skills_from(&url, home.path());
        let root = skills_root(home.path());
        let (ok, message) = want(&url, &root);
        assert_eq!(outcome.ok, ok, "{row}: {outcome:?}");
        assert!(outcome.message.starts_with(&message), "{row}: {:?} should start {message:?}", outcome.message);
        assert_eq!(outside_the_skills(home.path()), before, "{row}: nothing else in the home folder changed");
        let mut there: Vec<String> = std::fs::read_dir(&root)
            .map(|d| d.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect())
            .unwrap_or_default();
        there.sort();
        assert_eq!(there, left, "{row}");
        if left.contains(&"music") {
            let music = tree(&root.join("music"), &[]);
            let names: Vec<String> = music.keys().map(|p| p.display().to_string()).collect();
            let want: &[&str] = if row.starts_with("links") {
                &[MARKER, "SKILL.md"]
            } else {
                &[MARKER, "SKILL.md", "references", "references/api.md"]
            };
            assert_eq!(names, want, "{row}: no link is followed or copied");
        }
    }
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn the_skills_land_whatever_the_home_folder_already_holds() {
    use std::os::unix::fs::PermissionsExt;
    let (_server, url) = github(vec![("main.tar.gz", body(archive(TWO_SKILLS)))]).await;
    type Prepare = fn(&Path);
    // (row, the home folder before, ok, what the message says after the root)
    let rows: [(&str, Prepare, bool, &str); 7] = [
        ("a fresh home folder", |_| {}, true, ""),
        (
            "an .agents folder holding other things",
            |h| {
                std::fs::create_dir_all(h.join(".agents/other")).unwrap();
                std::fs::write(h.join(".agents/other/keep.md"), "keep").unwrap();
            },
            true,
            "",
        ),
        (
            "a skill folder setup wrote",
            |h| {
                let music = skills_root(h).join("music");
                std::fs::create_dir_all(&music).unwrap();
                std::fs::write(music.join(MARKER), "").unwrap();
                std::fs::write(music.join("stale.md"), "old").unwrap();
            },
            true,
            "",
        ),
        (
            "a skill folder of the developer's own",
            |h| {
                let music = skills_root(h).join("music");
                std::fs::create_dir_all(&music).unwrap();
                std::fs::write(music.join("SKILL.md"), "mine").unwrap();
            },
            true,
            "; left alone, since setup didn't write them: music",
        ),
        (
            "a skill folder that is a symlink to a marked folder",
            |h| {
                let elsewhere = h.join("elsewhere");
                std::fs::create_dir_all(&elsewhere).unwrap();
                std::fs::write(elsewhere.join(MARKER), "").unwrap();
                std::fs::create_dir_all(skills_root(h)).unwrap();
                std::os::unix::fs::symlink(&elsewhere, skills_root(h).join("music")).unwrap();
            },
            true,
            "; left alone, since setup didn't write them: music",
        ),
        (
            "an .agents that is a file",
            |h| std::fs::write(h.join(".agents"), "not a folder").unwrap(),
            false,
            "",
        ),
        (
            "a read-only skill folder setup wrote",
            |h| {
                let music = skills_root(h).join("music");
                std::fs::create_dir_all(&music).unwrap();
                std::fs::write(music.join(MARKER), "").unwrap();
                std::fs::write(music.join("SKILL.md"), "old").unwrap();
                std::fs::set_permissions(&music, std::fs::Permissions::from_mode(0o555)).unwrap();
            },
            false,
            "",
        ),
    ];
    for (row, prepare, ok, kept) in rows {
        let home = tempfile::tempdir().unwrap();
        prepare(home.path());
        let root = skills_root(home.path());
        let before = outside_the_skills(home.path());
        let skills_before = root.is_dir().then(|| tree(&root, &[]));
        let outcome = skills_from(&url, home.path());
        assert_eq!(outcome.ok, ok, "{row}: {outcome:?}");
        assert_eq!(outside_the_skills(home.path()), before, "{row}: nothing outside the skills folder changed");
        if ok {
            let written = if kept.is_empty() { 2 } else { 1 };
            assert_eq!(
                outcome.message,
                format!("copied {written} ElevenLabs skills into {}{kept}", root.display()),
                "{row}"
            );
            let tts = tree(&root.join("text-to-speech"), &[]);
            assert_eq!(tts.len(), 2, "{row}: {tts:?}");
        } else if row.contains("read-only") {
            assert!(outcome.message.starts_with(&root.join("music").display().to_string()), "{row}: {outcome:?}");
            assert_eq!(Some(tree(&root, &[])), skills_before, "{row}: left as it was");
            std::fs::set_permissions(root.join("music"), std::fs::Permissions::from_mode(0o755)).unwrap();
        } else {
            assert!(outcome.message.contains(".agents"), "{row}: {outcome:?}");
        }
        if row.contains("own") || row.contains("symlink") {
            assert_eq!(Some(tree(&root, &[]).into_iter().filter(|(p, _)| p.starts_with("music")).collect()),
                skills_before.map(|t| t.into_iter().filter(|(p, _)| p.starts_with("music")).collect::<std::collections::BTreeMap<_, _>>()),
                "{row}: the folder is left as it was");
        }
    }
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn five_runs_give_byte_identical_skills_with_nested_folders_and_odd_names() {
    let files: &[(&str, &str)] = &[
        ("music/SKILL.md", "# music"),
        ("music/a/b/c/deep.md", "deep"),
        ("music/with space.md", "space"),
        ("music/ünï 日本.md", "unicode"),
        ("music/-dash.md", "dash"),
        ("music/.hidden", "hidden"),
        ("music/empty/", ""),
        ("dubbing/SKILL.md", "# dubbing"),
    ];
    let (_server, url) = github(vec![("main.tar.gz", body(archive(files)))]).await;
    let home = tempfile::tempdir().unwrap();
    let before = outside_the_skills(home.path());
    let root = skills_root(home.path());
    let mut first = None;
    for run in 0..5 {
        let outcome = skills_from(&url, home.path());
        assert!(outcome.ok, "run {run}: {outcome:?}");
        let now = tree(&root, &[]);
        match &first {
            None => first = Some(now),
            Some(first) => assert_eq!(&now, first, "run {run}"),
        }
        assert_eq!(outside_the_skills(home.path()), before, "run {run}");
    }
    // Read back by name: macOS's tar may store a name in another Unicode form.
    for (path, text) in files {
        let at = root.join(path.trim_end_matches('/'));
        if path.ends_with('/') {
            assert!(at.is_dir(), "{path}");
        } else {
            assert_eq!(std::fs::read_to_string(&at).unwrap(), *text, "{path}");
        }
    }
    assert_eq!(first.unwrap().len(), files.len() + 7, "the files, five folders and the two marks");
}

// ── starting a program, against real child processes ───────────────

/// Waits for `file` to hold a pid, polling until a deadline.
#[cfg(unix)]
fn pid_in(file: &Path) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(pid) = std::fs::read_to_string(file).ok().and_then(|t| t.trim().parse().ok()) {
            return pid;
        }
        assert!(Instant::now() < deadline, "no pid in {}", file.display());
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Whether process `pid` is gone, waiting up to a few seconds for it.
#[cfg(unix)]
fn gone(pid: i32) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while unsafe { libc::kill(pid, 0) } == 0 {
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    true
}

/// The processes whose command line contains `marker`.
#[cfg(unix)]
fn left_behind(marker: &str) -> Vec<String> {
    let out = Command::new("ps").args(["-axo", "pid=,command="]).output().unwrap();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| l.contains(marker) && !l.contains("ps -axo"))
        .map(str::to_string)
        .collect()
}

#[cfg(unix)]
#[test]
fn a_program_reports_how_it_ended_whatever_it_prints() {
    let dir = tempfile::tempdir().unwrap();
    let limit = Duration::from_secs(20);
    // (row, the stand-in's script, what run_program returns)
    let rows: [(&str, &str, Result<(), String>); 9] = [
        ("stdin is closed", "read line && exit 1; exit 0", Ok(())),
        (
            "megabytes on stdout and stderr, then success",
            "head -c 4000000 /dev/zero | tr '\\0' o; head -c 4000000 /dev/zero | tr '\\0' e >&2; exit 0",
            Ok(()),
        ),
        (
            "megabytes on stdout and stderr, then a failure",
            "head -c 4000000 /dev/zero | tr '\\0' o; head -c 4000000 /dev/zero | tr '\\0' e >&2; \
             echo >&2; echo 'the real error' >&2; exit 4",
            Err("exited with 4: the real error".into()),
        ),
        (
            "only stdout says why",
            "echo 'on stdout'; exit 5",
            Err("exited with 5: on stdout".into()),
        ),
        ("nothing said", "exit 6", Err("exited with 6".into())),
        (
            "output that is not UTF-8",
            "printf 'caf\\351 \\377 broke\\n' >&2; exit 2",
            Err("exited with 2: caf\u{FFFD} \u{FFFD} broke".into()),
        ),
        ("killed by a signal", "kill -KILL $$", Err("exited with a signal".into())),
        (
            "killed by a signal after a line",
            "echo 'half way' >&2; kill -TERM $$",
            Err("exited with a signal: half way".into()),
        ),
        ("blank lines after the last one", "echo 'last' >&2; echo >&2; echo '   ' >&2; exit 1", Err("exited with 1: last".into())),
    ];
    for (row, body, want) in rows {
        let program = script(dir.path(), "claude", body);
        let started = Instant::now();
        assert_eq!(run_program(&program, &[], limit, dir.path()), want, "{row}");
        assert!(started.elapsed() < Duration::from_secs(15), "{row}: took {:?}", started.elapsed());
    }
}

#[cfg(unix)]
#[test]
fn a_program_that_vanished_or_lost_its_execute_bit_since_it_was_found_fails_to_start() {
    use std::os::unix::fs::PermissionsExt;
    let (dir, home) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let path = std::env::join_paths([dir.path()]).unwrap();
    let program = script(dir.path(), "codex", "exit 0");
    assert_eq!(on_path("codex", Some(&path), false), Some(program.clone()));
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o644)).unwrap();
    let err = run_program(&program, &[], Duration::from_secs(5), home.path()).unwrap_err();
    assert!(err.starts_with("couldn't start it: "), "lost its execute bit: {err}");
    std::fs::remove_file(&program).unwrap();
    let err = run_program(&program, &[], Duration::from_secs(5), home.path()).unwrap_err();
    assert!(err.starts_with("couldn't start it: "), "vanished: {err}");
    let outcomes = install(&[Agent::Codex(program)], LIMITS_FOR_TESTS, &|p, a, l| {
        run_program(p, a, l, home.path())
    });
    assert!(!outcomes[0].ok && outcomes[0].message.contains("couldn't start it"), "{outcomes:?}");
}

#[cfg(unix)]
#[test]
fn a_command_past_its_limit_is_stopped_with_its_children_and_grandchildren() {
    let dir = tempfile::tempdir().unwrap();
    let (child, grandchild) = (dir.path().join("child"), dir.path().join("grandchild"));
    let hangs = script(
        dir.path(),
        "codex",
        &format!(
            "sh -c '{sleep} & echo $! > \"{g}\"; wait' &\necho $! > '{c}'\nwait",
            sleep = sleep_marker(3011),
            g = grandchild.display(),
            c = child.display()
        ),
    );
    let started = Instant::now();
    let limit = Duration::from_millis(1500);
    let result = run_program(&hangs, &[], limit, dir.path());
    let (child, grandchild) = (pid_in(&child), pid_in(&grandchild));
    let _cleanup = (KillOnDrop(child), KillOnDrop(grandchild));
    assert_eq!(result, Err("ran out of time after 1 seconds and was stopped".into()));
    assert!(started.elapsed() < limit + Duration::from_secs(3), "{:?}", started.elapsed());
    assert!(gone(child), "the child {child} still runs");
    assert!(gone(grandchild), "the grandchild {grandchild} still runs");
    assert_eq!(left_behind(&sleep_marker(3011)), Vec::<String>::new());
    assert_eq!(left_behind(&hangs.display().to_string()), Vec::<String>::new());
}

#[cfg(unix)]
#[test]
fn fifty_runs_in_a_row_leave_no_process_behind() {
    let dir = tempfile::tempdir().unwrap();
    let works = script(dir.path(), "works", "echo fine");
    let fails = script(dir.path(), "fails", "echo broke >&2; exit 1");
    let sleep = sleep_marker(3012);
    let hangs = script(dir.path(), "hangs", &format!("sh -c '{sleep}' &\n{sleep} &\nwait"));
    for run in 0..50 {
        let (program, limit, want) = match run % 3 {
            0 => (&works, Duration::from_secs(10), Ok(())),
            1 => (&fails, Duration::from_secs(10), Err("exited with 1: broke".to_string())),
            _ => (&hangs, Duration::from_millis(100), Err("ran out of time after 100 ms and was stopped".to_string())),
        };
        assert_eq!(run_program(program, &[], limit, dir.path()), want, "run {run}");
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let mut left = left_behind(&sleep);
        left.extend(left_behind(&dir.path().display().to_string()));
        if left.is_empty() {
            break;
        }
        assert!(Instant::now() < deadline, "left behind: {left:?}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

// ── the error line a failed command leaves ──────────────────────────

#[test]
fn the_error_line_is_the_last_with_text_without_escapes_and_at_most_300_characters() {
    let long = "x".repeat(5_000_000);
    let cut = format!("{}…", "x".repeat(LINE_MAX - 1));
    // A two-byte character where a cut by bytes would split it.
    let straddles = format!("{}éz", "a".repeat(LINE_MAX - 1));
    let straddles_cut = format!("{}…", "a".repeat(LINE_MAX - 1));
    let exactly = "b".repeat(LINE_MAX);
    // (row, the output, the line)
    let rows: Vec<(&str, Vec<u8>, Option<String>)> = vec![
        ("5 MB with no newline", long.clone().into_bytes(), Some(cut.clone())),
        ("megabytes, then a short last line", format!("{long}\nthe real error\n").into_bytes(), Some("the real error".into())),
        (
            "a progress bar, then a coloured error",
            b"downloading  10%\rdownloading  50%\rdownloading 100%\r\n\x1b[31merror:\x1b[0m \x1b[1mno such plugin\x1b[22m\r\n".to_vec(),
            Some("error: no such plugin".into()),
        ),
        ("a progress bar redrawn last", b"done one\nstep 1/3\rstep 2/3\rstep 3/3".to_vec(), Some("step 3/3".into())),
        ("a window title and a link", b"\x1b]0;claude\x07\x1b]8;;https://x.test\x1b\\link\x1b]8;;\x1b\\ failed".to_vec(), Some("link failed".into())),
        ("a tab and a bell", b"one\ttwo\x07\x08three".to_vec(), Some("one twothree".into())),
        ("a multi-byte character straddling the cut", straddles.into_bytes(), Some(straddles_cut)),
        ("exactly 300 characters", exactly.clone().into_bytes(), Some(exactly)),
        ("bytes that are not UTF-8", b"caf\xe9 \xff broke\n".to_vec(), Some("caf\u{FFFD} \u{FFFD} broke".into())),
        ("empty output", Vec::new(), None),
        ("only blank lines and escape codes", b"\n  \r\n\x1b[0m\x1b[2K\r\x1b]0;t\x07\n\t\n".to_vec(), None),
    ];
    for (row, output, want) in rows {
        let got = clean_last_line(&output);
        assert_eq!(got, want, "{row}");
        let got = got.unwrap_or_default();
        assert!(got.chars().count() <= LINE_MAX, "{row}: {} characters", got.chars().count());
        assert!(!got.chars().any(char::is_control), "{row}: {got:?}");
    }
}

#[test]
fn only_the_tail_of_a_capture_is_read_and_a_character_cut_by_it_is_dropped() {
    use std::io::Write;
    let mut file = tempfile::tempfile().unwrap();
    // 10 MB, then a last line whose first character is cut by the tail.
    file.write_all(&vec![b'z'; 10_000_000]).unwrap();
    file.write_all(b"\n").unwrap();
    let mut line = "é".to_string();
    line.push_str(&"q".repeat(TAIL as usize - 1));
    file.write_all(line.as_bytes()).unwrap();
    let got = last_line(file).unwrap();
    assert_eq!(got, format!("{}…", "q".repeat(LINE_MAX - 1)));
}

#[cfg(unix)]
#[test]
fn a_failed_command_reports_a_short_clean_line_whatever_it_printed() {
    let dir = tempfile::tempdir().unwrap();
    // (row, the stand-in's script, what run_program returns)
    let rows: [(&str, &str, String); 3] = [
        (
            "5 MB on stderr with no newline",
            "head -c 5000000 /dev/zero | tr '\\0' e >&2; exit 1",
            format!("exited with 1: {}…", "e".repeat(LINE_MAX - 1)),
        ),
        (
            "megabytes, then a short last line",
            "head -c 5000000 /dev/zero | tr '\\0' e >&2; printf '\\nthe real error\\n' >&2; exit 1",
            "exited with 1: the real error".into(),
        ),
        (
            "a progress bar, then a coloured error",
            "i=0; while [ $i -lt 2000 ]; do printf 'step %d\\r' $i; i=$((i+1)); done; \
             printf '\\033[31merror:\\033[0m it broke\\r\\n' >&2; exit 2",
            "exited with 2: error: it broke".into(),
        ),
    ];
    for (row, body, want) in rows {
        let program = script(dir.path(), "claude", body);
        assert_eq!(run_program(&program, &[], Duration::from_secs(20), dir.path()), Err(want), "{row}");
    }
}

#[cfg(unix)]
#[test]
fn a_missing_home_folder_is_named_and_the_agent_is_not_blamed() {
    let dir = tempfile::tempdir().unwrap();
    let program = script(dir.path(), "claude", "exit 0");
    let home = dir.path().join("gone");
    assert_eq!(
        run_program(&program, &[], Duration::from_secs(5), &home),
        Err(format!("couldn't start it: the home folder {} is missing", home.display()))
    );
}

// ── setup --yes with no agent found ─────────────────────────────────

/// `setup --yes` with no agent found, the skills downloaded from a server
/// standing in for GitHub: what it prints and whether it fails.
#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn with_no_agent_found_setup_prints_what_the_skills_copy_did() {
    let empty = archive(&[("README.md", "readme")]);
    // (row, how the server answers, whether it fails, what it prints)
    let rows: [(&str, wiremock::ResponseTemplate, bool, &str); 3] = [
        ("a good archive", body(archive(TWO_SKILLS)), false, "Skills: copied 2 ElevenLabs skills into {root}\n"),
        (
            "an archive with no skills",
            body(empty),
            true,
            "Skills: failed: the download from {url} held no skills\n\
             To try again, run `elevenlabs setup --yes`.\n",
        ),
        (
            "a server error",
            wiremock::ResponseTemplate::new(500),
            true,
            "Skills: failed: couldn't download the skills from {url}: HTTP status server error (500 Internal \
             Server Error) for url ({url})\nTo try again, run `elevenlabs setup --yes`.\n",
        ),
    ];
    for (row, answer, fails, says) in rows {
        let (_server, url) = github(vec![("main.tar.gz", answer)]).await;
        let home = tempfile::tempdir().unwrap();
        let root = skills_root(home.path());
        let (text, failed) = report(&run_from(&[], home.path(), LIMITS_FOR_TESTS, &url));
        assert_eq!(failed, fails, "{row}");
        let says = says.replace("{root}", &root.display().to_string()).replace("{url}", &url);
        assert_eq!(text, says, "{row}");
    }
}
