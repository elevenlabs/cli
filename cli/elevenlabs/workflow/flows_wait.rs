//! `flows <image|video|text-to-speech|templates runs> create-and-wait`: start a
//! Flows generation or template run and poll it until it is `completed` or
//! `failed`, in one command. Ports `createAndWait` from elevenlabs-js#469
//! (`src/wrapper/flows.ts`); elevenlabs-python#889 is the Python counterpart.
//!
//! ## Why a sibling verb and not `create --wait`
//!
//! A custom command registered at an existing path *replaces* the generated
//! leaf (`graft_subcommand`), so `create --wait` would also take over every
//! plain `create`. That handler cannot hand back to the generated dispatch,
//! and the code that turns the generated flags into a request
//! (`collect_params_from_flags`, `--dry-run`, `--output`, `--format raw|http`,
//! `--no-retry`) is `pub(crate)` inside the Fern-generated framework (`src/`,
//! which `.fernignore` does not protect). Hooks don't fit either:
//! `transform_response` only sees the decoded body, not the response
//! headers or the parsed flags, and the only way to add a flag outside a
//! custom command is a global parameter, which would show `--wait` on every
//! command. A sibling leaf leaves the generated `create` exactly as it is.
//!
//! ## Keeping the flags in sync with the spec
//!
//! The sibling's flags are cloned at startup from the generated `create`
//! command, built by the framework's own `build_cli`, so a field added to the
//! spec becomes a flag here too. The request is assembled at run time from the
//! full spec's parameter locations ([`build_request`]), not from hand-written
//! field lists.
//!
//! The commands are built from `flows_spec.json`, the `/v1/flows/` slice of
//! `openapi0.json` plus every component it references. A second
//! `include_str!` of the full spec would add 2.3MB to the binary (the compiler
//! does not merge the two copies) and about 90ms to every startup; the slice
//! is 130KB and parses and builds in about 5ms. A unit test fails when the slice
//! no longer matches `openapi0.json`; regenerate it with
//! `UPDATE_FLOWS_SPEC=1 cargo test --bin elevenlabs flows_spec`.
//!
//! ## Polling
//!
//! Create and every GET go through the SDK's `execute_request_raw`, which uses
//! the CLI's executor (auth, `--debug`, and retries on 429/5xx) and returns the
//! response headers. The wait before each GET is the server's `Retry-After`,
//! never less than `--min-poll-interval`, plus 0–20% jitter. The create
//! response's `Retry-After` sets the first wait.

use std::io::{IsTerminal, Write};
use std::time::{Duration, Instant, SystemTime};

use fern_cli_sdk::app::{CliApp, CliCommandHandler};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::discovery::{RestDescription, RestMethod, RestResource};
use fern_cli_sdk::openapi::AppContext;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use serde_json::{json, Map, Value};

/// Matches the server's shortest `Retry-After` (3s for image, audio and text),
/// so a missing header never makes the CLI poll faster than the server would.
const DEFAULT_MIN_POLL_INTERVAL_SECS: f64 = 3.0;
/// Below this, a missing server hint would turn the wait into a tight loop.
const SMALLEST_MIN_POLL_INTERVAL_SECS: f64 = 0.5;
/// Positive jitter, matching the JS SDK's fetcher.
const JITTER_FACTOR: f64 = 0.2;

/// Exit code for a run or generation that finished as `failed`. The result is
/// still printed to stdout; the code is the API's (`elevenlabs errors`), since
/// the API is what reported the failure.
const EXIT_FAILED: i32 = CliError::EXIT_CODE_API;

/// Test-only: when set, sleeps are not performed. A virtual clock advances by
/// each requested sleep instead, and every sleep is logged to stderr as
/// `flows-wait: sleep <ms>ms`, so the wire tests can check pacing and timeouts
/// without waiting.
const FAKE_CLOCK_ENV: &str = "ELEVENLABS_FLOWS_WAIT_FAKE_CLOCK";

/// Generated flags that make no sense on a single-result polling command, or
/// that this path cannot honour.
const SKIPPED_ARGS: &[&str] = &[
    "page-all",
    "page-limit",
    "page-delay",
    "no-pager",
    "no-extract",
    // Retries come from the shared executor, which has no per-call switch.
    "no-retry",
];

/// One `create-and-wait` command: where it lives, and how to describe what it
/// waits on.
struct Target {
    /// Command path of the generated group, which is also the spec resource path.
    path: &'static [&'static str],
    /// Static User-Agent tag (see [`super::api::command_scope`]).
    scope: &'static str,
    /// What `create` makes, for help text.
    noun: &'static str,
}

static TARGETS: &[Target] = &[
    Target {
        path: &["flows", "image"],
        scope: "flows.image.create-and-wait",
        noun: "an image generation",
    },
    Target {
        path: &["flows", "video"],
        scope: "flows.video.create-and-wait",
        noun: "a video generation",
    },
    Target {
        path: &["flows", "text-to-speech"],
        scope: "flows.text-to-speech.create-and-wait",
        noun: "a text-to-speech generation",
    },
    Target {
        path: &["flows", "templates", "runs"],
        scope: "flows.templates.runs.create-and-wait",
        noun: "a template run",
    },
];

// ── Registration ────────────────────────────────────────────────────

/// The `/v1/flows/` slice of `openapi0.json`; see the module docs.
const FLOWS_SPEC: &str = include_str!("flows_spec.json");

pub fn register(app: CliApp) -> CliApp {
    let doc = match fern_cli_sdk::openapi::load_openapi_spec(FLOWS_SPEC, "elevenlabs") {
        Ok(doc) => doc,
        // Covered by `flows_spec_matches_openapi0`; never fail startup over it.
        Err(_) => return app,
    };
    let tree = fern_cli_sdk::openapi::commands::build_cli(&doc);
    TARGETS.iter().fold(app, |app, target| {
        let Some(create) =
            find_command(&tree, target.path).and_then(|g| g.find_subcommand("create"))
        else {
            return app;
        };
        let handler: CliCommandHandler =
            Box::new(move |matches, ctx| run(target, matches, super::util::downcast_ctx(ctx)?));
        app.command_under(target.path, sibling_command(target, create), handler)
    })
}

fn find_command<'a>(root: &'a clap::Command, path: &[&str]) -> Option<&'a clap::Command> {
    path.iter()
        .try_fold(root, |cmd, name| cmd.find_subcommand(name))
}

fn sibling_command(target: &Target, create: &clap::Command) -> clap::Command {
    let get_cmd = format!("elevenlabs {} get", target.path.join(" "));
    let mut cmd = clap::Command::new("create-and-wait")
        .about(format!(
            "Start {} and wait until it is completed or failed, then print it",
            target.noun
        ))
        .long_about(format!(
            "Start {noun} and wait until its `status` is `completed` or `failed`, then print the \
             final result (the same JSON as `{get_cmd}`).\n\n\
             Takes the same flags as `create`. Between status checks it waits for the server's \
             `Retry-After`, never less than --min-poll-interval. Progress goes to stderr when \
             stderr is a terminal or with --debug; stdout only carries the final result.\n\n\
             A `failed` result is printed like any other, and the command exits with code \
             {EXIT_FAILED}. If --wait-timeout runs out first, it exits with the same code and an \
             error with `reason: waitTimeout`, the last response under `details`, and the `get` \
             command that resumes waiting; the work keeps running on the server.\n\n\
             Meant for scripts and quickstarts. In production, set `webhook` on `create` \
             instead of holding a process open.",
            noun = target.noun,
        ));
    for arg in create.get_arguments() {
        if !SKIPPED_ARGS.contains(&arg.get_id().as_str()) {
            cmd = cmd.arg(arg.clone());
        }
    }
    for group in create.get_groups() {
        cmd = cmd.group(group.clone());
    }
    cmd.arg(
        clap::Arg::new("wait-timeout")
            .long("wait-timeout")
            .value_name("SECONDS")
            .value_parser(clap::value_parser!(f64))
            .help(
                "Give up waiting after this many seconds (the work keeps running on the server). \
                 Waits indefinitely when omitted. Not a per-request timeout.",
            ),
    )
    .arg(
        clap::Arg::new("min-poll-interval")
            .long("min-poll-interval")
            .value_name("SECONDS")
            .value_parser(clap::value_parser!(f64))
            .help(
                "Shortest wait between status checks, also used when the server sends no \
                 Retry-After. Default 3; at least 0.5.",
            ),
    )
}

// ── Options ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
struct WaitOptions {
    /// `None` waits indefinitely.
    timeout: Option<Duration>,
    min_interval: Duration,
}

impl WaitOptions {
    /// Validated before any request is sent.
    fn new(timeout_secs: Option<f64>, min_interval_secs: Option<f64>) -> Result<Self, CliError> {
        let min = min_interval_secs.unwrap_or(DEFAULT_MIN_POLL_INTERVAL_SECS);
        // `is_finite` also rejects NaN, which would collapse every sleep.
        if !min.is_finite() || min < SMALLEST_MIN_POLL_INTERVAL_SECS {
            return Err(CliError::Validation(format!(
                "--min-poll-interval must be a finite number of seconds, at least \
                 {SMALLEST_MIN_POLL_INTERVAL_SECS}; got {min}"
            )));
        }
        let timeout = match timeout_secs {
            Some(t) if t.is_nan() || t < 0.0 => {
                return Err(CliError::Validation(format!(
                    "--wait-timeout must be at least 0 seconds; got {t}"
                )))
            }
            // `inf` means the same as leaving it out.
            Some(t) if t.is_infinite() => None,
            Some(t) => Some(Duration::from_secs_f64(t)),
            None => None,
        };
        Ok(Self {
            timeout,
            min_interval: Duration::from_secs_f64(min),
        })
    }

    fn from_matches(matches: &clap::ArgMatches) -> Result<Self, CliError> {
        Self::new(
            matches.get_one::<f64>("wait-timeout").copied(),
            matches.get_one::<f64>("min-poll-interval").copied(),
        )
    }
}

// ── Pacing ──────────────────────────────────────────────────────────

/// The wait before the next GET: the server's `Retry-After` (delta-seconds or
/// HTTP date), never below `min`, times `1 + jitter * 20%`. `jitter` is in
/// `[0, 1)`. A missing, unparseable or `0` header falls back to `min`.
fn poll_delay(retry_after: Option<&str>, min: Duration, now: SystemTime, jitter: f64) -> Duration {
    let server = retry_after
        .and_then(|v| fern_cli_sdk::http::parse_retry_after(v, now))
        .unwrap_or(min);
    server.max(min).mul_f64(1.0 + jitter * JITTER_FACTOR)
}

/// Elapsed time and sleeping, swappable so tests don't wait.
trait Clock {
    fn elapsed(&self) -> Duration;
    fn sleep(&mut self, d: Duration);
}

struct RealClock(Instant);

impl Clock for RealClock {
    fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
    fn sleep(&mut self, d: Duration) {
        // Handlers run on a tokio worker; let the runtime know it is parked.
        tokio::task::block_in_place(|| std::thread::sleep(d));
    }
}

/// Advances only when slept, and logs each sleep (see [`FAKE_CLOCK_ENV`]).
#[derive(Default)]
struct FakeClock {
    elapsed: Duration,
    log: bool,
}

impl Clock for FakeClock {
    fn elapsed(&self) -> Duration {
        self.elapsed
    }
    fn sleep(&mut self, d: Duration) {
        if self.log {
            eprintln!("flows-wait: sleep {}ms", d.as_millis());
        }
        self.elapsed += d;
    }
}

// ── The wait loop ───────────────────────────────────────────────────

/// One response: the decoded body, its HTTP status and its `Retry-After`.
#[derive(Debug, Clone)]
struct Polled {
    body: Value,
    status: u16,
    retry_after: Option<String>,
}

fn status_of(body: &Value) -> &str {
    body.get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
}

fn is_terminal(body: &Value) -> bool {
    matches!(status_of(body), "completed" | "failed")
}

#[derive(Debug)]
enum WaitError {
    /// Still unfinished at the deadline; carries the last response.
    TimedOut(Polled),
    Cli(CliError),
}

/// Poll with `fetch` until the body is terminal. `first` is the create
/// response; `started` is when the create was sent, on `clock`'s timeline.
///
/// With a timeout, the last sleep is cut short at the deadline and one final
/// GET runs before giving up, so a result that lands right at the deadline is
/// still returned.
fn wait_until_terminal(
    first: Polled,
    mut fetch: impl FnMut() -> Result<Polled, CliError>,
    opts: &WaitOptions,
    started: Duration,
    clock: &mut dyn Clock,
    jitter: &mut dyn FnMut() -> f64,
    progress: &mut dyn FnMut(&Value, Duration),
) -> Result<Value, WaitError> {
    let mut current = first;
    while !is_terminal(&current.body) {
        let mut delay = poll_delay(
            current.retry_after.as_deref(),
            opts.min_interval,
            SystemTime::now(),
            jitter(),
        );
        if let Some(timeout) = opts.timeout {
            let remaining = (started + timeout).saturating_sub(clock.elapsed());
            if remaining.is_zero() {
                return Err(WaitError::TimedOut(current));
            }
            delay = delay.min(remaining);
        }
        progress(&current.body, delay);
        clock.sleep(delay);
        current = fetch().map_err(WaitError::Cli)?;
    }
    Ok(current.body)
}

// ── Building requests from the spec ─────────────────────────────────

/// An HTTP request assembled from a spec method and the parsed flags.
#[derive(Debug, Clone, PartialEq)]
struct ApiRequest {
    method: reqwest::Method,
    /// Relative to the base URL, path parameters filled in, no leading slash.
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<Value>,
    /// Path parameter values by wire name, for the follow-up GET.
    path_params: Map<String, Value>,
}

/// Path segment encoding: everything but RFC 3986 unreserved characters.
const PATH_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

/// The clap id the generated command gave a parameter: its wire name, or
/// `<name>-param` when that collides with a built-in flag.
fn arg_id_for(matches: &clap::ArgMatches, name: &str) -> Option<String> {
    let suffixed = format!("{name}-param");
    [suffixed, name.to_string()]
        .into_iter()
        .find(|id| matches.try_get_raw(id).is_ok())
}

/// Read one parameter's value from the flags, the way the generated command
/// does: JSON for object/array parameters, `null` for nullable ones, enum
/// display names mapped back to wire values, and repeated flags as a list.
fn flag_value(
    matches: &clap::ArgMatches,
    id: &str,
    param: &fern_cli_sdk::openapi::discovery::MethodParameter,
    skip_defaulted: bool,
) -> Option<Value> {
    let from_default = matches.value_source(id) == Some(clap::parser::ValueSource::DefaultValue);
    if from_default && skip_defaulted {
        return None;
    }
    if param.repeated {
        let mut items = Vec::new();
        for v in matches.get_many::<String>(id)? {
            match serde_json::from_str(v) {
                Ok(Value::Array(elems)) => items.extend(elems),
                _ => items.push(Value::String(v.clone())),
            }
        }
        return Some(if param.scalar_or_array && items.len() == 1 {
            items.remove(0)
        } else {
            Value::Array(items)
        });
    }
    let raw = matches.get_one::<String>(id)?;
    if let (true, Some(typed)) = (from_default, &param.default_value) {
        return Some(typed.clone());
    }
    if param.nullable && raw == "null" {
        return Some(Value::Null);
    }
    if matches!(param.param_type.as_deref(), Some("object" | "array")) {
        if let Ok(parsed) = serde_json::from_str(raw) {
            return Some(parsed);
        }
    }
    Some(Value::String(
        param.resolve_enum_display_to_wire(raw).into_owned(),
    ))
}

/// Insert `value` at a dotted body path (`a.b.c`), creating objects on the way.
fn insert_dotted(body: &mut Map<String, Value>, name: &str, value: Value) {
    match name.split_once('.') {
        None => {
            body.insert(name.to_string(), value);
        }
        Some((head, rest)) => {
            let child = body
                .entry(head.to_string())
                .or_insert_with(|| Value::Object(Map::new()));
            if let Value::Object(map) = child {
                insert_dotted(map, rest, value);
            }
        }
    }
}

fn scalar_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Assemble `method`'s request from the flags, routing each spec parameter to
/// its location. `--json` supplies the whole body; `--params` overrides
/// individual flags, as on the generated commands.
fn build_request(
    method: &RestMethod,
    matches: &clap::ArgMatches,
    body_json: Option<&str>,
) -> Result<ApiRequest, CliError> {
    let mut params = Map::new();
    for (name, param) in &method.parameters {
        let Some(id) = arg_id_for(matches, name) else {
            continue;
        };
        let is_body = param.location.as_deref() == Some("body");
        if let Some(v) = flag_value(matches, &id, param, is_body && body_json.is_some()) {
            params.insert(name.clone(), v);
        }
    }
    if let Some(overrides) = matches.try_get_one::<String>("params").ok().flatten() {
        let overrides: Map<String, Value> = serde_json::from_str(overrides)
            .map_err(|e| CliError::Validation(format!("Invalid --params JSON: {e}")))?;
        params.extend(overrides);
    }
    params_to_request(method, params, body_json)
}

fn params_to_request(
    method: &RestMethod,
    params: Map<String, Value>,
    body_json: Option<&str>,
) -> Result<ApiRequest, CliError> {
    let mut path = method.path.trim_start_matches('/').to_string();
    let mut query = Vec::new();
    let mut headers = Vec::new();
    let mut body_fields = Map::new();
    let mut body_flags = Vec::new();
    let mut path_params = Map::new();

    for (name, value) in params {
        let location = method
            .parameters
            .get(&name)
            .and_then(|p| p.location.as_deref());
        match location {
            Some("path") => {
                let encoded = utf8_percent_encode(&scalar_string(&value), PATH_SEGMENT).to_string();
                path = path.replace(&format!("{{{name}}}"), &encoded);
                path_params.insert(name, value);
            }
            Some("header") => headers.push((name, scalar_string(&value))),
            Some("body") => {
                body_flags.push(format!("--{}", name.replace('_', "-")));
                insert_dotted(&mut body_fields, &name, value);
            }
            // Query, and unknown `--params` keys, which the generated
            // executor also sends as query parameters.
            _ => query.push((name, scalar_string(&value))),
        }
    }

    if let Some(missing) = method
        .parameters
        .iter()
        .find(|(name, p)| p.location.as_deref() == Some("path") && !path_params.contains_key(*name))
    {
        return Err(CliError::Validation(format!(
            "Missing required path parameter '{}': pass --{}",
            missing.0,
            missing.0.replace('_', "-")
        )));
    }

    let body = match body_json {
        Some(raw) => {
            if !body_flags.is_empty() {
                body_flags.sort();
                return Err(CliError::Validation(format!(
                    "Cannot combine --json with per-field body flags ({}). Use one or the other.",
                    body_flags.join(", ")
                )));
            }
            Some(
                serde_json::from_str(raw)
                    .map_err(|e| CliError::Validation(format!("Invalid --json body: {e}")))?,
            )
        }
        None if !body_fields.is_empty() => Some(Value::Object(body_fields)),
        None if method.http_method.eq_ignore_ascii_case("post") => Some(json!({})),
        None => None,
    };

    Ok(ApiRequest {
        method: method
            .http_method
            .parse()
            .map_err(|_| CliError::Discovery(format!("bad HTTP method {}", method.http_method)))?,
        path,
        query,
        headers,
        body,
        path_params,
    })
}

/// The GET for the created resource: its path parameters come from the
/// create's (`template_id`) and, for the one the create didn't have, the new id.
fn get_request(get: &RestMethod, create: &ApiRequest, id: &str) -> Result<ApiRequest, CliError> {
    let mut params = Map::new();
    for (name, param) in &get.parameters {
        match param.location.as_deref() {
            Some("path") => {
                let value = create
                    .path_params
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| Value::String(id.to_string()));
                params.insert(name.clone(), value);
            }
            // Carry the API key header over, if one was passed explicitly.
            Some("header") => {
                if let Some((_, v)) = create.headers.iter().find(|(h, _)| h == name) {
                    params.insert(name.clone(), Value::String(v.clone()));
                }
            }
            _ => {}
        }
    }
    params_to_request(get, params, None)
}

/// The `get` command that resumes waiting, with its path parameters in the
/// order the path names them (`--template-id` before `--run-id`).
fn resume_command(target: &Target, get: &ApiRequest, get_method: &RestMethod) -> String {
    let mut cmd = format!("elevenlabs {} get", target.path.join(" "));
    let mut rest = get_method.path.as_str();
    while let Some(start) = rest.find('{') {
        let Some(len) = rest[start..].find('}') else {
            break;
        };
        let name = &rest[start + 1..start + len];
        if let Some(v) = get.path_params.get(name) {
            cmd.push_str(&format!(
                " --{} {}",
                name.replace('_', "-"),
                shell_quote(&scalar_string(v))
            ));
        }
        rest = &rest[start + len..];
    }
    cmd
}

fn shell_quote(s: &str) -> String {
    let safe = !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':' | '/'));
    if safe {
        s.to_string()
    } else {
        format!("'{}'", s.replace('\'', r"'\''"))
    }
}

fn timeout_message(last: &Value, get: &ApiRequest, timeout: Duration) -> String {
    let id = last.get("id").and_then(Value::as_str).unwrap_or("?");
    let what = match get.path_params.get("template_id") {
        Some(t) => format!("Template run {id} of template {}", scalar_string(t)),
        None => format!("Generation {id}"),
    };
    format!(
        "{what} is still {} after {}s; it keeps running on the server",
        status_of(last),
        timeout.as_secs_f64(),
    )
}

// ── The handler ─────────────────────────────────────────────────────

fn resource_at<'a>(doc: &'a RestDescription, path: &[&str]) -> Option<&'a RestResource> {
    let (first, rest) = path.split_first()?;
    rest.iter()
        .try_fold(doc.resources.get(*first)?, |r, name| r.resources.get(*name))
}

fn send(ctx: &AppContext, req: &ApiRequest) -> Result<Polled, CliError> {
    let client = crate::sdk::client(ctx);
    let mut opts = elevenlabs_sdk::RequestOptions::new();
    opts.additional_headers.extend(req.headers.iter().cloned());
    // The generated commands get the intent header from the framework; the
    // SDK executor path does not, so add it here (as `api::request_options`).
    if let Some(encoded) = super::intent::resolved_encoded() {
        opts.additional_headers
            .insert(super::intent::INTENT_HEADER.to_string(), encoded);
    }
    let query = (!req.query.is_empty()).then(|| req.query.clone());
    let raw = crate::sdk::block_on(client.flows.http_client.execute_request_raw::<Value>(
        req.method.clone(),
        &req.path,
        req.body.clone(),
        query,
        Some(opts),
    ))?;
    // `execute_request_raw` returns error responses as `Ok`; see `api::raw_request`.
    if raw.status_code >= 400 {
        return Err(CliError::Api {
            code: raw.status_code,
            message: super::api::api_error_message(&raw.body),
            reason: format!("http_{}", raw.status_code),
            details: Some(raw.body),
            help: None,
        });
    }
    let retry_after = raw
        .headers
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    Ok(Polled {
        body: raw.body,
        status: raw.status_code,
        retry_after,
    })
}

fn base_url(ctx: &AppContext) -> String {
    ctx.base_url_override()
        .map(str::to_string)
        .unwrap_or_else(|| elevenlabs_sdk::ClientConfig::default().base_url)
}

/// Mirrors the generated `--dry-run` output for the create request.
fn dry_run_value(ctx: &AppContext, req: &ApiRequest) -> Value {
    json!({
        "dry_run": true,
        "method": req.method.as_str(),
        "url": format!("{}/{}", base_url(ctx).trim_end_matches('/'), req.path),
        "query_params": req.query,
        // Header values can be credentials (`xi-api-key`); show names only.
        "headers": req.headers.iter().map(|(name, _)| json!([name, "<redacted>"])).collect::<Vec<_>>(),
        "body": req.body,
        "content_type": "application/json",
        "is_multipart_upload": false,
    })
}

fn emit(matches: &clap::ArgMatches, value: &Value) -> Result<(), CliError> {
    let pipeline = fern_cli_sdk::formatter::OutputPipeline::from_matches(matches, "elevenlabs")
        .map_err(|e| CliError::Validation(e.to_string()))?;
    let mut out = std::io::stdout().lock();
    pipeline
        .emit(&mut out, value, false, true)
        .map_err(|e| CliError::Other(e.into()))?;
    out.flush().map_err(|e| CliError::Other(e.into()))
}

fn flag(matches: &clap::ArgMatches, id: &str) -> bool {
    matches
        .try_get_one::<bool>(id)
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false)
}

fn describe(body: &Value, create: &ApiRequest) -> String {
    let id = body.get("id").and_then(Value::as_str).unwrap_or("?");
    match create.path_params.get("template_id") {
        Some(t) => format!("run {id} of template {}", scalar_string(t)),
        None => format!("generation {id}"),
    }
}

fn run(target: &Target, matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let opts = WaitOptions::from_matches(matches)?;
    let resource = resource_at(ctx.spec(), target.path).ok_or_else(|| {
        CliError::Discovery(format!(
            "no `{}` resource in the spec",
            target.path.join(" ")
        ))
    })?;
    let (Some(create_method), Some(get_method)) =
        (resource.methods.get("create"), resource.methods.get("get"))
    else {
        return Err(CliError::Discovery(format!(
            "`{}` has no create/get pair in the spec",
            target.path.join(" ")
        )));
    };

    let body_json = fern_cli_sdk::cli_args::resolve_body_json(matches)?;
    let create = build_request(create_method, matches, body_json.as_deref())?;

    // `--dry-run` behaves as on `create`: show the request, send nothing.
    if super::util::dry_run_flag(matches) {
        return emit(matches, &dry_run_value(ctx, &create));
    }

    let _scope = super::api::command_scope(target.scope);
    let show_progress = std::io::stderr().is_terminal() || flag(matches, "debug");
    let mut clock: Box<dyn Clock> = if std::env::var_os(FAKE_CLOCK_ENV).is_some() {
        Box::new(FakeClock {
            elapsed: Duration::ZERO,
            log: true,
        })
    } else {
        Box::new(RealClock(Instant::now()))
    };

    let started = clock.elapsed();
    let first = send(ctx, &create)?;
    let id = first
        .body
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CliError::Other(anyhow::anyhow!(
                "create response has no `id`: {}",
                first.body
            ))
        })?
        .to_string();
    let get = get_request(get_method, &create, &id)?;

    let mut progress = |body: &Value, next: Duration| {
        if show_progress {
            eprintln!(
                "flows: {} is {}; checking again in {:.1}s",
                describe(body, &create),
                status_of(body),
                next.as_secs_f64()
            );
        }
    };
    let result = wait_until_terminal(
        first,
        || send(ctx, &get),
        &opts,
        started,
        clock.as_mut(),
        &mut || rand::random::<f64>(),
        &mut progress,
    );
    let body = match result {
        Ok(body) => body,
        Err(WaitError::Cli(e)) => return Err(e),
        // An `Api` error rather than `Other`, whose `code: 500` /
        // `internalError` would read as a server or CLI fault. `code` is the
        // last GET's real status; `details` is that response (id, status).
        Err(WaitError::TimedOut(last)) => {
            let timeout = opts.timeout.unwrap_or_default();
            return Err(CliError::Api {
                code: last.status,
                message: timeout_message(&last.body, &get, timeout),
                reason: "waitTimeout".to_string(),
                help: Some(format!(
                    "To keep waiting, run: {}",
                    resume_command(target, &get, get_method)
                )),
                details: Some(last.body),
            });
        }
    };

    emit(matches, &body)?;
    if status_of(&body) == "failed" {
        eprintln!(
            "flows: {} failed (exit code {EXIT_FAILED})",
            describe(&body, &create)
        );
        std::process::exit(EXIT_FAILED);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: Duration = Duration::from_secs(3);

    #[test]
    fn delay_follows_retry_after_seconds() {
        let now = SystemTime::now();
        assert_eq!(
            poll_delay(Some("10"), MIN, now, 0.0),
            Duration::from_secs(10)
        );
        // Full jitter adds 20%.
        assert_eq!(
            poll_delay(Some("10"), MIN, now, 1.0),
            Duration::from_secs(12)
        );
    }

    #[test]
    fn delay_follows_retry_after_http_date() {
        let now = SystemTime::now();
        let date = httpdate::fmt_http_date(now + Duration::from_secs(20));
        let d = poll_delay(Some(&date), MIN, now, 0.0);
        // HTTP dates have one-second resolution.
        assert!(
            d > Duration::from_secs(18) && d <= Duration::from_secs(20),
            "{d:?}"
        );
    }

    #[test]
    fn delay_never_goes_below_the_minimum() {
        let now = SystemTime::now();
        for header in [
            None,
            Some("0"),
            Some("1"),
            Some("soon"),
            Some("-5"),
            Some(""),
        ] {
            assert_eq!(poll_delay(header, MIN, now, 0.0), MIN, "{header:?}");
        }
        let past = httpdate::fmt_http_date(now - Duration::from_secs(60));
        assert_eq!(poll_delay(Some(&past), MIN, now, 0.0), MIN);
    }

    #[test]
    fn options_are_validated() {
        let ok = WaitOptions::new(None, None).unwrap();
        assert_eq!(ok.min_interval, MIN);
        assert_eq!(ok.timeout, None);
        assert_eq!(
            WaitOptions::new(Some(f64::INFINITY), None).unwrap().timeout,
            None
        );
        assert_eq!(
            WaitOptions::new(Some(0.0), Some(0.5)).unwrap(),
            WaitOptions {
                timeout: Some(Duration::ZERO),
                min_interval: Duration::from_millis(500)
            }
        );
        for (timeout, min) in [
            (None, Some(0.49)),
            (None, Some(f64::NAN)),
            (None, Some(f64::INFINITY)),
            (None, Some(-1.0)),
            (Some(-1.0), None),
            (Some(f64::NAN), None),
        ] {
            assert!(
                matches!(WaitOptions::new(timeout, min), Err(CliError::Validation(_))),
                "{timeout:?} {min:?}"
            );
        }
    }

    fn polled(status: &str, retry_after: Option<&str>) -> Polled {
        Polled {
            body: json!({"id": "g1", "status": status}),
            status: 200,
            retry_after: retry_after.map(str::to_string),
        }
    }

    /// Run the loop over canned responses; returns the result, the sleeps and
    /// how many GETs ran.
    fn simulate(
        first: Polled,
        rest: Vec<Polled>,
        opts: WaitOptions,
    ) -> (Result<Value, WaitError>, Vec<Duration>, usize) {
        struct Recording<'a>(FakeClock, &'a mut Vec<Duration>);
        impl Clock for Recording<'_> {
            fn elapsed(&self) -> Duration {
                self.0.elapsed()
            }
            fn sleep(&mut self, d: Duration) {
                self.1.push(d);
                self.0.sleep(d);
            }
        }
        let mut sleeps = Vec::new();
        let mut gets = 0;
        let mut rest = rest.into_iter();
        let result = {
            let mut clock = Recording(FakeClock::default(), &mut sleeps);
            wait_until_terminal(
                first,
                || {
                    gets += 1;
                    Ok(rest.next().expect("polled past the last canned response"))
                },
                &opts,
                Duration::ZERO,
                &mut clock,
                &mut || 0.0,
                &mut |_, _| {},
            )
        };
        (result, sleeps, gets)
    }

    #[test]
    fn terminal_create_response_is_not_polled() {
        let opts = WaitOptions::new(None, None).unwrap();
        let (result, sleeps, gets) = simulate(polled("completed", Some("5")), vec![], opts);
        assert_eq!(result.unwrap()["status"], "completed");
        assert!(sleeps.is_empty());
        assert_eq!(gets, 0);
    }

    #[test]
    fn create_retry_after_sets_the_first_wait() {
        let opts = WaitOptions::new(None, None).unwrap();
        let (result, sleeps, gets) = simulate(
            polled("pending", Some("7")),
            vec![polled("processing", None), polled("failed", Some("9"))],
            opts,
        );
        assert_eq!(result.unwrap()["status"], "failed");
        assert_eq!(sleeps, vec![Duration::from_secs(7), MIN]);
        assert_eq!(gets, 2);
    }

    #[test]
    fn timeout_cuts_the_last_sleep_and_makes_one_final_get() {
        let opts = WaitOptions::new(Some(10.0), None).unwrap();
        let (result, sleeps, gets) = simulate(
            polled("pending", Some("6")),
            vec![
                polled("processing", Some("6")),
                polled("processing", Some("6")),
            ],
            opts,
        );
        assert!(
            matches!(result, Err(WaitError::TimedOut(ref p)) if p.body["status"] == "processing")
        );
        assert_eq!(sleeps, vec![Duration::from_secs(6), Duration::from_secs(4)]);
        assert_eq!(gets, 2);
    }

    #[test]
    fn zero_timeout_gives_up_without_polling() {
        let opts = WaitOptions::new(Some(0.0), None).unwrap();
        let (result, sleeps, gets) = simulate(polled("pending", None), vec![], opts);
        assert!(matches!(result, Err(WaitError::TimedOut(_))));
        assert!(sleeps.is_empty());
        assert_eq!(gets, 0);
    }

    #[test]
    fn result_at_the_deadline_is_returned() {
        let opts = WaitOptions::new(Some(3.0), None).unwrap();
        let (result, _, _) = simulate(
            polled("pending", None),
            vec![polled("completed", None)],
            opts,
        );
        assert_eq!(result.unwrap()["status"], "completed");
    }

    const FULL_SPEC: &str = include_str!("../openapi0.json");

    fn embedded_spec() -> RestDescription {
        fern_cli_sdk::openapi::load_openapi_spec(FULL_SPEC, "elevenlabs").unwrap()
    }

    /// Every `#/components/<kind>/<name>` reference reachable from `value`.
    fn collect_refs(value: &Value, components: &Value, out: &mut Map<String, Value>) {
        match value {
            Value::Object(map) => {
                if let Some(target) = map.get("$ref").and_then(Value::as_str) {
                    if let Some(rest) = target.strip_prefix("#/components/") {
                        let (kind, name) = rest.split_once('/').unwrap();
                        let kind_map = out
                            .entry(kind.to_string())
                            .or_insert_with(|| Value::Object(Map::new()))
                            .as_object_mut()
                            .unwrap();
                        if !kind_map.contains_key(name) {
                            let resolved = components[kind][name].clone();
                            kind_map.insert(name.to_string(), resolved.clone());
                            collect_refs(&resolved, components, out);
                        }
                    }
                }
                map.values().for_each(|v| collect_refs(v, components, out));
            }
            Value::Array(items) => items.iter().for_each(|v| collect_refs(v, components, out)),
            _ => {}
        }
    }

    /// The `/v1/flows/` slice of the full spec: those paths, the components
    /// they reference (transitively), any security schemes, and every other
    /// top-level key unchanged.
    fn flows_slice(full: &Value) -> Value {
        let mut out = full.as_object().unwrap().clone();
        let paths: Map<String, Value> = full["paths"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(p, _)| p.starts_with("/v1/flows/"))
            .map(|(p, v)| (p.clone(), v.clone()))
            .collect();
        let mut components = Map::new();
        collect_refs(
            &Value::Object(paths.clone()),
            &full["components"],
            &mut components,
        );
        if let Some(schemes) = full["components"].get("securitySchemes") {
            components.insert("securitySchemes".into(), schemes.clone());
        }
        out.insert("paths".into(), Value::Object(paths));
        out.insert("components".into(), Value::Object(components));
        Value::Object(out)
    }

    #[test]
    fn flows_spec_matches_openapi0() {
        let full: Value = serde_json::from_str(FULL_SPEC).unwrap();
        let expected = flows_slice(&full);
        let current: Value = serde_json::from_str(FLOWS_SPEC).unwrap_or(Value::Null);
        if current == expected {
            return;
        }
        if std::env::var_os("UPDATE_FLOWS_SPEC").is_some() {
            let path = concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cli/elevenlabs/workflow/flows_spec.json"
            );
            std::fs::write(path, serde_json::to_string(&expected).unwrap() + "\n").unwrap();
            return;
        }
        panic!(
            "flows_spec.json is out of date with openapi0.json; regenerate it with \
             `UPDATE_FLOWS_SPEC=1 cargo test --bin elevenlabs flows_spec`"
        );
    }

    #[test]
    fn sibling_flags_match_the_full_spec_create() {
        let full_tree = fern_cli_sdk::openapi::commands::build_cli(&embedded_spec());
        let slice = fern_cli_sdk::openapi::load_openapi_spec(FLOWS_SPEC, "elevenlabs").unwrap();
        let slice_tree = fern_cli_sdk::openapi::commands::build_cli(&slice);
        let args = |tree: &clap::Command, target: &Target| -> Vec<String> {
            let create = find_command(tree, target.path)
                .and_then(|g| g.find_subcommand("create"))
                .expect(target.scope);
            create.get_arguments().map(|a| format!("{a:?}")).collect()
        };
        for target in TARGETS {
            assert_eq!(
                args(&full_tree, target),
                args(&slice_tree, target),
                "{}",
                target.scope
            );
        }
    }

    #[test]
    fn every_target_has_a_create_get_pair_and_a_generated_create() {
        let doc = embedded_spec();
        let tree = fern_cli_sdk::openapi::commands::build_cli(&doc);
        for target in TARGETS {
            let resource = resource_at(&doc, target.path).expect(target.scope);
            assert!(resource.methods.contains_key("create"), "{}", target.scope);
            assert!(resource.methods.contains_key("get"), "{}", target.scope);
            let create = find_command(&tree, target.path)
                .and_then(|g| g.find_subcommand("create"))
                .expect(target.scope);
            let sibling = sibling_command(target, create);
            let ids: Vec<&str> = sibling
                .get_arguments()
                .map(|a| a.get_id().as_str())
                .collect();
            assert!(
                ids.contains(&"json") && ids.contains(&"wait-timeout"),
                "{ids:?}"
            );
            assert!(!ids.contains(&"page-all"), "{ids:?}");
        }
    }

    #[test]
    fn run_request_is_built_from_the_generated_flags() {
        let doc = embedded_spec();
        let tree = fern_cli_sdk::openapi::commands::build_cli(&doc);
        let target = &TARGETS[3];
        let create_cmd = find_command(&tree, target.path)
            .and_then(|g| g.find_subcommand("create"))
            .unwrap();
        let matches = sibling_command(target, create_cmd)
            .try_get_matches_from([
                "create-and-wait",
                "--template-id",
                "tpl/1",
                "--inputs",
                r#"{"prompt":"a corgi"}"#,
                "--version-id",
                "latest",
            ])
            .unwrap();
        let resource = resource_at(&doc, target.path).unwrap();
        let req = build_request(&resource.methods["create"], &matches, None).unwrap();
        assert_eq!(req.method, reqwest::Method::POST);
        assert_eq!(req.path, "v1/flows/templates/tpl%2F1/runs");
        assert_eq!(
            req.body,
            Some(json!({"inputs": {"prompt": "a corgi"}, "version_id": "latest"}))
        );

        let get = get_request(&resource.methods["get"], &req, "run_9").unwrap();
        assert_eq!(get.method, reqwest::Method::GET);
        assert_eq!(get.path, "v1/flows/templates/tpl%2F1/runs/run_9");
        assert_eq!(get.body, None);
        assert_eq!(
            resume_command(target, &get, &resource.methods["get"]),
            "elevenlabs flows templates runs get --template-id tpl/1 --run-id run_9"
        );

        let conflict = build_request(
            &resource.methods["create"],
            &matches,
            Some(r#"{"inputs":{}}"#),
        );
        assert!(matches!(conflict, Err(CliError::Validation(m)) if m.contains("--inputs")));
    }
}
