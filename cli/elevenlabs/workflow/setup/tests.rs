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
