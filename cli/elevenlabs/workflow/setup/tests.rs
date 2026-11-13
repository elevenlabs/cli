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

/// Runs nothing; records each command and fails those in `fails`.
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
        match self.fails.contains(&line.as_str()) {
            true => Err("exited with 1: boom".into()),
            false => Ok(()),
        }
    }

    fn ran(&self) -> Vec<String> {
        self.ran.borrow().clone()
    }

    /// [`install`] with this recorder, the time used never counting.
    fn install(&self, agents: &[Agent]) -> Vec<Outcome> {
        install(agents, LIMITS_FOR_TESTS, &|p, a, l| self.run(p, a, l), &|| Duration::ZERO)
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
fn install_runs_each_agents_own_commands_in_order_and_says_how_to_finish() {
    let recorder = Recorder::new(&[]);
    let outcomes = recorder.install(&all_agents());
    assert_eq!(
        recorder.ran(),
        [
            "claude plugin marketplace add elevenlabs/plugin",
            "claude plugin marketplace update elevenlabs",
            "claude plugin install elevenlabs@elevenlabs",
            "claude plugin update elevenlabs@elevenlabs",
            "codex plugin marketplace add elevenlabs/plugin",
            "codex plugin marketplace upgrade elevenlabs",
            "codex plugin add elevenlabs@elevenlabs",
        ]
    );
    assert_eq!(
        report(&outcomes),
        (
            vec![
                "Claude Code: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, \
                 restart Claude Code, then run /mcp and sign in to the ElevenLabs MCP server."
                    .to_string(),
                "Codex: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, restart \
                 Codex, then run `codex mcp login elevenlabs` in a terminal to sign in to the ElevenLabs MCP server."
                    .to_string(),
                "Cursor: open https://cursor.com/marketplace/elevenlabs and select Install.".to_string(),
            ],
            false
        )
    );

    let recorder = Recorder::new(&["claude plugin install elevenlabs@elevenlabs"]);
    let outcomes = recorder.install(&all_agents()[..2]);
    assert_eq!(
        report(&outcomes),
        (
            vec![
                "Claude Code: failed: `claude plugin install elevenlabs@elevenlabs` exited with 1: boom."
                    .to_string(),
                "Codex: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, restart \
                 Codex, then run `codex mcp login elevenlabs` in a terminal to sign in to the ElevenLabs MCP server."
                    .to_string(),
                "To try again, run `elevenlabs setup --yes`.".to_string(),
            ],
            true
        ),
        "no finishing step for the agent that failed"
    );
    assert_eq!(
        report(&[]),
        (
            vec![
                "No Claude Code, Codex or Cursor found. If you use Claude Code in the Claude desktop app, ask it \
                 to run \"elevenlabs setup --yes\" for you. For other coding agents, install the ElevenLabs \
                 skills with: npx skills add elevenlabs/skills"
                    .to_string()
            ],
            false
        )
    );
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
    let outcomes = install(&all_agents(), limits, &slow, &|| used.get());
    assert_eq!(recorder.ran().len(), 2, "{:?}", recorder.ran());
    for outcome in &outcomes[..2] {
        assert_eq!(
            (outcome.ok, outcome.message.as_str()),
            (false, "ran out of time: setup stops after 50s"),
            "{outcomes:?}"
        );
    }
    assert!(outcomes[2].ok, "Cursor runs nothing");
}

/// A fraction of a second written as digits before `s`, such as `12.6s`.
fn has_fractional_seconds(text: &str) -> bool {
    text.split('.').skip(1).any(|rest| {
        let after = rest.trim_start_matches(|c: char| c.is_ascii_digit());
        after.len() < rest.len() && after.starts_with('s')
    })
}

#[test]
fn a_limit_cut_short_reads_as_whole_seconds() {
    // (the limit, as shown): the whole run's cap shortens a command's limit,
    // and `onboard setup` takes its account request's time off the run's.
    let rows = [
        (Duration::from_nanos(12_642_639_792), "13s"),
        (Duration::from_nanos(99_999_912_542), "100s"),
        (Duration::from_nanos(99_873_421_000), "100s"),
        (Duration::from_millis(1_499), "1s"),
        (Duration::from_secs(50), "50s"),
        (Duration::from_millis(999), "999ms"),
        (Duration::from_millis(200), "200ms"),
    ];
    for (total, shown) in rows {
        let limits = Limits { command: Duration::from_secs(30), total };
        let outcomes = install(&all_agents()[..1], limits, &|_, _, _| Ok(()), &|| total);
        assert_eq!(outcomes[0].message, format!("ran out of time: setup stops after {shown}"));
    }
    // `onboard setup`'s cap, as onboard/mod.rs makes it.
    let total = LIMITS.total.saturating_sub(Duration::from_nanos(126_578_458));
    let limits = Limits { total, ..LIMITS };
    let outcomes = install(&all_agents()[1..2], limits, &|_, _, _| Ok(()), &|| LIMITS.total);
    let (said, _) = report(&outcomes);
    assert_eq!(said[0], "Codex: failed: ran out of time: setup stops after 100s.");
    assert!(!has_fractional_seconds(&said.join(" ")), "{said:?}");
    assert!(has_fractional_seconds("after 12.642639792s"), "the check finds what it looks for");
}

#[test]
fn a_result_line_ends_in_one_full_stop() {
    let cut = format!("{}…", "q".repeat(LINE_MAX - 1));
    let upgrade = "Codex: failed: `codex plugin marketplace upgrade elevenlabs` exited with 1:";
    // (the error line, what follows `upgrade`): Codex's real one first, as
    // its upgrade and then its add fail.
    let rows = [
        ("Error: 1 upgrade failure(s) occurred.", " Error: 1 upgrade failure(s) occurred."),
        ("Error: no such plugin!", " Error: no such plugin!"),
        ("is the plugin there?", " is the plugin there?"),
        ("error: no such plugin", " error: no such plugin."),
    ];
    for (error, said) in rows {
        let fails = |_: &Path, a: &[&str], _: Duration| match a {
            ["plugin", "marketplace", "upgrade", ..] | ["plugin", "add", ..] => {
                Err(format!("exited with 1: {error}"))
            }
            _ => Ok(()),
        };
        let outcomes = install(&all_agents()[1..2], LIMITS_FOR_TESTS, &fails, &|| Duration::ZERO);
        assert_eq!(report(&outcomes).0[0], format!("{upgrade}{said}"));
    }
    let fails = |_: &Path, _: &[&str], _: Duration| Err(format!("exited with 1: {cut}"));
    let outcomes = install(&all_agents()[1..2], LIMITS_FOR_TESTS, &fails, &|| Duration::ZERO);
    assert_eq!(
        report(&outcomes).0[0],
        format!("Codex: failed: `codex plugin marketplace add elevenlabs/plugin` exited with 1: {cut}"),
        "a line cut at {LINE_MAX} characters ends in `…`"
    );
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
    assert_eq!(err, "ran out of time after 200ms and was stopped");
    let err = run_program(&hangs, &[], Duration::from_millis(1_600), dir.path()).unwrap_err();
    assert_eq!(err, "ran out of time after 2s and was stopped", "not 1.6s");

    let outcomes = install(
        &[Agent::ClaudeCode(hangs), Agent::Cursor],
        Limits {
            command: Duration::from_millis(200),
            total: Duration::from_millis(300),
        },
        &|p, a, l| run_program(p, a, l, dir.path()),
        &|| started.elapsed(),
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
    assert_eq!(question(&[]), None, "nothing to install into");
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
    assert_eq!(
        plan(&[Agent::ClaudeCode("/bin/claude".into()), Agent::Cursor]),
        [
            "Claude Code (/bin/claude): install the ElevenLabs plugin (skills and the hosted MCP server)",
            "Cursor: open https://cursor.com/marketplace/elevenlabs and select Install",
        ]
    );
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
    /// `CURSOR_AGENT` set but empty, which doesn't count.
    EmptyVariable,
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
                CursorBy::EmptyVariable,
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
                    CursorBy::EmptyVariable => vars.push(("CURSOR_AGENT", "")),
                    CursorBy::Nothing => {}
                }
                if !matches!(cursor, CursorBy::EmptyVariable | CursorBy::Nothing) {
                    want.push(Agent::Cursor);
                }
                let path = std::env::join_paths([bin.path()]).unwrap();
                let got = found(&env_of(&vars), &|n| on_path(n, Some(&path), false), home.path());
                assert_eq!(got, want, "Claude Code {claude:?}, Codex {codex}, Cursor {cursor:?}");
                rows += 1;
            }
        }
    }
    assert_eq!(rows, 40);
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
    // Each in a folder of its own, so the names differ even where case doesn't.
    let named = |name: &str| {
        let dir = base.path().join(format!("named {name}"));
        std::fs::create_dir(&dir).unwrap();
        script(&dir, name, "exit 0")
    };
    let (claude_exe, upper) = (named("Claude.exe"), named("CLAUDE"));
    let (node_exe, claude_cmd, desktop) = (named("node.exe"), named("claude.cmd"), named("claude-desktop"));

    // (row, CLAUDE_CODE_EXECPATH, the program it names when it counts)
    let rows: [(&str, &Path, Option<&Path>); 14] = [
        ("Claude.exe, as Windows spells it", &claude_exe, Some(&claude_exe)),
        ("CLAUDE, in capitals", &upper, Some(&upper)),
        ("node", &node, None),
        ("node.exe", &node_exe, None),
        ("bun", &bun, None),
        ("claude.cmd, a shim that would run node", &claude_cmd, None),
        ("claude-desktop", &desktop, None),
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

/// The seven commands, in the order `install` runs them.
const COMMANDS: [&str; 7] = [
    "claude plugin marketplace add elevenlabs/plugin",
    "claude plugin marketplace update elevenlabs",
    "claude plugin install elevenlabs@elevenlabs",
    "claude plugin update elevenlabs@elevenlabs",
    "codex plugin marketplace add elevenlabs/plugin",
    "codex plugin marketplace upgrade elevenlabs",
    "codex plugin add elevenlabs@elevenlabs",
];

/// The commands of [`COMMANDS`] whose result decides their agent's: Claude
/// Code's `plugin install` and `plugin update`, and Codex's `plugin add`.
const DECIDES: [usize; 3] = [2, 3, 6];

const INSTALLED: &str = "installed the ElevenLabs plugin (skills and the hosted MCP server)";

#[test]
fn each_failing_command_stops_its_own_agent_and_no_other() {
    let claude = Agent::ClaudeCode("claude".into());
    let codex = Agent::Codex("codex".into());
    let both = [claude.clone(), codex.clone(), Agent::Cursor];
    // (agents, the commands that fail, the commands that ran, each agent's
    // result: `None` for success, else the command whose error it shows)
    let rows: [(&[Agent], &[usize], &[usize], &[Option<usize>]); 18] = [
        (&[claude.clone()], &[0], &[0, 1, 2, 3], &[None]),
        (&[claude.clone()], &[1], &[0, 1, 2, 3], &[None]),
        (&[claude.clone()], &[0, 1], &[0, 1, 2, 3], &[None]),
        (&[claude.clone()], &[2], &[0, 1, 2], &[Some(2)]),
        (&[claude.clone()], &[3], &[0, 1, 2, 3], &[Some(3)]),
        (&[claude.clone()], &[0, 2], &[0, 1, 2], &[Some(0)]),
        (&[claude.clone()], &[1, 3], &[0, 1, 2, 3], &[Some(1)]),
        (&[codex.clone()], &[4], &[4, 5, 6], &[None]),
        (&[codex.clone()], &[5], &[4, 5, 6], &[None]),
        (&[codex.clone()], &[6], &[4, 5, 6], &[Some(6)]),
        (&[codex.clone()], &[4, 6], &[4, 5, 6], &[Some(4)]),
        (&both, &[], &[0, 1, 2, 3, 4, 5, 6], &[None, None, None]),
        (&both, &[0, 4], &[0, 1, 2, 3, 4, 5, 6], &[None, None, None]),
        (&both, &[2], &[0, 1, 2, 4, 5, 6], &[Some(2), None, None]),
        (&both, &[3], &[0, 1, 2, 3, 4, 5, 6], &[Some(3), None, None]),
        (&both, &[5], &[0, 1, 2, 3, 4, 5, 6], &[None, None, None]),
        (&both, &[6], &[0, 1, 2, 3, 4, 5, 6], &[None, Some(6), None]),
        (&both, &[0, 2, 6], &[0, 1, 2, 4, 5, 6], &[Some(0), Some(6), None]),
    ];
    for (agents, fails, ran, results) in rows {
        let fails: Vec<&str> = fails.iter().map(|&i| COMMANDS[i]).collect();
        let row = format!("{:?} with {fails:?} failing", agents.iter().map(Agent::name).collect::<Vec<_>>());
        let recorder = Recorder::new(&fails);
        let outcomes = recorder.install(agents);
        let want: Vec<String> = ran.iter().map(|&i| COMMANDS[i].to_string()).collect();
        assert_eq!(recorder.ran(), want, "{row}");
        let want: Vec<Outcome> = agents
            .iter()
            .zip(results)
            .map(|(agent, shows)| match (agent, shows) {
                (Agent::Cursor, _) => Outcome {
                    agent: "Cursor",
                    ok: true,
                    message: format!("open {CURSOR_PAGE} and select Install"),
                    finish: None,
                },
                (_, None) => Outcome {
                    agent: agent.name(),
                    ok: true,
                    message: INSTALLED.into(),
                    finish: agent.finish(),
                },
                (_, Some(i)) => Outcome {
                    agent: agent.name(),
                    ok: false,
                    message: format!("`{}` exited with 1: boom", COMMANDS[*i]),
                    finish: None,
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
        let outcomes = install(
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
            let decides = |line: &str| DECIDES.iter().any(|&i| COMMANDS[i] == line);
            let deciding_failure = mine.iter().position(|c| c.failed && decides(&c.line));
            if let Some(at) = deciding_failure {
                assert_eq!(at + 1, mine.len(), "{why}: nothing runs after a deciding command fails");
                let first = mine.iter().find(|c| c.failed).unwrap();
                assert_eq!(outcome.message, format!("`{}` failed", first.line), "{why}: the first failed command shows");
            }
            let finished = mine.len() == planned.len();
            if !finished && deciding_failure.is_none() {
                assert!(
                    outcome.message.contains("ran out of time"),
                    "{why}: stopped early only when time ran out: {outcome:?}"
                );
            }
            let every_deciding_command_worked = planned
                .iter()
                .filter(|c| decides(c))
                .all(|c| mine.iter().any(|m| &m.line == c && !m.failed));
            let want_ok = matches!(agent, Agent::Cursor) || every_deciding_command_worked;
            assert_eq!(outcome.ok, want_ok, "{why}: {outcome:?} after {lines:?}");
        }
        assert_eq!(next, calls.len(), "{why}: every command belongs to an agent, in their order");
    }
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
    let outcomes = install(
        &[Agent::Codex(program)],
        LIMITS_FOR_TESTS,
        &|p, a, l| run_program(p, a, l, home.path()),
        &|| Duration::ZERO,
    );
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
    // Long enough for the stand-in to start both on a busy machine.
    let limit = Duration::from_secs(3);
    let result = run_program(&hangs, &[], limit, dir.path());
    let (child, grandchild) = (pid_in(&child), pid_in(&grandchild));
    let _cleanup = (KillOnDrop(child), KillOnDrop(grandchild));
    assert_eq!(result, Err("ran out of time after 3s and was stopped".into()));
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
            _ => (&hangs, Duration::from_millis(100), Err("ran out of time after 100ms and was stopped".to_string())),
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
        // The escape parser keeps these as whitespace or text: the filter drops them.
        ("a vertical tab and a form feed", b"error:\x0bvt\x0cff\n".to_vec(), Some("error:vtff".into())),
        ("a one-character next line, U+0085", "error: nel\u{85}x\n".into(), Some("error: nelx".into())),
        ("a multi-byte character straddling the cut", straddles.into_bytes(), Some(straddles_cut)),
        ("exactly 300 characters", exactly.clone().into_bytes(), Some(exactly)),
        ("bytes that are not UTF-8", b"caf\xe9 \xff broke\n".to_vec(), Some("caf\u{FFFD} \u{FFFD} broke".into())),
        ("ESC ( B after a coloured error, as tput sgr0 prints", b"\x1b[31merror: sgr0 reset\x1b(B\x1b[m\n".to_vec(), Some("error: sgr0 reset".into())),
        ("ESC # 8", b"error: \x1b#8aligned\n".to_vec(), Some("error: aligned".into())),
        ("ESC 7, ESC = and ESC 8", b"\x1b7\x1b=saved\x1b8 error: cursor\n".to_vec(), Some("saved error: cursor".into())),
        ("a DCS string", b"error: \x1bPq#0;2;0;0;0\x1b\\dcs gone\n".to_vec(), Some("error: dcs gone".into())),
        // BEL doesn't end an APC string, so the rest of the text goes with it.
        ("an APC string ended by BEL", b"error: \x1b_Gf=100;AAAA\x07apc gone\n".to_vec(), Some("error:".into())),
        ("an APC string ended by ESC \\", b"error: \x1b_Gf=100;AAAA\x1b\\apc gone\n".to_vec(), Some("error: apc gone".into())),
        ("a one-character CSI", "error: c1 \u{9b}31mred\n".into(), Some("error: c1 red".into())),
        ("a one-character OSC title ended by BEL", "\u{9d}0;title\u{7}error: c1 title\n".into(), Some("error: c1 title".into())),
        ("a plain escape cut off at the end", b"error: trailing\x1b(".to_vec(), Some("error: trailing".into())),
        ("a bidi override around a word", "error: \u{202e}evil\u{202c} ok\n".into(), Some("error: evil ok".into())),
        ("brackets and parentheses in plain text", b"error: failed (code 3) [retry]\n".to_vec(), Some("error: failed (code 3) [retry]".into())),
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

/// A program that never stops starting new ones while its group is killed:
/// one started as the signal lands must not survive it.
#[cfg(unix)]
#[test]
fn a_program_starting_others_as_it_is_stopped_leaves_none_running() {
    let dir = tempfile::tempdir().unwrap();
    let sleep = sleep_marker(3013);
    let forks = script(dir.path(), "forks", &format!("while :; do {sleep} & done"));
    for run in 0..20 {
        let got = run_program(&forks, &[], Duration::from_millis(50), dir.path());
        assert_eq!(got, Err("ran out of time after 50ms and was stopped".to_string()), "run {run}");
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let left = left_behind(&sleep);
        if left.is_empty() {
            break;
        }
        assert!(Instant::now() < deadline, "left running: {} processes, first {:?}", left.len(), &left[..1]);
        std::thread::sleep(Duration::from_millis(20));
    }
}

// ── a signal caught during `run` ────────────────────────────────────

/// The variable that makes a test below act as the child it starts.
#[cfg(unix)]
const CHILD: &str = "ELEVENLABS_SETUP_TEST_CHILD";

/// Run the test `name` of this module again in a child copy of this test
/// program, with SIGTERM at its default and [`CHILD`] set, and how the child
/// ended. A name that matches no test ends it with 0.
#[cfg(unix)]
fn in_a_child(name: &str) -> std::process::ExitStatus {
    use std::os::unix::process::CommandExt;
    let module = module_path!().split_once("::").unwrap().1;
    let mut child = Command::new(std::env::current_exe().unwrap());
    child
        .args([&format!("{module}::{name}"), "--exact", "--test-threads=1", "-q"])
        .env(CHILD, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    unsafe {
        child.pre_exec(|| {
            libc::signal(libc::SIGTERM, libc::SIG_DFL);
            Ok(())
        });
    }
    child.status().unwrap()
}

/// A signal caught between two commands stops the run before the next starts.
#[cfg(unix)]
#[test]
fn a_signal_caught_before_a_command_starts_exits_before_it_starts() {
    if std::env::var_os(CHILD).is_some() {
        let _catch = interrupt::Catch::start();
        unsafe { libc::raise(libc::SIGTERM) };
        // Started, a missing program would only fail.
        let _ = run_program(Path::new("/nonexistent/claude"), &[], Duration::from_secs(5), Path::new("/"));
        std::process::exit(3);
    }
    let status = in_a_child("a_signal_caught_before_a_command_starts_exits_before_it_starts");
    assert_eq!(status.code(), Some(143), "{status:?}");
}

/// A signal caught after the last command still ends the run as interrupted.
#[cfg(unix)]
#[test]
fn a_signal_caught_after_the_last_command_exits_once_the_handlers_are_back() {
    if std::env::var_os(CHILD).is_some() {
        let catch = interrupt::Catch::start();
        unsafe { libc::raise(libc::SIGTERM) };
        catch.finish();
        std::process::exit(3);
    }
    let status = in_a_child("a_signal_caught_after_the_last_command_exits_once_the_handlers_are_back");
    assert_eq!(status.code(), Some(143), "{status:?}");
}
