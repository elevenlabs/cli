use super::super::util::opt_string;
use super::auth::*;
use super::browser::reachable_in;
use super::clipboard::{classify, Clip, Marker};
use super::env_file::{file_key, with_key, PLACEHOLDER_NOTE};
use super::onboard_command;
use super::project::last4;

#[test]
fn the_sign_in_is_read_out_of_main_rs() {
    let c = sign_in_config().expect("main.rs declares the sign-in");
    assert_eq!(c.scheme, "OAuth");
    assert_eq!(c.client_id.len(), 36, "{}", c.client_id);
    assert!(
        c.authorize_url.ends_with("/app/oauth/authorize"),
        "{}",
        c.authorize_url
    );
    assert!(c.token_url.ends_with("/v1/oauth/token"), "{}", c.token_url);
    assert_eq!(c.redirect_host, "localhost");
    assert!(c.redirect_ports.contains(&8484));
    assert!(c.scopes.iter().any(|s| s == "user_read") && c.scopes.len() > 20);
    assert!(c.success_url.starts_with("https://") && c.error_url.starts_with("https://"));
}

#[test]
fn the_clipboard_is_a_key_a_marker_or_nothing() {
    let key = "sk_0123456789abcdef0123456789abcdef";
    assert!(matches!(classify(Some(format!("  {key}\n"))), Clip::Key(k) if k == key));
    for m in [Marker::NotAvailable, Marker::NotPermitted, Marker::Failed] {
        assert_eq!(classify(Some(m.text())), Clip::Marker(m));
    }
    assert_eq!(Marker::Failed.text(), "elevenlabs:failed");
    assert!(
        matches!(classify(Some(format!("{key}_residency_eu1"))), Clip::Key(_)),
        "residency suffix"
    );
    for junk in [
        "",
        "sk_short",
        "sk_0123456789abcdef with space",
        "sk_vendor_ABCDEF0123456789abcdef0123456789",
        "sk-proj-0123456789abcdef0123456789abcdef",
        "elevenlabs:",
        "elevenlabs:other",
        "hi",
    ] {
        assert!(
            matches!(classify(Some(junk.into())), Clip::Nothing),
            "{junk:?}"
        );
    }
    assert!(matches!(classify(None), Clip::Nothing));
}

#[test]
fn the_nearest_dotenv_and_its_key_are_found_up_the_tree() {
    use super::env_file::nearest_dotenv;
    let root = tempfile::tempdir().unwrap();
    let nested = root.path().join("apps").join("web");
    std::fs::create_dir_all(&nested).unwrap();
    assert!(nearest_dotenv(&nested).is_none(), "no .env anywhere");

    std::fs::write(root.path().join(".env"), "ELEVENLABS_API_KEY=sk_root\n").unwrap();
    assert_eq!(
        nearest_dotenv(&nested).expect("the root .env"),
        (root.path().join(".env"), Some("sk_root".to_string()))
    );

    // Only the nearest file is read, as loaders do, even when its line is empty.
    std::fs::write(nested.join(".env"), "ELEVENLABS_API_KEY=\n").unwrap();
    assert_eq!(nearest_dotenv(&nested).unwrap().1, Some(String::new()));
    std::fs::write(nested.join(".env"), "OTHER=1\n").unwrap();
    assert_eq!(nearest_dotenv(&nested).unwrap().1, None, "no key line");
    std::fs::write(
        nested.join(".env"),
        "ELEVENLABS_API_KEY=sk_old\nOTHER=1\nELEVENLABS_API_KEY=sk_new\n",
    )
    .unwrap();
    assert_eq!(
        nearest_dotenv(&nested).unwrap().1.as_deref(),
        Some("sk_new")
    );
}

#[test]
fn the_key_line_is_replaced_in_place_or_appended_once() {
    assert_eq!(with_key("", "sk_1"), "ELEVENLABS_API_KEY=sk_1\n");
    assert_eq!(with_key("A=1", "sk_1"), "A=1\nELEVENLABS_API_KEY=sk_1\n");
    assert_eq!(
        with_key("A=1\nexport ELEVENLABS_API_KEY=\"old\"\nB=2\n", "sk_1"),
        "A=1\nexport ELEVENLABS_API_KEY=sk_1\nB=2\n",
        "export is kept"
    );
    assert_eq!(
        with_key("A=1\r\nB=2\r\n", "sk_1"),
        "A=1\r\nB=2\r\nELEVENLABS_API_KEY=sk_1\r\n",
        "line endings are kept"
    );
    assert_eq!(
        file_key("ELEVENLABS_API_KEY=\nELEVENLABS_API_KEY=sk_2\n"),
        Some("sk_2".into()),
        "the last line wins, as for dotenv"
    );
    assert_eq!(
        with_key("ELEVENLABS_API_KEY=sk_1\nELEVENLABS_API_KEY=sk_2\n", "sk_3"),
        "ELEVENLABS_API_KEY=sk_1\nELEVENLABS_API_KEY=sk_3\n"
    );
    assert_eq!(file_key("exported=1"), None);
    let placeholder = with_key("", "");
    assert!(placeholder.starts_with(PLACEHOLDER_NOTE));
    assert!(placeholder.ends_with("ELEVENLABS_API_KEY=\n"));
    assert_eq!(
        file_key(&placeholder),
        Some(String::new()),
        "an empty line is a placeholder"
    );
    assert_eq!(with_key(&placeholder, ""), placeholder, "not duplicated");
    assert_eq!(file_key("ELEVENLABS_API_KEY = 'sk_1'"), Some("sk_1".into()));
    assert_eq!(file_key("OTHER=1"), None);
    assert_eq!(
        file_key("ELEVENLABS_API_KEY=sk_1 # prod"),
        Some("sk_1".into())
    );
    // A line the loader can't parse still holds the key, and is still the one replaced.
    for contents in [
        "NEXT_PUBLIC_APP_NAME=Joe's Pizza\nELEVENLABS_API_KEY=sk_1\n",
        "\u{feff}ELEVENLABS_API_KEY=sk_1\n",
        "ELEVENLABS_API_KEY=sk_1 ",
        "ELEVENLABS_API_KEY=\"sk_1\" \nOTHER=1\n",
        "export ELEVENLABS_API_KEY='sk_1'\t\n",
        "ELEVEN_KEY=sk_1\nELEVENLABS_API_KEY=${ELEVEN_KEY}\n",
        "ELEVEN_KEY=sk_1 \nELEVENLABS_API_KEY=${ELEVEN_KEY}\n",
        "\u{feff}ELEVEN_KEY=sk_1\nELEVENLABS_API_KEY=${ELEVEN_KEY}\n",
        "OTHER=sk_1\nAPP=Joe's Pizza\nELEVENLABS_API_KEY=${OTHER}\n",
        "ELEVENLABS_API_KEY=sk_0\nAPP=Joe's Pizza\nELEVENLABS_API_KEY=sk_1 # $5\n",
    ] {
        assert_eq!(file_key(contents), Some("sk_1".into()), "{contents:?}");
        assert_eq!(file_key(&with_key(contents, "sk_2")), Some("sk_2".into()));
    }
    // Parsed as the runtime's loader parses it.
    for line in [
        "ELEVENLABS_API_KEY=sk_1\t# prod",
        "export ELEVENLABS_API_KEY=sk_1",
    ] {
        assert_eq!(file_key(line), Some("sk_1".into()), "{line}");
    }
    assert_eq!(last4("sk_abc"), "_abc");
    assert_eq!(last4("ab"), "ab");
}

#[test]
fn only_a_loopback_url_counts_as_local() {
    assert_eq!(
        loopback("http://localhost:8000/"),
        Some("http://localhost:8000".into())
    );
    assert_eq!(
        loopback("http://127.0.0.1:8000"),
        Some("http://127.0.0.1:8000".into())
    );
    assert_eq!(
        loopback("http://[::1]:3000"),
        Some("http://[::1]:3000".into())
    );
    assert_eq!(loopback("http://localhost:8000@evil.example.com/"), None);
    assert_eq!(loopback("https://evil.example.com/localhost"), None);
    assert_eq!(loopback("https://api.elevenlabs.io"), None);
}

#[test]
fn a_browser_reaches_a_laptop_but_not_a_remote_box() {
    let none = |_: &str| None;
    let only = |name: &'static str| move |n: &str| (n == name).then(|| "x".to_string());
    assert!(reachable_in("macos", &none, false));
    assert!(reachable_in("windows", &none, false));
    assert!(!reachable_in("macos", &only("SSH_CONNECTION"), false));
    assert!(!reachable_in("linux", &none, false), "no display");
    assert!(reachable_in("linux", &only("DISPLAY"), false));
    assert!(reachable_in("linux", &only("WSL_DISTRO_NAME"), false));
    assert!(!reachable_in("linux", &only("DISPLAY"), true), "container");
    assert!(!reachable_in("linux", &only("CODESPACES"), false));
}

/// The developer's own git config (a global excludes file, say) must not
/// change what the tests see, and the tests must not touch their clipboard.
/// Set once for the whole test process, before any test runs git.
fn isolate() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        std::env::set_var("GIT_CONFIG_GLOBAL", "/dev/null");
        std::env::set_var("GIT_CONFIG_NOSYSTEM", "1");
        // Nor its default excludes file, $XDG_CONFIG_HOME/git/ignore.
        let empty = tempfile::tempdir().unwrap().keep();
        std::env::set_var("XDG_CONFIG_HOME", empty);
        std::env::set_var(super::clipboard::NO_CLIPBOARD_ENV, "1");
    });
}

fn git(dir: &std::path::Path, args: &[&str]) {
    isolate();
    let ok = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.email=t@example.com", "-c", "user.name=t"])
        .args(args)
        .output()
        .unwrap()
        .status
        .success();
    assert!(ok, "git {args:?}");
}

fn project(dir: &std::path::Path) -> super::project::Project {
    isolate();
    super::project::Project::at(dir.to_path_buf(), None)
}

#[test]
fn every_step_parses_its_flags_and_bare_onboard_is_refused() {
    let cmd = onboard_command();
    let parse = |args: &[&str]| cmd.clone().try_get_matches_from(args);
    assert!(parse(&["onboard"]).is_err(), "a step is required");
    let m = parse(&["onboard", "init", "--env-file", ".env.dev", "--no-browser"]).unwrap();
    let (name, sub) = m.subcommand().unwrap();
    assert_eq!(name, "init");
    assert_eq!(opt_string(sub, "env-file").as_deref(), Some(".env.dev"));
    assert!(sub.get_flag("no-browser"));
    assert!(
        parse(&["onboard", "connect", "--product", "music"]).is_err(),
        "one key for every product, so no product to name"
    );
    let m = parse(&["onboard", "connect", "--replace", "--wait", "--no-browser"]).unwrap();
    let sub = m.subcommand_matches("connect").unwrap();
    assert!(sub.get_flag("replace") && sub.get_flag("wait") && sub.get_flag("no-browser"));
    assert_eq!(
        parse(&["onboard", "status"]).unwrap().subcommand_name(),
        Some("check")
    );
    assert_eq!(
        parse(&["onboard", "test"]).unwrap().subcommand_name(),
        Some("test")
    );
    assert!(parse(&["onboard", "check", "--replace"]).is_err());
}

#[test]
fn the_env_file_follows_the_stack_and_is_pinned_in_every_next_command() {
    use super::project::{Project, Stack};
    let dir = tempfile::tempdir().unwrap();
    let p = project(dir.path());
    assert_eq!(
        (p.stack, p.env_display.as_str()),
        (Stack::Other, ".env.local")
    );
    // Named in every next command, so a later stack change can't move it.
    assert_eq!(p.next("check"), "check --env-file .env.local");
    std::fs::write(dir.path().join("requirements.txt"), "elevenlabs==2\n").unwrap();
    let p = project(dir.path());
    assert_eq!((p.stack, p.env_display.as_str()), (Stack::Python, ".env"));
    assert!(p.sdk_declared());
    std::fs::write(dir.path().join("package.json"), "{}").unwrap();
    let mut p = Project::at(dir.path().to_path_buf(), Some("config/my env".into()));
    assert_eq!(p.stack, Stack::Node);
    assert_eq!(p.env_file, dir.path().join("config/my env"));
    assert_eq!(p.show(&dir.path().join("config/my env")), "config/my env");
    assert!(p.env_file_problem().unwrap().contains("doesn't exist"));
    p.no_browser = true;
    assert_eq!(
        p.next("connect --replace"),
        "connect --replace --env-file 'config/my env' --no-browser"
    );
    assert_eq!(
        p.next("init"),
        "init --env-file 'config/my env' --no-browser"
    );
    assert_eq!(super::project::shell_arg("it's"), r"'it'\''s'");
    // --no-browser goes on through check, so a later connect keeps it.
    assert_eq!(
        p.next("check"),
        "check --env-file 'config/my env' --no-browser"
    );
    // A file up the tree is named from the project, as a command run there needs it.
    let app = dir.path().join("apps/web");
    std::fs::create_dir_all(&app).unwrap();
    let p = project(&app);
    assert_eq!(p.show(&dir.path().join(".env")), "../../.env");
    assert_eq!(p.show(&app.join("config/.env")), "config/.env");
    assert!(p
        .loading("../../.env")
        .contains("only read env files in the app's folder"));
}

#[test]
fn a_tracked_env_file_says_how_to_untrack_it() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    std::fs::write(dir.path().join(".env.local"), "A=1\n").unwrap();
    git(dir.path(), &["add", ".env.local"]);
    let problem = project(dir.path()).env_file_problem().unwrap();
    assert!(problem.contains("git rm --cached .env.local"), "{problem}");
    assert!(
        !problem.contains("such as .env.local"),
        "never suggests the refused file"
    );
}

#[test]
fn the_shell_wins_then_the_env_file_then_a_dotenv_up_the_tree() {
    use super::project::KeySource;
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    let p = project(&app);
    assert!(p.key_given(None).is_none());

    std::fs::write(app.join(".env.local"), "ELEVENLABS_API_KEY=\n").unwrap();
    assert!(
        p.key_given(None).is_none(),
        "an empty placeholder is not a key"
    );

    // A `.env` up the tree counts when the env file has no key line.
    std::fs::write(app.join(".env.local"), "OTHER=1\n").unwrap();
    std::fs::write(root.path().join(".env"), "ELEVENLABS_API_KEY=sk_dotenv\n").unwrap();
    let k = p.key_given(None).unwrap();
    assert_eq!(
        (k.value.as_str(), &k.source),
        ("sk_dotenv", &KeySource::Dotenv(root.path().join(".env")))
    );
    // Exported from that `.env` into the shell (direnv, mise), it's still the file's.
    assert_eq!(
        p.key_given(Some("sk_dotenv".into())).unwrap().source,
        KeySource::Dotenv(root.path().join(".env"))
    );

    // The env file wins over that `.env`, as it does for Next.js and Vite.
    std::fs::write(app.join(".env.local"), "ELEVENLABS_API_KEY=sk_file\n").unwrap();
    let k = p.key_given(None).unwrap();
    assert_eq!(
        (k.value.as_str(), &k.source),
        ("sk_file", &KeySource::EnvFile)
    );

    // A variable really set in the shell wins over every file.
    let k = p.key_given(Some("sk_shell".into())).unwrap();
    assert_eq!(
        (k.value.as_str(), &k.source),
        ("sk_shell", &KeySource::Shell)
    );
    assert!(k.file(&p).is_none());
    assert_eq!(
        p.key_given(Some("sk_file".into())).unwrap().source,
        KeySource::EnvFile
    );

    // An empty line in the env file is the value Next.js and Vite load, over any `.env`.
    std::fs::write(app.join(".env.local"), "ELEVENLABS_API_KEY=\n").unwrap();
    assert!(
        p.key_given(None).is_none(),
        "the placeholder wins over the .env up the tree"
    );

    // A Python app's env file is the `.env` the runtime loads, but the shell's
    // value is read from before that, so a key set twice in it is still the
    // file's, and the last line (the app's loader's) is checked.
    let py = tempfile::tempdir().unwrap();
    std::fs::write(py.path().join("requirements.txt"), "elevenlabs\n").unwrap();
    std::fs::write(
        py.path().join(".env"),
        "ELEVENLABS_API_KEY=sk_old\nELEVENLABS_API_KEY=sk_new\n",
    )
    .unwrap();
    let k = project(py.path()).key_given(None).unwrap();
    assert_eq!(
        (k.value.as_str(), &k.source),
        ("sk_new", &KeySource::EnvFile)
    );
}

#[test]
fn the_sdk_counts_only_when_declared_and_imported_in_source() {
    let dir = tempfile::tempdir().unwrap();
    let d = dir.path();
    std::fs::write(
        d.join("package.json"),
        r#"{"dependencies":{"express":"4"}}"#,
    )
    .unwrap();
    std::fs::create_dir_all(d.join("node_modules/x")).unwrap();
    std::fs::write(
        d.join("node_modules/x/i.js"),
        "import '@elevenlabs/elevenlabs-js'",
    )
    .unwrap();
    std::fs::write(d.join("README.md"), "uses @elevenlabs/elevenlabs-js").unwrap();
    let p = project(d);
    assert!(!p.sdk_declared());
    assert!(!p.sdk_imported(), "node_modules and docs don't count");
    std::fs::write(
        d.join("package.json"),
        r#"{"dependencies":{"@elevenlabs/elevenlabs-js":"2"}}"#,
    )
    .unwrap();
    std::fs::create_dir_all(d.join("src")).unwrap();
    std::fs::write(
        d.join("src/speak.ts"),
        "import { ElevenLabsClient } from \"@elevenlabs/elevenlabs-js\";",
    )
    .unwrap();
    assert!(p.sdk_declared() && p.sdk_imported());

    // Any ElevenLabs SDK counts, in any component file, but not the CLI.
    let web = tempfile::tempdir().unwrap();
    let w = web.path();
    std::fs::write(
        w.join("package.json"),
        r#"{"devDependencies":{"@elevenlabs/cli":"1"}}"#,
    )
    .unwrap();
    std::fs::write(
        w.join("App.vue"),
        "<script>import x from '@elevenlabs/cli'</script>",
    )
    .unwrap();
    let p = project(w);
    assert!(
        !p.sdk_declared() && !p.sdk_imported(),
        "the CLI is not an SDK"
    );
    std::fs::write(
        w.join("package.json"),
        r#"{"dependencies":{"@elevenlabs/react":"0.8"}}"#,
    )
    .unwrap();
    std::fs::write(
        w.join("App.vue"),
        "<script>import { useConversation } from '@elevenlabs/react'</script>",
    )
    .unwrap();
    assert!(p.sdk_declared() && p.sdk_imported());
    assert!(super::project::imports_js_sdk(
        "import '@elevenlabs/client'"
    ));

    let py = tempfile::tempdir().unwrap();
    std::fs::write(
        py.path().join("pyproject.toml"),
        "dependencies = [\"elevenlabs-extra\"]\n",
    )
    .unwrap();
    std::fs::write(
        py.path().join("app.py"),
        "from elevenlabs.client import ElevenLabs\n",
    )
    .unwrap();
    let p = project(py.path());
    assert!(
        !p.sdk_declared(),
        "another package that starts with the name"
    );
    std::fs::write(
        py.path().join("pyproject.toml"),
        "dependencies = [\"elevenlabs>=2\"]\n",
    )
    .unwrap();
    assert!(p.sdk_declared() && p.sdk_imported());

    let setup = tempfile::tempdir().unwrap();
    std::fs::write(
        setup.path().join("setup.py"),
        "setup(install_requires=[\"elevenlabs\"])\n",
    )
    .unwrap();
    assert!(project(setup.path()).sdk_declared(), "setup.py counts too");
    std::fs::write(setup.path().join("setup.py"), "setup()\n").unwrap();
    std::fs::write(
        setup.path().join("setup.cfg"),
        "[options]\ninstall_requires =\n    elevenlabs>=2\n",
    )
    .unwrap();
    assert!(
        project(setup.path()).sdk_declared(),
        "and so does setup.cfg"
    );

    // A monorepo app declares it in its own folder; a sample beside the app doesn't count.
    let mono = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(mono.path().join("examples/tts")).unwrap();
    std::fs::write(
        mono.path().join("examples/tts/requirements.txt"),
        "elevenlabs\n",
    )
    .unwrap();
    assert!(!project(mono.path()).sdk_declared());
    // The app's own `examples` folder does.
    std::fs::create_dir_all(mono.path().join("app/examples")).unwrap();
    std::fs::write(
        mono.path().join("app/examples/page.tsx"),
        "import { ElevenLabsClient } from \"@elevenlabs/elevenlabs-js\";",
    )
    .unwrap();
    assert!(project(mono.path()).sdk_imported());
    std::fs::create_dir_all(mono.path().join("backend")).unwrap();
    std::fs::write(mono.path().join("backend/requirements.txt"), "elevenlabs\n").unwrap();
    assert!(project(mono.path()).sdk_declared());
}

#[test]
fn the_worker_gets_this_commands_arguments() {
    use super::session::worker_args_from;
    let args = ["onboard", "connect", "--replace", "--launch-worker"].map(std::ffi::OsString::from);
    let out = worker_args_from(args.into_iter(), "worker");
    assert_eq!(
        out,
        ["onboard", "connect", "--replace", "--worker"].map(std::ffi::OsString::from)
    );
}

#[test]
fn a_session_runs_exactly_while_its_worker_holds_the_lock() {
    use super::session::Session;
    let dir = tempfile::tempdir().unwrap();
    let s = Session::in_dir(dir.path().to_path_buf());
    assert!(!s.running(), "nothing started");
    let lock = s.take().unwrap().expect("the first worker takes the lock");
    assert!(s.running());
    assert!(
        s.take().unwrap().is_none(),
        "a second worker for the project stands down"
    );
    s.finish(&lock, &serde_json::json!({ "next": "write_code" }))
        .unwrap();
    drop(lock);
    // A child another test is spawning holds a copy of the lock until it execs.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
    while s.running() && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(!s.running(), "the lock goes with the worker");
    assert_eq!(
        s.wait(std::time::Duration::ZERO, |_| false).unwrap()["next"],
        "write_code",
        "--wait prints the result"
    );
}

#[test]
fn waiting_gives_the_link_while_the_approval_is_open() {
    use super::connect::wait;
    use super::session::Session;
    let dir = tempfile::tempdir().unwrap();
    let p = project(dir.path());
    let s = Session::in_dir(dir.path().join("session"));
    std::fs::create_dir_all(dir.path().join("session")).unwrap();
    let r = wait(&s, &p, std::time::Duration::ZERO, true, false);
    assert_eq!(
        (r["next"].as_str(), r["run"].as_str()),
        (
            Some("run"),
            Some("elevenlabs onboard connect --replace --env-file .env.local")
        ),
        "the restart keeps --replace"
    );

    let _lock = s.take().unwrap().unwrap();
    std::fs::write(
        dir.path().join("session/worker.log"),
        "Opening browser…\n  URL: https://elevenlabs.io/app/oauth/authorize?x=1\n",
    )
    .unwrap();
    let r = wait(&s, &p, std::time::Duration::ZERO, false, false);
    assert_eq!(r["next"], "wait");
    assert!(r["instruction"]
        .as_str()
        .unwrap()
        .contains("without ending your turn"));
    assert_eq!(
        r["run"],
        "elevenlabs onboard connect --wait --env-file .env.local"
    );
    // A line the worker is still writing is not a link yet.
    std::fs::write(dir.path().join("session/worker.log"), "  URL: ").unwrap();
    assert!(s.url().is_none());
    std::fs::write(
        dir.path().join("session/worker.log"),
        "Opening browser…\n  URL: https://elevenlabs.io/app/oauth/authorize?x=1\n",
    )
    .unwrap();
    assert!(r["say"]
        .as_str()
        .unwrap()
        .contains("https://elevenlabs.io/app/oauth/authorize?x=1"));
    let r = wait(&s, &p, std::time::Duration::ZERO, true, false);
    assert_eq!(
        r["run"],
        "elevenlabs onboard connect --replace --wait --env-file .env.local"
    );

    // Without a browser, the first reply hands the link over as soon as there is one.
    let mut p = p;
    p.no_browser = true;
    let started = std::time::Instant::now();
    let r = wait(&s, &p, std::time::Duration::from_secs(20), false, true);
    assert_eq!(r["next"], "wait");
    assert!(started.elapsed() < std::time::Duration::from_secs(5));

    // A sign-in another project replaced: the retry is this command's own.
    std::fs::write(
        dir.path().join("session/result.json"),
        r#"{"next":"ask_user","say":"replaced","details":{"key_status":"sign_in_replaced"}}"#,
    )
    .unwrap();
    let r = wait(&s, &p, std::time::Duration::ZERO, true, false);
    assert_eq!(
        r["run"],
        "elevenlabs onboard connect --replace --env-file .env.local --no-browser"
    );
}

#[cfg(unix)]
#[test]
fn starting_a_sign_in_replaces_another_projects() {
    use super::session::Session;
    isolate();
    let root = tempfile::tempdir().unwrap();
    let other = Session::in_dir(root.path().join("other"));
    std::fs::create_dir_all(root.path().join("other")).unwrap();
    let _held = other.take().unwrap().unwrap();
    // The other worker: a real child process, so only it is signalled.
    let mut worker = std::process::Command::new("sleep")
        .arg("30")
        .spawn()
        .unwrap();
    std::fs::write(
        root.path().join("other/worker.pid"),
        worker.id().to_string(),
    )
    .unwrap();
    assert!(other.running());

    // One past the browser is storing a key the developer approved: left alone.
    let storing = Session::in_dir(root.path().join("storing"));
    std::fs::create_dir_all(root.path().join("storing")).unwrap();
    let storing_lock = storing.take().unwrap().unwrap();
    storing.signed_in(&storing_lock);
    let mut storing_worker = std::process::Command::new("sleep")
        .arg("30")
        .spawn()
        .unwrap();
    std::fs::write(
        root.path().join("storing/worker.pid"),
        storing_worker.id().to_string(),
    )
    .unwrap();

    Session::in_dir(root.path().join("ours")).stop_others();
    assert!(
        !worker.wait().unwrap().success(),
        "the other worker was stopped"
    );
    let told = other.result().expect("the replaced session is told why");
    assert_eq!(told["details"]["key_status"], "sign_in_replaced");
    assert_eq!(told["next"], "ask_user");
    assert!(told["instruction"].as_str().is_some());
    assert!(
        storing_worker.try_wait().unwrap().is_none() && storing.result().is_none(),
        "the one storing its key keeps going"
    );
    storing_worker.kill().unwrap();
    let _ = storing_worker.wait();
}

#[test]
fn a_reply_names_the_next_step_and_the_command() {
    use super::reply::{Next, Reply};
    let v = Reply::new(Next::WriteCode, "Signed in.")
        .run("check")
        .with_rules()
        .detail("key_last4", "abcd")
        .to_value();
    assert_eq!(v["next"], "write_code");
    assert!(v["instruction"]
        .as_str()
        .unwrap()
        .contains("run `run` to check it"));
    assert_eq!(v["run"], "elevenlabs onboard check");
    assert!(v["rules"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r.as_str().unwrap().contains("git clean")));
    assert_eq!(v["details"]["key_last4"], "abcd");
    let bare = Reply::new(Next::Done, "Done.").to_value();
    assert!(
        bare.get("run").is_none() && bare.get("rules").is_none() && bare.get("details").is_none()
    );
}

#[test]
fn a_failed_sign_in_says_what_to_do_and_keeps_its_flags() {
    use super::connect::failed;
    let dir = tempfile::tempdir().unwrap();
    let mut p = project(dir.path());
    p.no_browser = true;
    // The sign-in flow's own wording for a declined approval.
    let denied = failed(
        &p,
        "Authorization server returned error: access_denied (declined)",
        true,
    )
    .to_value();
    assert_eq!(denied["next"], "ask_user");
    assert_eq!(
        denied["run"],
        "elevenlabs onboard connect --replace --env-file .env.local --no-browser"
    );
    assert!(
        denied["say"]
            .as_str()
            .unwrap()
            .contains("app/settings/api-keys"),
        "offers the manual path too"
    );
    let late = failed(
        &p,
        "Timed out waiting for the OAuth callback after 300s.",
        false,
    )
    .to_value();
    assert_eq!(late["next"], "run");
    assert!(late["say"].as_str().unwrap().contains("5 minutes"));
    let other = failed(&p, "state mismatch", false).to_value();
    assert!(other["say"].as_str().unwrap().contains("state mismatch"));
    assert_eq!(other["details"]["key_status"], "sign_in_failed");
}

#[test]
fn every_clipboard_outcome_has_its_reply() {
    use super::connect::{outcome_of, reply_for, Outcome};
    let key = "sk_0123456789abcdef0123456789abcdef01234567";
    let ok = |_: &str| Ok(true);
    assert_eq!(
        outcome_of(Clip::Key(key.into()), ok),
        Outcome::Store(key.into())
    );
    let other = outcome_of(Clip::Key(key.into()), |_| Ok(false));
    assert!(matches!(
        other,
        Outcome::Manual {
            status: "clipboard_invalid",
            ..
        }
    ));
    let unknown = outcome_of(Clip::Key(key.into()), |_| {
        Err(fern_cli_sdk::error::CliError::Validation("x".into()))
    });
    assert!(matches!(
        unknown,
        Outcome::Manual {
            status: "could_not_verify",
            ..
        }
    ));
    assert_eq!(
        outcome_of(Clip::Marker(Marker::NotPermitted), ok),
        Outcome::NotPermitted
    );
    assert!(matches!(
        outcome_of(Clip::Marker(Marker::NotAvailable), ok),
        Outcome::Manual {
            status: "not_available",
            ..
        }
    ));
    assert!(matches!(
        outcome_of(Clip::Marker(Marker::Failed), ok),
        Outcome::Manual {
            status: "failed",
            ..
        }
    ));
    assert!(matches!(
        outcome_of(Clip::Nothing, ok),
        Outcome::Manual {
            status: "no_key",
            ..
        }
    ));

    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    let p = project(dir.path());
    let stored = reply_for(&p, &Outcome::Store(key.into()))
        .unwrap()
        .to_value();
    assert_eq!(stored["next"], "write_code");
    assert_eq!(stored["details"]["key_status"], "created");
    assert_eq!(stored["details"]["key_last4"], "4567");
    assert!(stored["details"]["load_key"]
        .as_str()
        .unwrap()
        .contains(".env.local"));
    assert!(
        !stored.to_string().contains(key),
        "the key is never in a reply"
    );
    let env = std::fs::read_to_string(dir.path().join(".env.local")).unwrap();
    assert!(env.contains(key));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(dir.path().join(".env.local"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    let admin = reply_for(&p, &Outcome::NotPermitted).unwrap().to_value();
    assert_eq!(
        (
            admin["next"].as_str(),
            admin["details"]["key_status"].as_str()
        ),
        (Some("ask_user"), Some("not_permitted"))
    );
    let manual = reply_for(
        &p,
        &Outcome::Manual {
            status: "no_key",
            why: "no key reached this command",
        },
    )
    .unwrap()
    .to_value();
    assert_eq!(manual["next"], "ask_user");
    assert!(manual["say"]
        .as_str()
        .unwrap()
        .contains("The old key (ending in 4567) is still in .env.local"));
    assert_eq!(
        manual["run"],
        "elevenlabs onboard check --env-file .env.local"
    );

    // A first key in an app that already uses the SDK still gets the code
    // details (how to load the key); a replaced one goes straight to check.
    let fresh = tempfile::tempdir().unwrap();
    git(fresh.path(), &["init", "-q"]);
    std::fs::write(
        fresh.path().join("package.json"),
        r#"{"dependencies":{"@elevenlabs/elevenlabs-js":"2"}}"#,
    )
    .unwrap();
    std::fs::write(
        fresh.path().join("index.js"),
        "import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';",
    )
    .unwrap();
    let first = reply_for(&project(fresh.path()), &Outcome::Store(key.into()))
        .unwrap()
        .to_value();
    assert_eq!(first["next"], "write_code");
    std::fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies":{"@elevenlabs/elevenlabs-js":"2"}}"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("index.js"),
        "import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';",
    )
    .unwrap();
    let replaced = reply_for(&p, &Outcome::Store(key.into()))
        .unwrap()
        .to_value();
    assert_eq!(
        (replaced["next"].as_str(), replaced["run"].as_str()),
        (
            Some("run"),
            Some("elevenlabs onboard check --env-file .env.local")
        )
    );
    assert_eq!(replaced["details"]["key_status"], "created");
}

#[test]
fn the_manual_path_prepares_the_named_file_and_pins_it_for_check() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    std::fs::create_dir_all(dir.path().join("config")).unwrap();
    let p = super::project::Project::at(dir.path().to_path_buf(), Some("config/.env.dev".into()));
    let v = super::manual::reply(&p, &p.env_file, "No browser.")
        .unwrap()
        .to_value();
    assert_eq!(v["next"], "ask_user");
    assert_eq!(
        v["run"],
        "elevenlabs onboard check --env-file config/.env.dev"
    );
    let say = v["say"].as_str().unwrap();
    assert!(
        say.starts_with("No browser.")
            && say.contains("in config/.env.dev")
            && say.contains("Never paste it into this chat")
    );
    let env = std::fs::read_to_string(dir.path().join("config/.env.dev")).unwrap();
    assert!(env.ends_with("ELEVENLABS_API_KEY=\n"));
    assert!(
        std::fs::read_to_string(dir.path().join("config/.gitignore"))
            .unwrap()
            .contains("/.env.dev")
    );
    super::manual::reply(&p, &p.env_file, "No browser.").unwrap();
    assert_eq!(
        std::fs::read_to_string(dir.path().join("config/.env.dev")).unwrap(),
        env,
        "nothing duplicated"
    );
    let pending = super::check::no_key(&p).unwrap().to_value();
    assert_eq!(pending["next"], "ask_user");
    assert!(pending["say"].as_str().unwrap().contains("still empty"));
}

#[test]
fn gitignore_entries_match_only_the_env_file() {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    let odd = dir.path().join("keys[dev].env");
    std::fs::write(&odd, "x").unwrap();
    assert!(super::env_file::ensure_ignored(&odd).unwrap(), "added");
    let other = dir.path().join("keysd.env");
    std::fs::write(&other, "x").unwrap();
    assert!(
        !super::env_file::ensure_ignored(&odd).unwrap(),
        "already ignored"
    );
    let status = std::process::Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["status", "--porcelain"])
        .output()
        .unwrap();
    let status = String::from_utf8_lossy(&status.stdout);
    assert!(
        status.contains("keysd.env") && !status.contains("keys[dev].env"),
        "{status}"
    );
}
