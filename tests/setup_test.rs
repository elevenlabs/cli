//! `elevenlabs setup` (and `onboard setup` where it differs) through the real
//! binary, against stand-ins for Claude Code and Codex. The CLI starts with
//! an empty environment but for a temporary `HOME` and `TMPDIR` and a PATH
//! holding only the stand-ins, so the developer's own agents, their configs
//! and the variables that name them are never found or touched.
#![cfg(unix)]

use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

const OFFLINE: &str = "http://127.0.0.1:9";

/// The seven commands, in order, that install the plugin in Claude Code and Codex.
const INSTALLS: [&str; 7] = [
    "claude plugin marketplace add elevenlabs/plugin",
    "claude plugin marketplace update elevenlabs",
    "claude plugin install elevenlabs@elevenlabs",
    "claude plugin update elevenlabs@elevenlabs",
    "codex plugin marketplace add elevenlabs/plugin",
    "codex plugin marketplace upgrade elevenlabs",
    "codex plugin add elevenlabs@elevenlabs",
];

const CLAUDE_DONE: &str =
    "Claude Code: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, \
    restart Claude Code, then run /mcp and sign in to the ElevenLabs MCP server.\n";
const CODEX_DONE: &str = "Codex: installed the ElevenLabs plugin (skills and the hosted MCP server). To finish, restart \
    Codex, then run `codex mcp login elevenlabs` in a terminal to sign in to the ElevenLabs MCP server.\n";
const NO_AGENT: &str = "No Claude Code, Codex or Cursor found. If you use Claude Code in the Claude desktop app, ask it \
    to run \"elevenlabs setup\" for you. For other coding agents, install the ElevenLabs skills with: npx skills \
    add elevenlabs/skills\n";

/// The CLI run in `cwd` with nothing of the developer's environment: only
/// `home` as `HOME` and `TMPDIR`, and `path` as PATH.
fn cli(home: &Path, cwd: &Path, path: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(args)
        .current_dir(cwd)
        .env_clear()
        .env("HOME", home)
        .env("TMPDIR", home)
        .env("PATH", path)
        .env("ELEVENLABS_BASE_URL", OFFLINE)
        // A sign-in is read from files in `home`, never the system keychain.
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .stdin(Stdio::null());
    cmd
}

fn temp() -> (tempfile::TempDir, tempfile::TempDir) {
    (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap())
}

fn all_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// What the command printed, after checking it exited with `code`.
fn stdout(cmd: &mut Command, code: i32) -> String {
    let out = cmd.output().unwrap();
    assert_eq!(out.status.code(), Some(code), "{}", all_output(&out));
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// `onboard setup`'s reply, which is one line of JSON.
fn reply(cmd: &mut Command) -> serde_json::Value {
    let out = stdout(cmd, 0);
    assert_eq!(out.lines().count(), 1, "{out}");
    serde_json::from_str(&out).unwrap()
}

/// A program at `path`, written by a short-lived `sh`, never by this process.
/// Tests run as threads of one process: if one forks while another holds a
/// script open for writing, the forked child keeps that descriptor until it
/// execs, and Linux then refuses to start the script with "Text file busy"
/// (rust-lang/rust#114554). A file this process never opens can't be held.
fn program(path: &Path, text: &str) {
    let status = Command::new("/bin/sh")
        .args([
            "-c",
            "printf '%s' \"$1\" > \"$2\" && chmod 755 \"$2\"",
            "sh",
            text,
        ])
        .arg(path)
        .stdin(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success(), "writing {}", path.display());
}

/// Stand-ins for `claude` and `codex` in `dir`, each adding the command it
/// was given to `dir/ran`, and failing `plugin install` when `fail_install`.
fn stand_ins(dir: &Path, fail_install: bool) -> String {
    for name in ["claude", "codex"] {
        let fails = match fail_install {
            true => "case \"$*\" in *\"plugin install\"*) echo 'something broke' >&2; echo 'install refused' >&2; exit 1;; esac\n",
            false => "",
        };
        let ran = dir.join("ran");
        program(
            &dir.join(name),
            &format!(
                "#!/bin/sh\necho \"{name} $*\" >> '{}'\n{fails}exit 0\n",
                ran.display()
            ),
        );
    }
    dir.display().to_string()
}

fn recorded(file: &Path) -> Vec<String> {
    std::fs::read_to_string(file)
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn setup_installs_in_each_agent_with_or_without_yes() {
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let path = stand_ins(bin.path(), false);
    // `--yes`, which agents add by habit, changes nothing.
    for args in [&["setup"][..], &["setup", "--yes"]] {
        let said = stdout(&mut cli(home.path(), cwd.path(), &path, args), 0);
        assert_eq!(said, format!("{CLAUDE_DONE}{CODEX_DONE}"), "{args:?}");
    }
    assert_eq!(recorded(&bin.path().join("ran")), INSTALLS.repeat(2));
}

#[test]
fn with_dry_run_setup_installs_nothing() {
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let path = stand_ins(bin.path(), false);
    let said = stdout(
        &mut cli(home.path(), cwd.path(), &path, &["setup", "--dry-run"]),
        0,
    );
    assert_eq!(said, "Nothing was installed, because of --dry-run.\n");
    assert_eq!(recorded(&bin.path().join("ran")), Vec::<String>::new());
}

/// With none of the three found, `setup` says how to get the skills, and
/// nothing is run or written.
#[test]
fn with_no_agent_found_setup_says_how_to_get_the_skills_and_changes_nothing() {
    let (home, cwd) = temp();
    let empty = tempfile::tempdir().unwrap();
    let path = empty.path().to_str().unwrap();
    let said = stdout(&mut cli(home.path(), cwd.path(), path, &["setup"]), 0);
    assert_eq!(said, NO_AGENT);
    let r = reply(&mut cli(
        home.path(),
        cwd.path(),
        path,
        &["onboard", "setup"],
    ));
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(r["say"], NO_AGENT.trim_end());
    assert_eq!(r["details"]["setup"], serde_json::json!([]), "{r}");
    let written: Vec<_> = std::fs::read_dir(home.path())
        .unwrap()
        .flatten()
        .map(|e| e.file_name())
        .collect();
    assert!(
        written.is_empty(),
        "nothing written in the home folder: {written:?}"
    );
}

#[test]
fn a_failed_install_exits_1_with_the_agents_error_and_the_command_to_try_again() {
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let path = stand_ins(bin.path(), true);
    let failed = "Claude Code: failed: `claude plugin install elevenlabs@elevenlabs` exited with 1: install refused.";
    let said = stdout(&mut cli(home.path(), cwd.path(), &path, &["setup"]), 1);
    assert_eq!(
        said,
        format!("{failed}\n{CODEX_DONE}To try again, run `elevenlabs setup`.\n")
    );
    // Under npx, the command to try again is npx's too.
    let r = reply(
        cli(home.path(), cwd.path(), &path, &["onboard", "setup"]).env("npm_command", "exec"),
    );
    assert_eq!(r["next"], "done", "{r}");
    let say = r["say"].as_str().unwrap();
    assert!(
        say.starts_with(&format!("{failed} Codex: installed")),
        "{r}"
    );
    assert!(
        say.ends_with("To try again, run `npx -y @elevenlabs/cli@latest setup`."),
        "{r}"
    );
    assert_eq!(r["details"]["setup"][0]["ok"], false, "{r}");
}

#[test]
fn without_a_home_folder_setup_and_onboard_setup_say_so() {
    let (home, cwd) = temp();
    let mut setup = cli(home.path(), cwd.path(), "/nonexistent", &["setup"]);
    let out = setup.env_remove("HOME").output().unwrap();
    assert!(!out.status.success(), "{}", all_output(&out));
    assert!(
        all_output(&out).contains("No home folder found (HOME is not set)."),
        "{}",
        all_output(&out)
    );
    let mut onboard = cli(
        home.path(),
        cwd.path(),
        "/nonexistent",
        &["onboard", "setup"],
    );
    let r = reply(onboard.env_remove("HOME"));
    assert_eq!(r["next"], "done", "{r}");
    assert_eq!(
        r["say"],
        "ElevenLabs could not be set up in the coding agents: no home folder was found. To try again later, \
         run `elevenlabs setup`."
    );
}

#[test]
fn setup_starts_agents_from_the_shells_environment_never_the_projects_env() {
    let (home, cwd) = temp();
    let (bin, project_bin) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let ran = bin.path().join("ran");
    for (dir, said) in [(bin.path(), "shell's"), (project_bin.path(), "project's")] {
        program(
            &dir.join("claude"),
            &format!("#!/bin/sh\necho \"{said} claude $* [${{ELEVENLABS_SETUP_TEST:-unset}}]\" >> '{}'\n", ran.display()),
        );
    }
    let env = format!(
        "PATH={}\nELEVENLABS_SETUP_TEST=from-the-project\n",
        project_bin.path().display()
    );
    std::fs::write(cwd.path().join(".env"), env).unwrap();
    stdout(
        &mut cli(
            home.path(),
            cwd.path(),
            bin.path().to_str().unwrap(),
            &["setup"],
        ),
        0,
    );
    let want: Vec<String> = INSTALLS[..4]
        .iter()
        .map(|c| format!("shell's {c} [unset]"))
        .collect();
    assert_eq!(recorded(&ran), want);
}

/// Under npx, PATH starts with the project's `node_modules/.bin` and the one
/// in each folder above it. A `claude`, `codex` or `node` there is the
/// project's own: none ever runs. The agent gets a PATH without those
/// folders, or any relative or empty entry, so a `node` it starts by name,
/// as the npm `claude` and `codex` do, is never the project's either.
#[test]
fn under_npx_nothing_in_the_projects_node_modules_runs() {
    let home = tempfile::tempdir().unwrap();
    let base = tempfile::tempdir().unwrap();
    let (parent, markers) = (base.path().join("parent"), base.path().join("markers"));
    let cwd = parent.join("project");
    std::fs::create_dir(&markers).unwrap();
    // A relative entry is read from the agent's folder, the home folder, and
    // an empty one is that folder itself.
    let project_bins = [
        (cwd.join("node_modules").join(".bin"), "project"),
        (parent.join("node_modules").join(".bin"), "parent"),
        (home.path().join("node_modules").join(".bin"), "relative"),
        (home.path().to_path_buf(), "empty"),
    ];
    for (dir, whose) in &project_bins {
        std::fs::create_dir_all(dir).unwrap();
        for name in ["claude", "codex", "node"] {
            let marker = markers.join(format!("{whose} {name}"));
            program(
                &dir.join(name),
                &format!("#!/bin/sh\n: > '{}'\n", marker.display()),
            );
        }
    }
    let started = || -> Vec<_> {
        let found = std::fs::read_dir(&markers).unwrap().flatten();
        found.map(|e| e.file_name()).collect()
    };
    let neutral = tempfile::tempdir().unwrap();
    let node_ran = neutral.path().join("node ran");
    let node = format!("#!/bin/sh\necho \"node $*\" >> '{}'\n", node_ran.display());
    program(&neutral.path().join("node"), &node);
    let bin = tempfile::tempdir().unwrap();
    let seen = bin.path().join("seen");
    for name in ["claude", "codex"] {
        let script = format!(
            "#!/bin/sh\nprintf '%s\\n' \"$PATH\" >> '{}'\nnode {name}\n",
            seen.display()
        );
        program(&bin.path().join(name), &script);
    }
    let (first, second) = (project_bins[0].0.display(), project_bins[1].0.display());
    let projects = format!("{first}:{second}:node_modules/.bin:");
    let path = format!(
        "{projects}:{}:{}",
        bin.path().display(),
        neutral.path().display()
    );
    stdout(&mut cli(home.path(), &cwd, &path, &["setup"]), 0);

    assert_eq!(
        started(),
        Vec::<std::ffi::OsString>::new(),
        "a project's program ran"
    );
    let node: Vec<String> = INSTALLS
        .iter()
        .map(|c| format!("node {}", &c[..c.find(' ').unwrap()]))
        .collect();
    assert_eq!(
        recorded(&node_ran),
        node,
        "the agent's node is the one outside the project"
    );
    let clean = format!("{}:{}", bin.path().display(), neutral.path().display());
    assert_eq!(recorded(&seen), vec![clean; 7]);
    // With only the project's programs on PATH, nothing is found.
    let said = stdout(&mut cli(home.path(), &cwd, &projects, &["setup"]), 0);
    assert_eq!(said, NO_AGENT);
    assert_eq!(
        started(),
        Vec::<std::ffi::OsString>::new(),
        "a project's program ran"
    );
}

// ── interrupts ──────────────────────────────────────────────────────

/// Stand-ins that run every command at once but `hang_on`, which starts
/// `sleep <marker>`, writes its pid to `dir/pid`, and waits.
fn hanging_stand_ins(dir: &Path, hang_on: &str, marker: &str) -> String {
    let pid = dir.join("pid");
    for name in ["claude", "codex"] {
        let script = format!(
            "#!/bin/sh\nif [ \"{name} $*\" = \"{hang_on}\" ]; then\n  /bin/sleep {marker} &\n  echo $! > '{0}.new'\n  \
             /bin/mv '{0}.new' '{0}'\n  wait\nfi\nexit 0\n",
            pid.display()
        );
        program(&dir.join(name), &script);
    }
    dir.display().to_string()
}

/// A `sleep` time no other run of these tests on this machine uses:
/// `<seconds>.<this process's id>`, the id at a fixed width so one marker
/// never contains another.
fn marker(seconds: u32) -> String {
    format!("{seconds}.{:010}", std::process::id())
}

/// The processes whose command line contains `text`.
fn running(text: &str) -> Vec<String> {
    let out = Command::new("ps")
        .args(["-axo", "pid=,command="])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|l| l.contains(text) && !l.contains("ps -axo"))
        .map(str::to_string)
        .collect()
}

/// Nothing whose command line holds `marker` or the stand-ins' folder is left.
fn nothing_left(marker: &str, bin: &Path, row: &str) {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let mut left = running(&format!("sleep {marker}"));
        left.extend(running(&bin.display().to_string()));
        if left.is_empty() {
            return;
        }
        assert!(Instant::now() < deadline, "{row}: left running: {left:?}");
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Kills the process when the test ends, passed or not.
struct KillOnDrop(i32);

impl Drop for KillOnDrop {
    fn drop(&mut self) {
        unsafe { libc::kill(self.0, libc::SIGKILL) };
    }
}

/// `cmd` started with SIGINT at its default, as from a terminal, or ignored.
fn start(mut cmd: Command, sigint: libc::sighandler_t) -> (std::process::Child, KillOnDrop) {
    use std::os::unix::process::CommandExt;
    unsafe {
        cmd.pre_exec(move || {
            libc::signal(libc::SIGINT, sigint);
            Ok(())
        });
    }
    let child = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let pid = child.id() as i32;
    (child, KillOnDrop(pid))
}

fn pid_in(file: &Path) -> i32 {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(pid) = std::fs::read_to_string(file)
            .ok()
            .and_then(|t| t.trim().parse().ok())
        {
            return pid;
        }
        assert!(
            Instant::now() < deadline,
            "the stand-in never started its child"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn exit_of(child: &mut std::process::Child, within: Duration) -> std::process::ExitStatus {
    let deadline = Instant::now() + within;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status;
        }
        assert!(Instant::now() < deadline, "still running after {within:?}");
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn an_interrupt_during_an_agent_command_stops_it_and_exits_as_interrupted() {
    // (row, the command running, how the CLI was started, the signal, the exit code)
    let rows: [(&str, &str, &[&str], i32, i32); 3] = [
        (
            "SIGINT during a Claude Code command",
            INSTALLS[2],
            &["setup"],
            libc::SIGINT,
            130,
        ),
        (
            "SIGTERM during a Codex command",
            INSTALLS[6],
            &["setup"],
            libc::SIGTERM,
            143,
        ),
        (
            "SIGINT during `onboard setup`",
            INSTALLS[3],
            &["onboard", "setup"],
            libc::SIGINT,
            130,
        ),
    ];
    for (n, (row, command, args, signal, code)) in rows.into_iter().enumerate() {
        let marker = marker(3200 + n as u32);
        let (home, cwd) = temp();
        let bin = tempfile::tempdir().unwrap();
        let path = hanging_stand_ins(bin.path(), command, &marker);
        let (mut setup, _setup) = start(cli(home.path(), cwd.path(), &path, args), libc::SIG_DFL);
        let _sleep = KillOnDrop(pid_in(&bin.path().join("pid")));
        unsafe { libc::kill(setup.id() as i32, signal) };
        let status = exit_of(&mut setup, Duration::from_secs(10));
        assert_eq!(status.code(), Some(code), "{row}: {status:?}");
        nothing_left(&marker, bin.path(), row);
    }
}

#[test]
fn with_sigint_ignored_an_interrupt_changes_nothing_and_sigterm_still_stops_it() {
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let marker = marker(3300);
    let path = hanging_stand_ins(bin.path(), INSTALLS[1], &marker);
    let (mut setup, _setup) = start(
        cli(home.path(), cwd.path(), &path, &["setup"]),
        libc::SIG_IGN,
    );
    let sleep = pid_in(&bin.path().join("pid"));
    let _sleep = KillOnDrop(sleep);
    unsafe { libc::kill(setup.id() as i32, libc::SIGINT) };
    // Ignored: for a while, both keep running.
    let until = Instant::now() + Duration::from_millis(300);
    while Instant::now() < until {
        assert!(
            setup.try_wait().unwrap().is_none(),
            "setup stopped on an ignored SIGINT"
        );
        assert_eq!(
            unsafe { libc::kill(sleep, 0) },
            0,
            "the agent command stopped on an ignored SIGINT"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    unsafe { libc::kill(setup.id() as i32, libc::SIGTERM) };
    let status = exit_of(&mut setup, Duration::from_secs(10));
    assert_eq!(status.code(), Some(143), "{status:?}");
    nothing_left(&marker, bin.path(), "SIGTERM after an ignored SIGINT");
}

// ── counting runs ───────────────────────────────────────────────────

/// The `cmd/` tags on each request `server` received, and whether each
/// carried `key`.
async fn counted(server: &wiremock::MockServer, key: &str) -> Vec<(String, bool)> {
    let requests = server.received_requests().await.unwrap();
    requests
        .iter()
        .map(|r| {
            assert_eq!(r.url.path(), "/v1/user");
            let agent = r.headers["user-agent"].to_str().unwrap();
            let tags: Vec<&str> = agent.split(' ').filter(|t| t.starts_with("cmd/")).collect();
            (
                tags.join(" "),
                r.headers.get("xi-api-key").is_some_and(|k| k == key),
            )
        })
        .collect()
}

#[tokio::test]
async fn with_a_key_and_telemetry_on_setup_counts_each_agent_it_tried() {
    const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let agents = stand_ins(bin.path(), true);
    let empty = tempfile::tempdir().unwrap();
    let uri = server.uri();
    let setup = |path: &str, args: &[&str], key: bool, code: i32| {
        let args = [&["--base-url", uri.as_str()], args].concat();
        let mut cmd = cli(home.path(), cwd.path(), path, &args);
        if key {
            cmd.env("ELEVENLABS_API_KEY", KEY);
        }
        stdout(&mut cmd, code);
    };

    setup(&agents, &["setup"], true, 1);
    let mut want = vec![
        ("cmd/setup.claude_code.failed".to_string(), true),
        ("cmd/setup.codex.installed".into(), true),
    ];
    assert_eq!(counted(&server, KEY).await, want);
    setup(empty.path().to_str().unwrap(), &["setup"], true, 0);
    want.push(("cmd/setup.none".into(), true));
    assert_eq!(counted(&server, KEY).await, want);
    // Nothing more: telemetry off, no key, or --dry-run.
    setup(&agents, &["setup", "--telemetry", "off"], true, 1);
    setup(&agents, &["setup"], false, 1);
    setup(&agents, &["setup", "--dry-run"], true, 0);
    setup(
        empty.path().to_str().unwrap(),
        &["setup", "--dry-run"],
        true,
        0,
    );
    assert_eq!(counted(&server, KEY).await, want);
}

/// The installs are over when the count is made, so Ctrl-C has its usual
/// effect again: it ends `setup` at once, not after the requests' time limit.
#[tokio::test]
async fn an_interrupt_while_a_run_is_counted_ends_setup_at_once() {
    use std::os::unix::process::ExitStatusExt;
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .respond_with(wiremock::ResponseTemplate::new(200).set_delay(Duration::from_secs(4)))
        .mount(&server)
        .await;
    let (home, cwd) = temp();
    let bin = tempfile::tempdir().unwrap();
    let agents = stand_ins(bin.path(), false);
    let uri = server.uri();
    let mut cmd = cli(
        home.path(),
        cwd.path(),
        &agents,
        &["--base-url", &uri, "setup"],
    );
    cmd.env(
        "ELEVENLABS_API_KEY",
        "sk_0123456789abcdef0123456789abcdef01234567",
    );
    let (mut setup, _kill) = start(cmd, libc::SIG_DFL);
    let deadline = Instant::now() + Duration::from_secs(20);
    while server.received_requests().await.unwrap().is_empty() {
        assert!(Instant::now() < deadline, "setup never counted the run");
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(recorded(&bin.path().join("ran")).len(), INSTALLS.len());
    unsafe { libc::kill(setup.id() as i32, libc::SIGINT) };
    let status = exit_of(&mut setup, Duration::from_secs(2));
    assert_eq!(status.signal(), Some(libc::SIGINT), "{status:?}");
}
