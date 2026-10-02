use super::super::util::opt_string;
use super::auth::*;
use super::browser::reachable_in;
use super::clipboard::{classify, next_for, Clip};
use super::env_file::{file_key, with_key, PLACEHOLDER_NOTE};
use super::{last4, onboard_command};

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
    assert!(matches!(
        classify(Some("elevenlabs:not_available".into())),
        Clip::Marker("not_available")
    ));
    assert!(matches!(
        classify(Some("elevenlabs:not_permitted".into())),
        Clip::Marker("not_permitted")
    ));
    assert!(matches!(
        classify(Some("elevenlabs:failed".into())),
        Clip::Marker("failed")
    ));
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
    assert_eq!(next_for("created"), "none");
    assert_eq!(next_for("not_available"), "legacy");
    assert_eq!(next_for("not_permitted"), "legacy");
    assert_eq!(next_for("failed"), "legacy");
    for s in ["no_key", "clipboard_invalid"] {
        assert_eq!(next_for(s), "paste_key");
    }
    assert_eq!(super::clipboard::marker_text("failed"), "elevenlabs:failed");
}

#[test]
fn the_nearest_dotenv_with_a_key_is_found_up_the_tree() {
    use super::env_file::dotenv_with_key;
    let root = tempfile::tempdir().unwrap();
    let nested = root.path().join("apps").join("web");
    std::fs::create_dir_all(&nested).unwrap();
    assert!(dotenv_with_key(&nested).is_none(), "no .env anywhere");

    std::fs::write(
        root.path().join(".env"),
        "ELEVENLABS_API_KEY=sk_root
",
    )
    .unwrap();
    let (path, key) = dotenv_with_key(&nested).expect("the root .env");
    assert_eq!(path, root.path().join(".env"));
    assert_eq!(key, "sk_root");

    // The nearest file wins, and an empty line is not a key.
    std::fs::write(
        nested.join(".env"),
        "ELEVENLABS_API_KEY=
",
    )
    .unwrap();
    assert!(
        dotenv_with_key(&nested).is_none(),
        "nearest .env has no key"
    );
    std::fs::write(
        nested.join(".env"),
        "OTHER=1
ELEVENLABS_API_KEY=sk_near
",
    )
    .unwrap();
    assert_eq!(dotenv_with_key(&nested).unwrap().1, "sk_near");
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

#[test]
fn the_command_parses_its_flags_and_refuses_unknown_products() {
    let cmd = onboard_command();
    let m = cmd
        .clone()
        .try_get_matches_from(["onboard", "--product", "music", "--replace"])
        .unwrap();
    assert_eq!(opt_string(&m, "product").as_deref(), Some("music"));
    assert!(m.get_flag("replace"));
    assert!(cmd
        .clone()
        .try_get_matches_from(["onboard", "--product", "tts"])
        .is_err());
    let status = cmd
        .try_get_matches_from(["onboard", "status", "--env-file", ".env"])
        .unwrap();
    assert!(matches!(status.subcommand(), Some(("status", _))));
}
