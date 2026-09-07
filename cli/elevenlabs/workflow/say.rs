//! The `say` command: turn text into speech and play it, straight from
//! the terminal.
//!
//! `elevenlabs text-to-speech convert` can already reach the API, but it
//! makes you supply a voice ID, a model ID, an output format and a file
//! path every time, and then find your own player. `say` remembers the
//! first four in `~/.elevenlabs/config.json` (via [`super::settings`],
//! the same file `residency` writes) and pipes the audio into whatever
//! player is on the box.
//!
//! Playback shells out rather than linking an audio stack: `rodio` pulls
//! in `cpal` -> `alsa-sys`, which needs `libasound2-dev` at build time and
//! would break the static musl targets this workspace distributes (see the
//! `cfg(target_env = "musl")` gates in Cargo.toml). Spawning is done the
//! way `super::components` does it — argv passed individually, never
//! through a shell.

use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use clap::{Args as _, FromArgMatches as _};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use fern_cli_sdk::sdk_executor::SdkRequestExecutor;
use futures_util::StreamExt;
use serde_json::{json, Value};

use super::settings;

/// George — the voice the co-generated SDK uses in its own doctests, so a
/// first run with no config still produces something sensible.
const DEFAULT_VOICE_ID: &str = "JBFqnCBsd6RMkjVDRZzb";
/// The most expressive model, and the only family that honors audio tags
/// (`[whispers]`, `[laughs]`) rather than reading them aloud as text. Costs
/// roughly 0.8s more to first audio than `eleven_flash_v2_5`, which stays the
/// swap when speed matters more than delivery:
/// `elevenlabs say config model eleven_flash_v2_5`.
const DEFAULT_MODEL_ID: &str = "eleven_v3";

const MP3_FORMAT: &str = "mp3_44100_128";
const WAV_FORMAT: &str = "wav_44100";

/// Section holding `say`'s settings inside `~/.elevenlabs/config.json`.
const SECTION: &str = "say";

// ── Settings ────────────────────────────────────────────────────────

/// One configurable default, as the user types it and as it is stored.
struct SettingSpec {
    /// What `elevenlabs say config <key> <value>` accepts.
    cli_key: &'static str,
    /// Key inside the `say` object in the config file.
    json_key: &'static str,
    label: &'static str,
    /// Built-in fallback, or `None` when the value is derived at runtime.
    default: Option<&'static str>,
}

const SETTINGS: &[SettingSpec] = &[
    SettingSpec {
        cli_key: "voice",
        json_key: "voice_id",
        label: "Voice",
        default: Some(DEFAULT_VOICE_ID),
    },
    SettingSpec {
        cli_key: "model",
        json_key: "model_id",
        label: "Model",
        default: Some(DEFAULT_MODEL_ID),
    },
    SettingSpec {
        cli_key: "output-format",
        json_key: "output_format",
        label: "Format",
        default: None,
    },
    SettingSpec {
        cli_key: "player",
        json_key: "player",
        label: "Player",
        default: None,
    },
];

fn spec_for(cli_key: &str) -> Option<&'static SettingSpec> {
    SETTINGS.iter().find(|s| s.cli_key == cli_key)
}

fn read_configured(json_key: &str) -> Option<String> {
    settings::read_setting(&[SECTION, json_key]).filter(|v| !v.trim().is_empty())
}

/// Flag > config file > built-in default. A whitespace-only value counts as
/// unset at every layer — the same trap `residency::base_url_to_export`
/// guards, since an empty voice ID would build a URL like `/v1/…//stream`.
fn resolve<'a>(flag: Option<&'a str>, configured: Option<&'a str>, default: &'a str) -> &'a str {
    for candidate in [flag, configured] {
        if let Some(v) = candidate {
            if !v.trim().is_empty() {
                return v.trim();
            }
        }
    }
    default
}

// ── Output formats ──────────────────────────────────────────────────

/// The codec half of an `output_format` (`mp3_44100_128` -> `mp3`).
///
/// Doubles as validation: rather than pinning the full enum from
/// `elevenlabs-types` — which would reject any format the API adds before
/// the next regeneration — this accepts a known codec plus numeric
/// parameters, which catches typos without blocking new formats.
fn codec_of(format: &str) -> Option<&'static str> {
    let (codec, rest) = format.split_once('_')?;
    let codec = match codec {
        "mp3" => "mp3",
        "wav" => "wav",
        "pcm" => "pcm",
        "opus" => "opus",
        "ulaw" => "ulaw",
        "alaw" => "alaw",
        _ => return None,
    };
    let numeric = !rest.is_empty()
        && rest
            .split('_')
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()));
    numeric.then_some(codec)
}

/// File extension to write for a format, so `afplay` and PowerShell's
/// `SoundPlayer` pick the right decoder from the name.
fn extension_for(format: &str) -> &'static str {
    match codec_of(format) {
        Some("wav") => "wav",
        Some("opus") => "opus",
        Some("pcm") | Some("ulaw") | Some("alaw") => "raw",
        _ => "mp3",
    }
}

/// The format implied by an output filename, when the user did not name one.
fn format_from_extension(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "mp3" => Some(MP3_FORMAT),
        "wav" => Some(WAV_FORMAT),
        "opus" | "ogg" => Some("opus_48000_128"),
        "pcm" | "raw" => Some("pcm_44100"),
        "ulaw" => Some("ulaw_8000"),
        "alaw" => Some("alaw_8000"),
        _ => None,
    }
}

/// Pick the wire format.
///
/// An explicit `--output-format` always wins. Otherwise a `--output`
/// filename decides, ahead of the configured default — writing `out.wav`
/// and getting MP3 bytes inside it is worse than ignoring the config. With
/// neither, the player decides: `aplay`/`paplay`/`SoundPlayer` only handle
/// WAV.
fn output_format_for(
    explicit: Option<&str>,
    configured: Option<&str>,
    output: Option<&Path>,
    container: Container,
) -> String {
    if let Some(fmt) = explicit.map(str::trim).filter(|f| !f.is_empty()) {
        return fmt.to_string();
    }
    if let Some(fmt) = output.and_then(format_from_extension) {
        return fmt.to_string();
    }
    if let Some(fmt) = configured.map(str::trim).filter(|f| !f.is_empty()) {
        return fmt.to_string();
    }
    match container {
        Container::WavOnly => WAV_FORMAT.to_string(),
        Container::Any => MP3_FORMAT.to_string(),
    }
}

// ── Players ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Container {
    /// Decodes whatever we send it.
    Any,
    /// WAV only — the format has to follow.
    WavOnly,
}

/// A player we know how to drive.
#[derive(Clone, Copy, Debug)]
struct PlayerSpec {
    program: &'static str,
    /// Fixed arguments that precede the target.
    args: &'static [&'static str],
    /// Reads audio from stdin (so it can start before the download finishes).
    reads_stdin: bool,
    container: Container,
}

/// Preference order. stdin-capable players come first: they are what makes
/// streaming pay off, since playback starts on the first chunk instead of
/// after the last one.
const PLAYERS: &[PlayerSpec] = &[
    PlayerSpec {
        program: "ffplay",
        args: &["-nodisp", "-autoexit", "-loglevel", "error", "-i"],
        reads_stdin: true,
        container: Container::Any,
    },
    PlayerSpec {
        program: "mpv",
        args: &["--no-video", "--really-quiet"],
        reads_stdin: true,
        container: Container::Any,
    },
    PlayerSpec {
        program: "afplay",
        args: &[],
        reads_stdin: false,
        container: Container::Any,
    },
    PlayerSpec {
        program: "paplay",
        args: &[],
        reads_stdin: false,
        container: Container::WavOnly,
    },
    PlayerSpec {
        program: "aplay",
        args: &["-q"],
        reads_stdin: false,
        container: Container::WavOnly,
    },
    PlayerSpec {
        program: "powershell",
        args: &[],
        reads_stdin: false,
        container: Container::WavOnly,
    },
];

fn spec_for_program(program: &str) -> Option<&'static PlayerSpec> {
    PLAYERS.iter().find(|p| p.program == program)
}

/// Players worth probing on this OS. `powershell` is Windows-only —
/// PowerShell Core exists on Linux but `Media.SoundPlayer` does not.
fn candidates() -> impl Iterator<Item = &'static PlayerSpec> {
    PLAYERS.iter().filter(|p| match p.program {
        "powershell" => cfg!(windows),
        "afplay" => cfg!(target_os = "macos"),
        _ => true,
    })
}

/// A player chosen for this run. Owns its program name because `--player`
/// and the config file supply names we do not know at compile time.
#[derive(Clone, Debug)]
struct Player {
    program: String,
    args: Vec<String>,
    reads_stdin: bool,
    container: Container,
}

impl Player {
    fn from_spec(spec: &PlayerSpec) -> Self {
        Self {
            program: spec.program.to_string(),
            args: spec.args.iter().map(|s| (*s).to_string()).collect(),
            reads_stdin: spec.reads_stdin,
            container: spec.container,
        }
    }

    /// A player the user named that is not in [`PLAYERS`]. We know nothing
    /// about it, so assume the conservative shape: a file argument, and a
    /// format it is most likely to understand.
    fn unknown(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            reads_stdin: false,
            container: Container::Any,
        }
    }
}

/// Is `program` runnable? Mirrors what `Command::spawn` will resolve, so a
/// missing player is reported before any API call is made.
fn on_path(program: &str) -> bool {
    let has_separator = program.contains('/') || (cfg!(windows) && program.contains('\\'));
    if has_separator {
        return Path::new(program).is_file();
    }
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| {
        if dir.as_os_str().is_empty() {
            return false;
        }
        if dir.join(program).is_file() {
            return true;
        }
        // Windows resolves bare names through PATHEXT.
        cfg!(windows) && dir.join(format!("{program}.exe")).is_file()
    })
}

/// `--player` > configured player > the first candidate on PATH.
///
/// A name the user supplied is honored even if we do not recognize it, so
/// `--player my-wrapper` works; only "nothing at all" is an error.
fn resolve_player(explicit: Option<&str>, configured: Option<&str>) -> Result<Player, CliError> {
    let named = [explicit, configured]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|v| !v.is_empty());

    if let Some(name) = named {
        if !on_path(name) {
            return Err(CliError::Validation(format!(
                "Audio player '{name}' was not found on PATH. Install it, pass \
                 --player <command>, or clear the default with \
                 'elevenlabs say config player --unset'."
            )));
        }
        return Ok(spec_for_program(name)
            .map(Player::from_spec)
            .unwrap_or_else(|| Player::unknown(name)));
    }

    candidates()
        .find(|spec| on_path(spec.program))
        .map(Player::from_spec)
        .ok_or_else(|| {
            CliError::Validation(format!(
                "No audio player found. Install ffmpeg (for ffplay) or mpv, or pass \
                 --player <command>. Looked for: {}. To skip playback entirely, use \
                 --output <path>.",
                candidates()
                    .map(|s| s.program)
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

/// Build the argv for a player invocation.
///
/// Arguments are passed individually and never through a shell, so nothing
/// in `target` is re-parsed. PowerShell is the one exception in shape — it
/// takes a script string — so the path is embedded in a single-quoted
/// literal with `'` doubled, PowerShell's own escape.
fn player_argv(player: &Player, target: &str) -> Vec<String> {
    if player.program == "powershell" {
        let quoted = target.replace('\'', "''");
        return vec![
            "-NoProfile".to_string(),
            "-Command".to_string(),
            format!("(New-Object Media.SoundPlayer '{quoted}').PlaySync()"),
        ];
    }
    let mut argv = player.args.clone();
    argv.push(target.to_string());
    argv
}

fn spawn_player(player: &Player, target: &str, stdin: Stdio) -> Result<Child, CliError> {
    Command::new(&player.program)
        .args(player_argv(player, target))
        .stdin(stdin)
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                CliError::Validation(format!(
                    "Audio player '{}' was not found on PATH.",
                    player.program
                ))
            } else {
                CliError::Other(anyhow::anyhow!("Could not run {}: {e}", player.program))
            }
        })
}

// ── Text input ──────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
enum TextSource {
    Literal(String),
    Stdin,
}

/// Where the text comes from: the positional arguments, or stdin when the
/// user wrote `-` or piped something in.
fn text_source(positional: &[String], stdin_is_tty: bool) -> Result<TextSource, CliError> {
    if positional.len() == 1 && positional[0] == "-" {
        return Ok(TextSource::Stdin);
    }
    if positional.is_empty() {
        return if stdin_is_tty {
            Err(CliError::Validation(
                "No text to speak. Try: elevenlabs say \"hello from the terminal\" \
                 (or pipe text in, or pass -)."
                    .to_string(),
            ))
        } else {
            Ok(TextSource::Stdin)
        };
    }
    Ok(TextSource::Literal(positional.join(" ")))
}

fn read_text(source: TextSource) -> Result<String, CliError> {
    let text = match source {
        TextSource::Literal(t) => t,
        TextSource::Stdin => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| CliError::Other(anyhow::anyhow!("Could not read stdin: {e}")))?;
            buf
        }
    };
    if text.trim().is_empty() {
        return Err(CliError::Validation(
            "Nothing to speak — the text was empty.".to_string(),
        ));
    }
    Ok(text)
}

// ── The request ─────────────────────────────────────────────────────

/// Run a future to completion from a synchronous handler.
///
/// Same shape as `fern_cli_sdk::sdk_executor::block_on` — `block_in_place`
/// parks this worker thread so a nested `block_on` is legal — but generic
/// over the output so the body can return [`CliError`] directly instead of
/// being forced through `SdkError`.
fn run_async<F: std::future::Future>(future: F) -> F::Output {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(future))
}

/// Send the request and validate the status, returning the response with
/// its body still unread.
///
/// This goes through the CLI executor directly rather than
/// `client.text_to_speech.stream(...)`, which returns a `ByteStream` that
/// *discards the HTTP status*. `CliExecutor` does not reject non-2xx on the
/// executor path (see the note in `super::api`, where a 401 once surfaced
/// as "no agents found" with exit 0) — so with the SDK method a 401 would
/// be piped into the audio player as JSON.
///
/// Kept separate from [`pump`] so the status is known *before* a player is
/// spawned: a failed request must never flash an audio window or make
/// ffplay complain about the JSON error body.
fn start_stream(
    ctx: &AppContext,
    voice_id: &str,
    body: Value,
    output_format: &str,
) -> Result<reqwest::Response, CliError> {
    let base = elevenlabs_sdk::ClientConfig::default().base_url;
    let url = format!(
        "{}/v1/text-to-speech/{}/stream",
        base.trim_end_matches('/'),
        percent_encoding::utf8_percent_encode(voice_id, percent_encoding::NON_ALPHANUMERIC),
    );
    // The executor rewrites scheme/host/port from the resolved base URL, so
    // residency, ELEVENLABS_BASE_URL and --base-url all still win over the
    // default host used here.
    let request = reqwest::Client::new()
        .post(url)
        .query(&[("output_format", output_format)])
        .json(&body)
        .build()
        .map_err(|e| CliError::Other(anyhow::anyhow!("Could not build the request: {e}")))?;

    let executor = ctx.build_sdk_executor();

    run_async(async move {
        let response = SdkRequestExecutor::execute(&*executor, request)
            .await
            .map_err(|e| e.into_cli_error())?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }
        let body = response.bytes().await.unwrap_or_default();
        let parsed: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
        Err(CliError::Api {
            code: status.as_u16(),
            message: super::api::api_error_message(&parsed),
            reason: format!("http_{}", status.as_u16()),
            details: (!parsed.is_null()).then_some(parsed),
            help: None,
        })
    })
}

/// Copy an already-validated response body into `sink`, chunk by chunk.
fn pump(response: reqwest::Response, sink: &mut dyn Write) -> Result<(), CliError> {
    run_async(async move {
        let mut chunks = Box::pin(response.bytes_stream());
        while let Some(chunk) = chunks.next().await {
            let chunk =
                chunk.map_err(|e| CliError::Network(format!("Audio stream failed: {e}")))?;
            match sink.write_all(&chunk) {
                Ok(()) => {}
                // The player was closed (q in ffplay, Ctrl-C in mpv). That is
                // a normal way to stop listening, not a failure.
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => return Ok(()),
                Err(e) => {
                    return Err(CliError::Other(anyhow::anyhow!(
                        "Could not write audio: {e}"
                    )))
                }
            }
        }
        sink.flush()
            .map_err(|e| CliError::Other(anyhow::anyhow!("Could not flush audio: {e}")))
    })
}

// ── `say` ───────────────────────────────────────────────────────────

#[derive(clap::Args)]
struct SayArgs {
    /// Text to speak. Pass `-`, or pipe text in, to read from stdin.
    #[arg(value_name = "TEXT")]
    text: Vec<String>,

    /// Voice ID for this run. Overrides `elevenlabs say config voice`.
    #[arg(long, value_name = "ID")]
    voice: Option<String>,

    /// Model ID for this run. Overrides `elevenlabs say config model`.
    #[arg(long, value_name = "ID")]
    model: Option<String>,

    /// Audio format, e.g. mp3_44100_128 or wav_44100.
    #[arg(long, value_name = "FORMAT")]
    output_format: Option<String>,

    /// Write the audio to a file instead of playing it.
    #[arg(short = 'o', long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Player command to use, e.g. ffplay, mpv, afplay.
    #[arg(long, value_name = "COMMAND")]
    player: Option<String>,
}

fn handle_say(args: SayArgs, ctx: &AppContext) -> Result<(), CliError> {
    let text = read_text(text_source(&args.text, std::io::stdin().is_terminal())?)?;

    let configured_voice = read_configured("voice_id");
    let configured_model = read_configured("model_id");
    let configured_format = read_configured("output_format");
    let configured_player = read_configured("player");

    let voice_id = resolve(
        args.voice.as_deref(),
        configured_voice.as_deref(),
        DEFAULT_VOICE_ID,
    )
    .to_string();
    let model_id = resolve(
        args.model.as_deref(),
        configured_model.as_deref(),
        DEFAULT_MODEL_ID,
    )
    .to_string();

    // Resolve the player before generating anything: "no player installed"
    // should not cost an API call. Skipped entirely for --output.
    let player = match args.output {
        Some(_) => None,
        None => Some(resolve_player(
            args.player.as_deref(),
            configured_player.as_deref(),
        )?),
    };

    let output_format = output_format_for(
        args.output_format.as_deref(),
        configured_format.as_deref(),
        args.output.as_deref(),
        player.as_ref().map_or(Container::Any, |p| p.container),
    );
    if codec_of(&output_format).is_none() {
        return Err(CliError::Validation(format!(
            "Invalid output format '{output_format}'. Expected something like \
             mp3_44100_128, wav_44100 or pcm_24000."
        )));
    }
    if let Some(p) = &player {
        if p.container == Container::WavOnly && codec_of(&output_format) != Some("wav") {
            return Err(CliError::Validation(format!(
                "The '{}' player only handles WAV, but the format is '{output_format}'. \
                 Use --output-format {WAV_FORMAT}, or a player that decodes it (ffplay, mpv).",
                p.program
            )));
        }
    }

    let body = json!({ "text": text, "model_id": model_id });

    // Tag the User-Agent with `cmd/say` for the duration of this command,
    // the way every other hand-written command does.
    let _scope = super::api::command_scope("say");

    // Send and check the status before anything is spawned or created, so a
    // 401 costs the user nothing but an error message.
    let response = start_stream(ctx, &voice_id, body, &output_format)?;

    match (&args.output, &player) {
        (Some(path), _) => {
            let mut file = std::fs::File::create(path).map_err(|e| {
                CliError::Other(anyhow::anyhow!("Could not create {}: {e}", path.display()))
            })?;
            pump(response, &mut file)?;
            eprintln!("Saved to {}", path.display());
            Ok(())
        }
        (None, Some(player)) if player.reads_stdin => {
            let mut child = spawn_player(player, "-", Stdio::piped())?;
            let mut stdin = child
                .stdin
                .take()
                .ok_or_else(|| CliError::Other(anyhow::anyhow!("Player stdin was not piped")))?;
            let result = pump(response, &mut stdin);
            // Closing stdin is what tells the player the stream ended.
            drop(stdin);
            let wait = child.wait();
            result?;
            wait.map_err(|e| {
                CliError::Other(anyhow::anyhow!(
                    "Could not wait for {}: {e}",
                    player.program
                ))
            })?;
            Ok(())
        }
        (None, Some(player)) => {
            // File-based player: buffer the stream, then play the file.
            let mut temp = tempfile::Builder::new()
                .prefix("elevenlabs-say-")
                .suffix(&format!(".{}", extension_for(&output_format)))
                .tempfile()
                .map_err(|e| {
                    CliError::Other(anyhow::anyhow!("Could not create a temporary file: {e}"))
                })?;
            pump(response, temp.as_file_mut())?;
            temp.as_file_mut()
                .sync_all()
                .map_err(|e| CliError::Other(anyhow::anyhow!("Could not flush audio: {e}")))?;
            let path = temp.path().to_string_lossy().to_string();
            let status = spawn_player(player, &path, Stdio::null())?
                .wait()
                .map_err(|e| {
                    CliError::Other(anyhow::anyhow!(
                        "Could not wait for {}: {e}",
                        player.program
                    ))
                })?;
            if !status.success() {
                return Err(CliError::Other(anyhow::anyhow!(
                    "{} exited with {}",
                    player.program,
                    status
                )));
            }
            Ok(())
        }
        (None, None) => unreachable!("a player is resolved whenever --output is absent"),
    }
}

// ── `say config` ────────────────────────────────────────────────────

#[derive(clap::Args)]
struct ConfigArgs {
    /// Setting to read or change: voice, model, output-format or player.
    /// Omit to show everything.
    key: Option<String>,

    /// New value. Omit to show the current one.
    value: Option<String>,

    /// Clear the setting and fall back to the default.
    #[arg(long, conflicts_with = "value")]
    unset: bool,
}

fn validate_value(spec: &SettingSpec, value: &str) -> Result<(), CliError> {
    // Everything here is echoed back to a terminal by `say config`, and a
    // player name is executed, so refuse control characters outright.
    if value.chars().any(char::is_control) {
        return Err(CliError::Validation(format!(
            "Invalid {}: control characters are not allowed.",
            spec.cli_key
        )));
    }
    if value.trim().is_empty() {
        return Err(CliError::Validation(format!(
            "Invalid {}: the value is empty. Use --unset to clear it.",
            spec.cli_key
        )));
    }
    match spec.cli_key {
        "output-format" => {
            if codec_of(value.trim()).is_none() {
                return Err(CliError::Validation(format!(
                    "Invalid output format '{value}'. Expected something like \
                     mp3_44100_128, wav_44100 or pcm_24000."
                )));
            }
        }
        _ => {
            if value.trim().split_whitespace().count() > 1 {
                return Err(CliError::Validation(format!(
                    "Invalid {}: '{value}' contains whitespace.",
                    spec.cli_key
                )));
            }
        }
    }
    Ok(())
}

/// How a value is being sourced, for the `say config` listing.
fn describe(spec: &SettingSpec) -> String {
    match (read_configured(spec.json_key), spec.default) {
        (Some(v), _) => v,
        (None, Some(d)) => format!("{d} (default)"),
        (None, None) if spec.cli_key == "output-format" => {
            format!("{MP3_FORMAT} (default, follows the player)")
        }
        (None, None) => match resolve_player(None, None) {
            Ok(p) => format!("{} (auto-detected)", p.program),
            Err(_) => "none found on PATH".to_string(),
        },
    }
}

fn handle_config(args: ConfigArgs, _ctx: &AppContext) -> Result<(), CliError> {
    let Some(key) = args.key.as_deref() else {
        for spec in SETTINGS {
            println!("{:<7} {}", format!("{}:", spec.label), describe(spec));
        }
        println!(
            "\nSet one with 'elevenlabs say config <{}> <value>'.",
            SETTINGS
                .iter()
                .map(|s| s.cli_key)
                .collect::<Vec<_>>()
                .join("|")
        );
        return Ok(());
    };

    let spec = spec_for(key).ok_or_else(|| {
        CliError::Validation(format!(
            "Unknown setting '{key}'. Available: {}.",
            SETTINGS
                .iter()
                .map(|s| s.cli_key)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    })?;

    if args.unset {
        settings::write_setting(&[SECTION, spec.json_key], None)?;
        println!("{} cleared. Now: {}", spec.label, describe(spec));
        return Ok(());
    }

    let Some(value) = args.value.as_deref() else {
        println!("{:<7} {}", format!("{}:", spec.label), describe(spec));
        return Ok(());
    };

    validate_value(spec, value)?;
    let value = value.trim();
    settings::write_setting(&[SECTION, spec.json_key], Some(value))?;
    println!("{} set to: {value}", spec.label);
    Ok(())
}

// ── Registration ────────────────────────────────────────────────────

/// The `say` command, with `config` as a subcommand of it.
///
/// Built as one `clap::Command` rather than two registrations because
/// `custom_commands::graft_subcommand` is custom-wins on leaf collision:
/// registering `say` at the root would *replace* the parent that grafting
/// `config` under `["say"]` had just created, silently dropping the
/// subcommand. Registering the pair as a single command also removes the
/// dispatch hazard — `walk_matches_to_custom(matches, &[], "say")` matches
/// every `say …` invocation, `say config` included.
fn say_command() -> clap::Command {
    let config = ConfigArgs::augment_args(
        clap::Command::new("config")
            .about("Show or set the default voice, model, format and player for 'say'"),
    );
    SayArgs::augment_args(
        clap::Command::new("say")
            .about("Speak text out loud using ElevenLabs text-to-speech")
            .long_about(
                "Convert text to speech and play it immediately.\n\n\
                 The voice, model, audio format and player default to whatever \
                 'elevenlabs say config' has stored, and can be overridden per run.\n\n\
                 Because 'config' is a subcommand, speaking that exact word needs \
                 'elevenlabs say -- config'.",
            ),
    )
    .subcommand(config)
}

fn dispatch(matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let to_validation = |e: clap::Error| CliError::Validation(e.to_string());
    match matches.subcommand() {
        Some(("config", sub)) => handle_config(
            ConfigArgs::from_arg_matches(sub).map_err(to_validation)?,
            ctx,
        ),
        _ => handle_say(
            SayArgs::from_arg_matches(matches).map_err(to_validation)?,
            ctx,
        ),
    }
}

/// Register `say` (and `say config`).
pub fn register(app: CliApp) -> CliApp {
    app.command(
        say_command(),
        Box::new(|matches, ctx| dispatch(matches, super::util::downcast_ctx(ctx)?)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── resolve ─────────────────────────────────────────────────────

    #[test]
    fn a_flag_beats_the_config_which_beats_the_default() {
        assert_eq!(resolve(Some("flag"), Some("config"), "default"), "flag");
        assert_eq!(resolve(None, Some("config"), "default"), "config");
        assert_eq!(resolve(None, None, "default"), "default");
    }

    #[test]
    fn a_whitespace_only_value_counts_as_unset() {
        // An empty voice ID would build `/v1/text-to-speech//stream`.
        assert_eq!(resolve(Some("   "), Some("config"), "default"), "config");
        assert_eq!(resolve(Some("   "), Some(""), "default"), "default");
    }

    // ── formats ─────────────────────────────────────────────────────

    #[test]
    fn known_formats_parse_to_their_codec() {
        assert_eq!(codec_of("mp3_44100_128"), Some("mp3"));
        assert_eq!(codec_of("wav_44100"), Some("wav"));
        assert_eq!(codec_of("pcm_24000"), Some("pcm"));
        assert_eq!(codec_of("opus_48000_128"), Some("opus"));
        assert_eq!(codec_of("ulaw_8000"), Some("ulaw"));
        assert_eq!(codec_of("alaw_8000"), Some("alaw"));
    }

    #[test]
    fn a_format_the_api_adds_later_is_still_accepted() {
        // Deliberately not pinned to the generated enum: a new bitrate must
        // not require a regeneration before it can be used.
        assert_eq!(codec_of("mp3_48000_256"), Some("mp3"));
    }

    #[test]
    fn typos_and_junk_are_rejected() {
        for bad in [
            "mp3",
            "mp3_",
            "_44100",
            "mpe3_44100_128",
            "wav_abc",
            "",
            "mp3_44100_",
            "; rm -rf /",
        ] {
            assert!(codec_of(bad).is_none(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn an_explicit_format_wins_over_everything() {
        assert_eq!(
            output_format_for(
                Some("opus_48000_64"),
                Some("wav_44100"),
                Some(Path::new("x.mp3")),
                Container::WavOnly
            ),
            "opus_48000_64"
        );
    }

    #[test]
    fn an_output_filename_beats_the_stored_default() {
        // Writing out.wav and getting MP3 bytes inside it is worse than
        // ignoring the configured format.
        assert_eq!(
            output_format_for(
                None,
                Some("mp3_44100_128"),
                Some(Path::new("out.wav")),
                Container::Any
            ),
            WAV_FORMAT
        );
    }

    #[test]
    fn the_stored_default_applies_when_nothing_else_decides() {
        assert_eq!(
            output_format_for(None, Some("opus_48000_96"), None, Container::Any),
            "opus_48000_96"
        );
    }

    #[test]
    fn a_wav_only_player_gets_wav() {
        assert_eq!(
            output_format_for(None, None, None, Container::WavOnly),
            WAV_FORMAT
        );
        assert_eq!(
            output_format_for(None, None, None, Container::Any),
            MP3_FORMAT
        );
    }

    #[test]
    fn an_unrecognized_extension_does_not_decide_the_format() {
        assert_eq!(
            output_format_for(None, None, Some(Path::new("out.bin")), Container::Any),
            MP3_FORMAT
        );
    }

    #[test]
    fn extensions_match_the_codec_so_players_pick_a_decoder() {
        assert_eq!(extension_for("mp3_44100_128"), "mp3");
        assert_eq!(extension_for("wav_44100"), "wav");
        assert_eq!(extension_for("opus_48000_128"), "opus");
        assert_eq!(extension_for("pcm_24000"), "raw");
    }

    // ── players ─────────────────────────────────────────────────────

    #[test]
    fn argv_never_goes_through_a_shell() {
        let ffplay = Player::from_spec(spec_for_program("ffplay").unwrap());
        assert_eq!(
            player_argv(&ffplay, "-"),
            vec!["-nodisp", "-autoexit", "-loglevel", "error", "-i", "-"]
        );

        let afplay = Player::from_spec(spec_for_program("afplay").unwrap());
        assert_eq!(player_argv(&afplay, "/tmp/a b.mp3"), vec!["/tmp/a b.mp3"]);
    }

    #[test]
    fn powershell_gets_a_single_quoted_script_with_quotes_doubled() {
        let ps = Player::from_spec(spec_for_program("powershell").unwrap());
        let argv = player_argv(&ps, "C:\\tmp\\it's.wav");
        assert_eq!(argv[0], "-NoProfile");
        assert_eq!(argv[1], "-Command");
        assert_eq!(
            argv[2],
            "(New-Object Media.SoundPlayer 'C:\\tmp\\it''s.wav').PlaySync()"
        );
    }

    #[test]
    fn an_unknown_player_is_driven_conservatively() {
        let p = Player::unknown("my-wrapper");
        assert!(
            !p.reads_stdin,
            "cannot assume an unknown player reads stdin"
        );
        assert_eq!(player_argv(&p, "/tmp/a.mp3"), vec!["/tmp/a.mp3"]);
    }

    #[test]
    fn stdin_players_are_preferred_so_streaming_pays_off() {
        let first_stdin = PLAYERS.iter().position(|p| p.reads_stdin).unwrap();
        let first_file = PLAYERS.iter().position(|p| !p.reads_stdin).unwrap();
        assert!(first_stdin < first_file);
    }

    #[test]
    fn a_missing_player_is_never_silently_accepted() {
        let err = resolve_player(Some("definitely-not-a-real-player-xyz"), None).unwrap_err();
        assert!(matches!(err, CliError::Validation(_)));
    }

    #[test]
    fn nothing_on_an_empty_path_resolves() {
        assert!(!on_path("ffplay-that-does-not-exist"));
        assert!(!on_path(""));
    }

    // ── text input ──────────────────────────────────────────────────

    #[test]
    fn positional_words_are_joined() {
        let args = ["this", "came", "from", "the", "terminal"].map(String::from);
        assert_eq!(
            text_source(&args, true).unwrap(),
            TextSource::Literal("this came from the terminal".to_string())
        );
    }

    #[test]
    fn a_lone_dash_reads_stdin() {
        assert_eq!(
            text_source(&[String::from("-")], true).unwrap(),
            TextSource::Stdin
        );
    }

    #[test]
    fn piped_input_needs_no_argument() {
        assert_eq!(text_source(&[], false).unwrap(), TextSource::Stdin);
    }

    #[test]
    fn an_interactive_terminal_with_no_text_is_an_error() {
        assert!(matches!(
            text_source(&[], true),
            Err(CliError::Validation(_))
        ));
    }

    #[test]
    fn a_dash_among_other_words_is_just_text() {
        let args = ["wait", "-", "then", "go"].map(String::from);
        assert!(matches!(
            text_source(&args, true).unwrap(),
            TextSource::Literal(_)
        ));
    }

    // ── settings ────────────────────────────────────────────────────

    #[test]
    fn every_setting_is_addressable_by_the_key_users_type() {
        for spec in SETTINGS {
            assert!(spec_for(spec.cli_key).is_some());
        }
        assert!(spec_for("nonsense").is_none());
    }

    #[test]
    fn values_with_control_characters_are_rejected() {
        // These get echoed back to a terminal, and a player name is executed.
        let voice = spec_for("voice").unwrap();
        assert!(validate_value(voice, "ok\u{1b}[31m").is_err());
        assert!(validate_value(voice, "ok\nnot ok").is_err());
        assert!(validate_value(voice, "21m00Tcm4TlvDq8ikWAM").is_ok());
    }

    #[test]
    fn an_id_with_whitespace_is_rejected() {
        assert!(validate_value(spec_for("model").unwrap(), "eleven flash").is_err());
        assert!(validate_value(spec_for("voice").unwrap(), "").is_err());
    }

    #[test]
    fn a_bad_format_is_rejected_at_set_time() {
        let fmt = spec_for("output-format").unwrap();
        assert!(validate_value(fmt, "mp3").is_err());
        assert!(validate_value(fmt, "mp3_44100_128").is_ok());
    }

    // ── command wiring ──────────────────────────────────────────────

    /// `config` is a subcommand of `say`, so it always wins over a
    /// positional of the same name; `--` is the documented escape hatch.
    #[test]
    fn saying_the_word_config_requires_a_double_dash() {
        let cmd = SayArgs::augment_args(
            clap::Command::new("say").subcommand(clap::Command::new("config")),
        );

        let matches = cmd
            .clone()
            .try_get_matches_from(["say", "config"])
            .expect("parses");
        assert_eq!(matches.subcommand_name(), Some("config"));

        let matches = cmd
            .try_get_matches_from(["say", "--", "config"])
            .expect("parses");
        assert_eq!(matches.subcommand_name(), None);
        assert_eq!(
            matches
                .get_many::<String>("text")
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["config"]
        );
    }

    #[test]
    fn flags_can_follow_the_text() {
        let cmd = SayArgs::augment_args(clap::Command::new("say"));
        let matches = cmd
            .try_get_matches_from(["say", "hello", "there", "--voice", "abc"])
            .expect("parses");
        assert_eq!(matches.get_one::<String>("voice").unwrap(), "abc");
        assert_eq!(
            matches
                .get_many::<String>("text")
                .unwrap()
                .cloned()
                .collect::<Vec<_>>(),
            vec!["hello", "there"]
        );
    }
}
