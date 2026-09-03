//! Optional agent-supplied intent, carried on every request as
//! `X-Agent-Intent`.
//!
//! The CLI is driven mostly by AI agents. We already know *which* command
//! ran (`cmd/agents.push` in the User-Agent — see [`super::api::command_scope`])
//! but never *why*. The hosted MCP server answers that by injecting a
//! `context` argument into every advertised tool schema; this is the CLI's
//! equivalent, minus the ability to make it required.
//!
//! ## Why the flag never reaches the wire
//!
//! The framework injects a [`GlobalParameter`]'s value **verbatim** at the
//! configured wire location — there is no validation hook anywhere between
//! clap and the request. So `--intent` is registered with
//! [`GlobalParameterApplyMode::Explicit`] and a target no operation opts
//! into, which means the framework accepts the flag but never sends it. A
//! second, hidden, env-only parameter carries the value that *is* sent, and
//! the only writer of that env var is [`resolve`] — which runs before clap
//! parses anything and refuses to write a value that looks like personal
//! data.
//!
//! Dropping is deliberately silent-ish: a rejected intent warns on stderr
//! and the command proceeds normally. Telemetry must never be able to fail
//! a user's request. The `feedback` command is the one exception (see
//! [`super::feedback`]) — there the text *is* the payload, so a rejection
//! is an error.

use std::sync::OnceLock;

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::openapi::discovery::{
    GlobalParameter, GlobalParameterApplyMode, GlobalParameterLocation,
};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// Header carrying the sanitised, percent-encoded intent.
pub const INTENT_HEADER: &str = "X-Agent-Intent";

/// Env var an agent sets to describe its goal once for a whole task.
const INTENT_ENV: &str = "ELEVENLABS_AGENT_INTENT";

/// Internal env var holding the sanitised, encoded value. Written only by
/// [`resolve`]; read by the framework through the hidden global parameter.
const CHECKED_ENV: &str = "ELEVENLABS_AGENT_INTENT_CHECKED";

/// Matches `xi_mcp`'s `_INTENT_MAX_LENGTH`, so the CLI and the MCP agree on
/// what "briefly" means and the backend's cap never has to truncate ours.
const MAX_CHARS: usize = 500;

/// Backstop on the encoded form. 500 characters of ASCII encode to ~500
/// bytes; 500 characters of CJK to ~4500. Beyond this we are into territory
/// where intermediate proxies start rejecting header sizes, and a request
/// that fails because of telemetry is the one outcome this module must not
/// produce.
const MAX_ENCODED_BYTES: usize = 4096;

/// Escape controls plus `%` itself, so the encoding round-trips. Non-ASCII
/// bytes are not expressible in an `AsciiSet` at all — `utf8_percent_encode`
/// always escapes them, which is what carries the non-English intents
/// through: header values are ASCII-only in practice, and a large slice of
/// what the MCP collects is not English.
const HEADER_ESCAPE: &AsciiSet = &CONTROLS.add(b'%');

/// The resolved intent for this process: `None` when absent or dropped.
/// `(plain, encoded)` — the JSON-bodied `feedback` command wants the former,
/// the header path the latter.
static RESOLVED: OnceLock<Option<(String, String)>> = OnceLock::new();

/// Why a value was refused. Each variant produces its own warning so the
/// agent learns what to change rather than just that "something" was wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropReason {
    Empty,
    TooLong,
    ControlChars,
    Email,
    Phone,
    Secret,
    Path,
    UrlCredentials,
}

impl DropReason {
    /// Agent-facing explanation. Phrased as an instruction, not a complaint —
    /// the reader is a model deciding what to send next time.
    pub fn advice(self) -> &'static str {
        match self {
            DropReason::Empty => "it is empty",
            DropReason::TooLong => {
                "it is longer than 500 characters — describe the goal in one sentence"
            }
            DropReason::ControlChars => "it contains line breaks or control characters",
            DropReason::Email => {
                "it looks like it contains an email address — describe the goal, not the data"
            }
            DropReason::Phone => {
                "it looks like it contains a phone number — describe the goal, not the data"
            }
            DropReason::Secret => {
                "it looks like it contains an API key or token — never include credentials"
            }
            DropReason::Path => {
                "it looks like it contains an absolute file path — describe the goal, not the data"
            }
            DropReason::UrlCredentials => {
                "it looks like it contains a URL with embedded credentials"
            }
        }
    }
}

impl std::fmt::Display for DropReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.advice())
    }
}

// ── Sanitising ──────────────────────────────────────────────────────

/// Validate agent-supplied free text, returning the trimmed value.
///
/// Pure: no env, no I/O. The same checks run again server-side — a client
/// -side filter is a nudge, not a boundary — but doing it here means the
/// data never leaves the machine and the agent gets told why.
pub fn sanitize(raw: &str) -> Result<String, DropReason> {
    let text = raw.trim();
    if text.is_empty() {
        return Err(DropReason::Empty);
    }
    if text.chars().count() > MAX_CHARS {
        return Err(DropReason::TooLong);
    }
    if text.chars().any(char::is_control) {
        return Err(DropReason::ControlChars);
    }
    if contains_email(text) {
        return Err(DropReason::Email);
    }
    if contains_url_credentials(text) {
        return Err(DropReason::UrlCredentials);
    }
    if contains_secret(text) {
        return Err(DropReason::Secret);
    }
    if contains_absolute_path(text) {
        return Err(DropReason::Path);
    }
    if contains_phone(text) {
        return Err(DropReason::Phone);
    }
    Ok(text.to_string())
}

/// Percent-encode a sanitised value for use as a header. Separate from
/// [`sanitize`] because `feedback` sends its text in a JSON body, where
/// encoding would be wrong.
pub fn encode_header(clean: &str) -> Result<String, DropReason> {
    let encoded = utf8_percent_encode(clean, HEADER_ESCAPE).to_string();
    if encoded.len() > MAX_ENCODED_BYTES {
        return Err(DropReason::TooLong);
    }
    Ok(encoded)
}

/// `local@domain.tld` inside any whitespace-delimited token.
fn contains_email(text: &str) -> bool {
    text.split_whitespace().any(|token| {
        // Strip punctuation an agent would naturally write around it.
        let token = token.trim_matches(|c: char| matches!(c, '(' | ')' | '<' | '>' | ',' | ';' | '"' | '\''));
        let Some((local, domain)) = token.split_once('@') else {
            return false;
        };
        if local.is_empty() || !local.chars().any(|c| c.is_ascii_alphanumeric()) {
            return false;
        }
        let domain = domain.trim_end_matches('.');
        let Some((host, tld)) = domain.rsplit_once('.') else {
            return false;
        };
        !host.is_empty() && tld.len() >= 2 && tld.chars().all(|c| c.is_ascii_alphabetic())
    })
}

/// `scheme://user:pass@host`.
fn contains_url_credentials(text: &str) -> bool {
    let mut rest = text;
    while let Some(idx) = rest.find("://") {
        let after = &rest[idx + 3..];
        let authority_end = after
            .find(|c: char| matches!(c, '/' | '?' | '#') || c.is_whitespace())
            .unwrap_or(after.len());
        let authority = &after[..authority_end];
        if let Some((userinfo, _)) = authority.split_once('@') {
            if userinfo.contains(':') && !userinfo.is_empty() {
                return true;
            }
        }
        rest = &after[authority_end..];
    }
    false
}

/// Credential markers, plus a length backstop for anything key-shaped that
/// does not carry a known prefix. The threshold is 40 so it clears
/// ElevenLabs resource ids (`agent_01jz…`, ~24-32 chars), which are useful
/// context and not secrets.
fn contains_secret(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    const MARKERS: &[&str] = &[
        "sk_",
        "xi-api-key",
        "xi_api_key",
        "bearer ",
        "api_key=",
        "apikey=",
        "access_token",
        "password",
        "secret_",
        "-----begin",
    ];
    if MARKERS.iter().any(|m| lower.contains(m)) {
        return true;
    }
    text.split(|c: char| !is_token_char(c)).any(|run| {
        run.len() >= 40
            && run.chars().any(|c| c.is_ascii_digit())
            && run.chars().any(|c| c.is_ascii_alphabetic())
    })
}

fn is_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '_' | '-')
}

/// Absolute filesystem paths. Relative ones (`agent_configs/x.json`) are
/// left alone — they are project structure, not personal data.
fn contains_absolute_path(text: &str) -> bool {
    const UNIX_PREFIXES: &[&str] = &["/Users/", "/home/", "/root/", "/var/folders/"];
    if UNIX_PREFIXES.iter().any(|p| text.contains(p)) {
        return true;
    }
    // Windows drive paths: a *standalone* letter, a colon, then a separator.
    // The "standalone" part is load-bearing — without it every `https://`
    // matches, since `s:/` has the same shape.
    let bytes = text.as_bytes();
    (1..bytes.len().saturating_sub(1)).any(|i| {
        bytes[i] == b':'
            && bytes[i - 1].is_ascii_alphabetic()
            && (bytes[i + 1] == b'\\' || bytes[i + 1] == b'/')
            // Nothing alphanumeric before the drive letter. `i < 2` is the
            // case where the letter starts the string.
            && (i < 2 || !bytes[i - 2].is_ascii_alphanumeric())
    })
}

/// Phone-shaped digit runs. Three narrow rules rather than one broad one,
/// because the false positives worth avoiding are dates and version
/// numbers, which agents write constantly.
fn contains_phone(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();

    // (a) E.164: `+` then at least 10 digits, ignoring spaces/dashes/parens.
    for (i, c) in chars.iter().enumerate() {
        if *c != '+' {
            continue;
        }
        let mut digits = 0usize;
        for c in &chars[i + 1..] {
            if c.is_ascii_digit() {
                digits += 1;
            } else if matches!(c, ' ' | '-' | '(' | ')' | '.') {
                continue;
            } else {
                break;
            }
        }
        if digits >= 10 {
            return true;
        }
    }

    // (b) A bare run of 9+ digits. Long enough that dates and ports do not
    //     reach it; anything that does is an identifier we would rather not
    //     collect.
    let mut run = 0usize;
    for c in &chars {
        if c.is_ascii_digit() {
            run += 1;
            if run >= 9 {
                return true;
            }
        } else {
            run = 0;
        }
    }

    // (c) Separated shapes like `555-010-9999` or `(555) 010 9999`: 10+
    //     digits joined only by phone separators. `:` and `/` break the run,
    //     which is what keeps `2026-08-25 14:30` and `25/08/2026` out.
    let mut digits = 0usize;
    for c in &chars {
        if c.is_ascii_digit() {
            digits += 1;
            if digits >= 10 {
                return true;
            }
        } else if matches!(c, ' ' | '-' | '(' | ')' | '.') {
            continue;
        } else {
            digits = 0;
        }
    }

    false
}

// ── Resolution ──────────────────────────────────────────────────────

/// Pull the raw intent out of argv, mirroring clap's last-wins semantics
/// for a `Set` argument. Stops at `--`, after which nothing is a flag.
///
/// Pre-parse scanning is unavoidable: the generated `<resource> <method>`
/// commands never call into this crate, so there is no post-parse hook that
/// runs for them.
fn scan_argv<I: IntoIterator<Item = String>>(args: I) -> Option<String> {
    let mut found = None;
    let mut iter = args.into_iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--" {
            break;
        }
        if arg == "--intent" {
            if let Some(value) = iter.next() {
                found = Some(value);
            }
        } else if let Some(value) = arg.strip_prefix("--intent=") {
            found = Some(value.to_string());
        }
    }
    found
}

/// Sanitise and encode in one step. Split out so the env plumbing below
/// stays trivial and the decision itself is testable on its own.
fn vet(raw: &str) -> Result<(String, String), DropReason> {
    let clean = sanitize(raw)?;
    let encoded = encode_header(&clean)?;
    Ok((clean, encoded))
}

/// Env plumbing for one raw input. Returns what should be published.
///
/// Always clears `CHECKED_ENV` first: it is internal, so any value already
/// in the environment came from a parent process rather than from us, and
/// honouring it would let a caller put an arbitrary unsanitised header on
/// the wire.
fn resolve_with(raw: Option<String>) -> Option<(String, String)> {
    std::env::remove_var(CHECKED_ENV);
    let raw = raw?;
    match vet(&raw) {
        Ok((clean, encoded)) => {
            std::env::set_var(CHECKED_ENV, &encoded);
            Some((clean, encoded))
        }
        Err(reason) => {
            eprintln!("warning: --intent dropped because {reason}.");
            None
        }
    }
}

/// Resolve, validate, and publish the intent for this process.
///
/// Runs from [`super::register`], i.e. before `CliApp::run` parses argv and
/// before clap reads `.env()` bindings, which is what lets the sanitised
/// value reach the framework at all.
pub fn resolve() {
    let raw = scan_argv(std::env::args()).or_else(|| std::env::var(INTENT_ENV).ok());
    let _ = RESOLVED.set(resolve_with(raw));
}

/// The sanitised intent as plain text, for JSON payloads.
pub fn resolved_text() -> Option<String> {
    RESOLVED.get()?.as_ref().map(|(plain, _)| plain.clone())
}

/// The sanitised intent percent-encoded, for the header on the hand-written
/// command path (`super::api::request_options`). The generated command path
/// gets it through the framework instead.
pub fn resolved_encoded() -> Option<String> {
    RESOLVED.get()?.as_ref().map(|(_, encoded)| encoded.clone())
}

// ── Registration ────────────────────────────────────────────────────

const INTENT_HELP: &str = "Optional. Why are you running this command? Briefly describe the \
     user's goal in one sentence (max 500 characters). Never include names, email addresses, \
     phone numbers, API keys, file paths, or any other personal or customer data — describe \
     the goal, not the data. Values that look like personal data are dropped with a warning.";

/// Register the intent parameters and resolve the value.
///
/// Two parameters, deliberately: see the module docs for why the flag the
/// agent types is never the one that goes on the wire.
pub fn register(app: CliApp) -> CliApp {
    resolve();
    app.global_parameter(GlobalParameter {
        name: "intent".into(),
        location: GlobalParameterLocation::Header,
        // Never sent — `Explicit` with no opt-ins means the framework skips
        // it. A target distinct from the real header keeps it that way even
        // if the apply-mode rules were ever loosened upstream.
        target: "X-Agent-Intent-Unvalidated".into(),
        env: Some(INTENT_ENV.into()),
        default: None,
        optional: true,
        apply: GlobalParameterApplyMode::Explicit,
        parameter_name: None,
        docs: Some(INTENT_HELP.into()),
    })
    .global_parameter(GlobalParameter {
        name: "intent-checked".into(),
        location: GlobalParameterLocation::Header,
        target: INTENT_HEADER.into(),
        env: Some(CHECKED_ENV.into()),
        default: None,
        optional: true,
        apply: GlobalParameterApplyMode::Auto,
        parameter_name: None,
        docs: Some("Internal: the sanitised value of --intent. Set --intent instead.".into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(args: &[&str]) -> Vec<String> {
        std::iter::once("elevenlabs")
            .chain(args.iter().copied())
            .map(String::from)
            .collect()
    }

    // ── sanitize: accepts ──

    #[test]
    fn accepts_an_ordinary_intent() {
        assert_eq!(
            sanitize("  pick a narrator voice for an audiobook  ").unwrap(),
            "pick a narrator voice for an audiobook"
        );
    }

    #[test]
    fn accepts_resource_ids_and_relative_paths() {
        // These are the most common real intents in the MCP data and are
        // not personal data — dropping them would gut the signal.
        sanitize("update agent_01jz9k4m2n8p7q6r5s4t3u2v1w to use eleven_turbo_v2").unwrap();
        sanitize("push agent_configs/support-bot.json after editing the prompt").unwrap();
        sanitize("poll flow run status for the sfx flow started at 14:30").unwrap();
        sanitize("bump TTS stability from 0.5 to 0.75").unwrap();
        sanitize("check what changed between 2026-08-25 and 2026-09-01").unwrap();
    }

    #[test]
    fn accepts_non_english_and_round_trips_it() {
        let clean =
            sanitize("mehrere Sprecher mit unterschiedlichen voice_ids in einem Durchlauf")
                .unwrap();
        let encoded = encode_header(&clean).unwrap();
        assert!(encoded.is_ascii());

        let accented = sanitize("gerar diálogo em português com duas vozes").unwrap();
        let encoded = encode_header(&accented).unwrap();
        assert!(encoded.is_ascii(), "header values must be ASCII");
        assert!(encoded.contains("%C3%A1"), "expected UTF-8 percent-encoding");
        let decoded = percent_encoding::percent_decode_str(&encoded)
            .decode_utf8()
            .unwrap();
        assert_eq!(decoded, accented);
    }

    #[test]
    fn a_percent_sign_survives_the_round_trip() {
        let clean = sanitize("raise stability by 25% for the narrator").unwrap();
        let encoded = encode_header(&clean).unwrap();
        assert!(encoded.contains("%25"));
        let decoded = percent_encoding::percent_decode_str(&encoded)
            .decode_utf8()
            .unwrap();
        assert_eq!(decoded, clean);
    }

    // ── sanitize: rejects ──

    #[test]
    fn rejects_empty_and_whitespace() {
        assert_eq!(sanitize(""), Err(DropReason::Empty));
        assert_eq!(sanitize("   \t "), Err(DropReason::Empty));
    }

    #[test]
    fn rejects_over_five_hundred_characters() {
        let long = "x".repeat(501);
        assert_eq!(sanitize(&long), Err(DropReason::TooLong));
        assert!(sanitize(&"x".repeat(500)).is_ok());
    }

    #[test]
    fn rejects_control_characters() {
        assert_eq!(sanitize("goal\nsecond line"), Err(DropReason::ControlChars));
        assert_eq!(sanitize("goal\r\nsecond line"), Err(DropReason::ControlChars));
        assert_eq!(sanitize("goal\tand more"), Err(DropReason::ControlChars));
        // A trailing newline is just sloppy quoting — trim it rather than
        // punish the agent for it.
        assert_eq!(sanitize("goal\r\n").unwrap(), "goal");
    }

    #[test]
    fn rejects_email_addresses() {
        assert_eq!(
            sanitize("send the render to jane.doe@example.com"),
            Err(DropReason::Email)
        );
        assert_eq!(sanitize("cc (bob@corp.co.uk)"), Err(DropReason::Email));
        // A bare @mention is not an address.
        assert!(sanitize("ask @support about the quota").is_ok());
    }

    #[test]
    fn rejects_phone_numbers() {
        assert_eq!(
            sanitize("assign +1 555 010 9999 to the outbound agent"),
            Err(DropReason::Phone)
        );
        assert_eq!(sanitize("dial 555-010-9999 next"), Err(DropReason::Phone));
        assert_eq!(sanitize("number 07700900123 please"), Err(DropReason::Phone));
    }

    #[test]
    fn rejects_secrets() {
        assert_eq!(
            sanitize("use sk_abc123 for the request"),
            Err(DropReason::Secret)
        );
        assert_eq!(
            sanitize("set the xi-api-key header"),
            Err(DropReason::Secret)
        );
        assert_eq!(
            sanitize("pass Bearer eyJhbGciOi to the endpoint"),
            Err(DropReason::Secret)
        );
        assert_eq!(
            sanitize(&format!("token {}", "a1b2c3d4e5".repeat(4))),
            Err(DropReason::Secret)
        );
    }

    #[test]
    fn rejects_absolute_paths() {
        assert_eq!(
            sanitize("read /Users/jane/projects/agent.json"),
            Err(DropReason::Path)
        );
        assert_eq!(sanitize("open C:\\Users\\jane\\a.json"), Err(DropReason::Path));
        // Relative project paths are structure, not personal data.
        assert!(sanitize("push agent_configs/support.json").is_ok());
    }

    #[test]
    fn rejects_urls_with_embedded_credentials() {
        assert_eq!(
            sanitize("proxy through https://user:pw@proxy.internal/v1"),
            Err(DropReason::UrlCredentials)
        );
        assert!(sanitize("fetch https://example.com/v1/voices").is_ok());
    }

    #[test]
    fn rejects_an_encoded_value_that_grows_past_the_header_budget() {
        // Each of these encodes to 9 bytes, so 500 of them clear the
        // character cap but blow the byte budget.
        let clean = "\u{10348}".repeat(500);
        assert_eq!(encode_header(&clean), Err(DropReason::TooLong));
    }

    // ── argv scanning ──

    #[test]
    fn scans_both_flag_spellings() {
        assert_eq!(
            scan_argv(argv(&["voices", "list", "--intent", "find a voice"])),
            Some("find a voice".to_string())
        );
        assert_eq!(
            scan_argv(argv(&["voices", "list", "--intent=find a voice"])),
            Some("find a voice".to_string())
        );
    }

    #[test]
    fn the_last_occurrence_wins_like_clap() {
        assert_eq!(
            scan_argv(argv(&["--intent", "first", "--intent", "second"])),
            Some("second".to_string())
        );
    }

    #[test]
    fn stops_at_the_double_dash_terminator() {
        assert_eq!(
            scan_argv(argv(&["voices", "list", "--", "--intent", "not a flag"])),
            None
        );
    }

    #[test]
    fn absent_flag_yields_nothing() {
        assert_eq!(scan_argv(argv(&["voices", "list"])), None);
        // A trailing `--intent` with no value is not a value.
        assert_eq!(scan_argv(argv(&["voices", "list", "--intent"])), None);
    }

    // ── env plumbing ──
    //
    // `resolve_with` mutates process-wide env, so these must not interleave.
    // Same shape as the `ENV_LOCK` harness in `api.rs`.

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_env<T>(body: impl FnOnce() -> T) -> T {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let saved = std::env::var(CHECKED_ENV).ok();
        let out = body();
        match saved {
            Some(v) => std::env::set_var(CHECKED_ENV, v),
            None => std::env::remove_var(CHECKED_ENV),
        }
        out
    }

    #[test]
    fn a_clean_value_is_published_and_exported() {
        with_env(|| {
            let resolved = resolve_with(Some("pick a narrator voice".to_string()));
            assert_eq!(
                resolved,
                Some((
                    "pick a narrator voice".to_string(),
                    // Spaces are legal in a header value and left readable.
                    "pick a narrator voice".to_string()
                ))
            );
            assert_eq!(
                std::env::var(CHECKED_ENV).as_deref(),
                Ok("pick a narrator voice")
            );
        });
    }

    #[test]
    fn a_rejected_value_exports_nothing() {
        with_env(|| {
            assert_eq!(resolve_with(Some("call +1 555 010 9999".to_string())), None);
            assert!(
                std::env::var(CHECKED_ENV).is_err(),
                "a dropped intent must not reach the wire"
            );
        });
    }

    #[test]
    fn an_inherited_checked_value_is_discarded() {
        with_env(|| {
            // Only `resolve_with` may write this var. A value already in the
            // environment came from a parent process, and trusting it would
            // put an unsanitised header on the wire.
            std::env::set_var(CHECKED_ENV, "smuggled%20value");
            assert_eq!(resolve_with(None), None);
            assert!(std::env::var(CHECKED_ENV).is_err());
        });
    }

    #[test]
    fn an_inherited_checked_value_loses_to_a_real_intent() {
        with_env(|| {
            std::env::set_var(CHECKED_ENV, "smuggled%20value");
            resolve_with(Some("list the workspace voices".to_string()));
            assert_eq!(
                std::env::var(CHECKED_ENV).as_deref(),
                Ok("list the workspace voices")
            );
        });
    }
}
