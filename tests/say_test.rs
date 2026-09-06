//! Integration tests for `elevenlabs say`, run against a local mock server.
//!
//! These cover the one thing the inline unit tests in
//! `cli/elevenlabs/workflow/say.rs` cannot: that the audio player is spawned
//! only after the HTTP status has been checked. The SDK's
//! `text_to_speech.stream()` returns a `ByteStream` that discards the status,
//! and `CliExecutor` does not reject non-2xx on the executor path — so the
//! obvious implementation pipes a JSON error body into the user's audio
//! player. `say` sends and validates first, then spawns; if anyone
//! "simplifies" that apart again, `a_failed_request_never_reaches_the_player`
//! is what notices.
//!
//! No network: `wiremock` binds localhost, and `HOME` is redirected at a temp
//! dir so a developer's own `~/.elevenlabs/config.json` cannot change what the
//! CLI asks for.

use std::path::Path;
use std::process::{Command, Output};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A recognizable WAV-ish payload. Only the bytes matter here — nothing
/// decodes it.
const AUDIO: &[u8] = b"RIFF\x24\x00\x00\x00WAVEfmt ELEVENLABS-SAY-TEST-PAYLOAD";

const VOICE_ID: &str = "JBFqnCBsd6RMkjVDRZzb";

/// Run the CLI with `HOME` redirected at a temp dir, so it cannot read or
/// write the developer's real `~/.elevenlabs/config.json`.
fn cli(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_elevenlabs"))
        .args(args)
        .env("HOME", home)
        .env("ELEVENLABS_API_KEY", "test-key")
        .env("NO_COLOR", "1")
        // The OS keyring is not reachable (or not unlocked) on CI runners, and
        // probing it can block; the file store needs no user interaction.
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env_remove("ELEVENLABS_BASE_URL")
        .env_remove("ELEVENLABS_VIA")
        .output()
        .expect("failed to spawn the elevenlabs binary")
}

/// Write a stand-in player that records the fact it ran. A real player needs
/// an audio device, which CI does not have.
#[cfg(unix)]
fn fake_player(dir: &Path, marker: &Path) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let script = dir.join("fake-player");
    std::fs::write(
        &script,
        format!(
            "#!/bin/sh\n# Consume stdin so a streaming caller is not left blocked.\ncat > /dev/null 2>&1\nprintf 'played' > '{}'\n",
            marker.display()
        ),
    )
    .expect("write fake player");
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))
        .expect("chmod fake player");
    script
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn a_failed_request_never_reaches_the_player() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE_ID}/stream")))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "detail": { "status": "invalid_api_key", "message": "Invalid API key" }
        })))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let marker = home.path().join("played.marker");
    let player = fake_player(home.path(), &marker);
    let out_file = home.path().join("should-not-exist.mp3");

    let out = cli(
        home.path(),
        &[
            "say",
            "hello",
            "--base-url",
            &server.uri(),
            "--player",
            player.to_str().unwrap(),
        ],
    );

    assert_eq!(
        out.status.code(),
        Some(1),
        "a 401 should exit with the API error code, got {:?}\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
    );
    // The framework owns which stream the error envelope lands on; assert on
    // the content, not the plumbing.
    let reported = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        reported.contains("Invalid API key") && reported.contains("401"),
        "the API's own message should survive, got: {reported}"
    );
    assert!(
        !marker.exists(),
        "the player must not run for a failed request — an error body would be \
         handed to it as if it were audio"
    );

    // The same guarantee for --output: no half-written file of JSON.
    let out = cli(
        home.path(),
        &[
            "say",
            "hello",
            "--base-url",
            &server.uri(),
            "--output",
            out_file.to_str().unwrap(),
        ],
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(
        !out_file.exists(),
        "a failed request must not leave a file behind"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn a_successful_request_streams_the_audio_to_the_player() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE_ID}/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(AUDIO))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let marker = home.path().join("played.marker");
    let player = fake_player(home.path(), &marker);

    let out = cli(
        home.path(),
        &[
            "say",
            "this came from the terminal",
            "--base-url",
            &server.uri(),
            "--player",
            player.to_str().unwrap(),
        ],
    );

    assert!(
        out.status.success(),
        "say failed (exit {:?}): {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(marker.exists(), "the player should have run");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn the_response_body_is_written_verbatim_to_output() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{VOICE_ID}/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(AUDIO))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let out_file = home.path().join("out.mp3");

    let out = cli(
        home.path(),
        &[
            "say",
            "hello",
            "--base-url",
            &server.uri(),
            "--output",
            out_file.to_str().unwrap(),
        ],
    );

    assert!(
        out.status.success(),
        "say --output failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        std::fs::read(&out_file).expect("read output"),
        AUDIO,
        "every chunk should reach the file unmodified"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread")]
async fn stored_defaults_drive_the_request() {
    let server = MockServer::start().await;
    let custom_voice = "21m00Tcm4TlvDq8ikWAM";
    Mock::given(method("POST"))
        .and(path(format!("/v1/text-to-speech/{custom_voice}/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(AUDIO))
        .mount(&server)
        .await;

    let home = tempfile::tempdir().expect("tempdir");
    let out_file = home.path().join("out.wav");

    assert!(cli(home.path(), &["say", "config", "voice", custom_voice])
        .status
        .success());
    assert!(cli(
        home.path(),
        &["say", "config", "model", "eleven_multilingual_v2"]
    )
    .status
    .success());

    // The voice only reaches the URL if the stored default was read, so a
    // matched mock is the assertion.
    let out = cli(
        home.path(),
        &[
            "say",
            "hello",
            "--base-url",
            &server.uri(),
            "--output",
            out_file.to_str().unwrap(),
        ],
    );
    assert!(
        out.status.success(),
        "the stored voice should have been used: {}",
        String::from_utf8_lossy(&out.stderr),
    );

    let requests = server.received_requests().await.expect("recorded requests");
    let body: serde_json::Value =
        serde_json::from_slice(&requests[0].body).expect("request body is JSON");
    assert_eq!(body["model_id"], "eleven_multilingual_v2");
    assert_eq!(body["text"], "hello");

    // `say config` must never disturb the residency key it shares a file with.
    assert!(cli(home.path(), &["residency", "eu-residency"])
        .status
        .success());
    assert!(cli(home.path(), &["say", "config", "voice", "--unset"])
        .status
        .success());
    let config: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(home.path().join(".elevenlabs/config.json")).expect("read config"),
    )
    .expect("config is JSON");
    assert_eq!(config["residency"], "eu-residency");
    assert_eq!(config["say"]["model_id"], "eleven_multilingual_v2");
    assert!(config["say"].get("voice_id").is_none());
}
