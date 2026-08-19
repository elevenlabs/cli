//! Auto-generated wire tests by @fern-api/cli-generator.
//!
//! Each test stands up an in-process `wiremock` server, points the
//! generated CLI at it via `--base-url`, drives one endpoint from an IR
//! example, and asserts the request the CLI *sent* — not just the response
//! it renders:
//!   - method + path, scalar query params, and auth headers;
//!   - the request body: opaque JSON bodies (`--json`) are matched
//!     field-by-field, and file/multipart uploads are driven with a real
//!     temp fixture file whose bytes must appear in the request body (so an
//!     upload the runtime sends as a text path, not a file part, is caught);
//!   - for the happy-path case, the rendered response body on stdout;
//!   - each case also has a negative twin: the mock serves a non-2xx with a
//!     JSON error body and the CLI is required to exit non-zero.
//!
//! No docker, no network — runs under a plain `cargo test`. Regenerated on
//! every `fern generate`; do not edit by hand.
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use wiremock::matchers::{
    body_partial_json as match_body_partial_json, body_string_contains as match_body_contains,
    header as match_header, header_regex as match_header_regex, method as match_method, path as match_path,
    query_param as match_query_param,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

use fern_cli_sdk::openapi::discovery::{BodyEncoding, RestResource};
use fern_cli_sdk::openapi::load_openapi_spec;

const MANIFEST: &str = include_str!("../wiremock/wire-test-cases.json");

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "binaryName")]
    binary_name: String,
    #[serde(rename = "rootGroup")]
    root_group: Option<String>,
    specs: Vec<SpecEntry>,
    #[serde(rename = "authEnvVars")]
    auth_env_vars: Vec<AuthEnvVar>,
    #[serde(rename = "authMock")]
    auth_mock: Option<AuthMock>,
    #[serde(rename = "loginTokenSetup")]
    login_token_setup: Option<LoginTokenSetup>,
    /// `ENDPOINT_SECURITY` auth: each endpoint picks its own scheme, so no single
    /// credential can be required across the board. Suppresses the bearer
    /// assertion entirely.
    #[serde(rename = "endpointSecurityAuth", default)]
    endpoint_security_auth: bool,
    cases: Vec<Case>,
}

/// A credential env var the CLI reads, and the value to export.
///
/// The value is not always a placeholder: `mock-utils` matches basic auth with an
/// exact base64 of `test-username:test-password`, so those halves must be seeded
/// verbatim or the CLI sends a well-formed `Authorization` header that can never
/// match its own mock. Presence-only schemes (bearer, apiKey) get `"test"`.
#[derive(Deserialize)]
struct AuthEnvVar {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct LoginTokenSetup {
    #[serde(rename = "schemeName")]
    scheme_name: String,
    token: String,
}

#[derive(Deserialize)]
struct SpecEntry {
    file: String,
    namespace: Option<String>,
}

#[derive(Deserialize)]
struct AuthMock {
    method: String,
    path: String,
    #[serde(rename = "responseBody")]
    response_body: String,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    method: String,
    path: String,
    params: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    body: serde_json::Value,
    #[serde(rename = "queryMatchers", default)]
    query_matchers: Vec<QueryMatcher>,
    #[serde(rename = "headerMatchers", default)]
    header_matchers: Vec<HeaderMatcher>,
    /// Whether the endpoint declares auth (from the IR). Gates the credential
    /// assertions below: an endpoint that declares none cannot be required to
    /// send one. Defaults to `false` so an older manifest degrades to "assert
    /// nothing" rather than "assert a credential nobody sends".
    #[serde(rename = "requiresAuth", default)]
    requires_auth: bool,
    #[serde(rename = "multipartFields", default)]
    multipart_fields: Vec<MultipartFieldSpec>,
    #[serde(rename = "expectError", default)]
    expect_error: bool,
    #[serde(rename = "omitOptionalFiles", default)]
    omit_optional_files: bool,
    /// Body properties this case took from the OpenAPI spec because the IR
    /// example omitted them while the spec marks them required. Reported in
    /// failure diagnostics so a spec-derived value is never mistaken for one an
    /// API author wrote.
    #[serde(rename = "specFilledBodyProperties", default)]
    spec_filled_body_properties: Vec<String>,
    response: ExpectedResponse,
}

#[derive(Deserialize)]
struct MultipartFieldSpec {
    #[serde(rename = "wireName")]
    wire_name: String,
    #[serde(rename = "isFile")]
    is_file: bool,
    #[serde(rename = "isOptional", default)]
    is_optional: bool,
    /// The spec's `encoding.<field>.contentType`, when declared. `None` for the
    /// common spec that declares no `encoding` object.
    #[serde(rename = "contentType", default)]
    content_type: Option<String>,
}

#[derive(Deserialize)]
struct QueryMatcher {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct HeaderMatcher {
    name: String,
    #[serde(rename = "equalTo")]
    equal_to: Option<String>,
    matches: Option<String>,
}

#[derive(Deserialize)]
struct ExpectedResponse {
    status: u16,
    body: String,
}

fn load_manifest() -> Manifest {
    serde_json::from_str(MANIFEST).expect("wire-test-cases.json is valid JSON")
}

/// Matcher asserting a substring is **absent** from the request body.
/// `wiremock` ships only positive body matchers, but proving the CLI emitted
/// *no* part for a field it was supposed to omit needs a negative one.
struct BodyDoesNotContain(String);

impl wiremock::Match for BodyDoesNotContain {
    fn matches(&self, request: &wiremock::Request) -> bool {
        !String::from_utf8_lossy(&request.body).contains(&self.0)
    }
}

/// The literal `Content-Disposition` fragment reqwest emits for a multipart
/// field named `field`, or `None` when reqwest would percent-encode the name
/// (`name*=utf-8''…`) instead of writing it verbatim, which happens for
/// non-ASCII names and for names containing `"`, `%`, CR or LF. Returning
/// `None` keeps the harness from requiring a fragment that will never appear.
fn content_disposition_name(field: &str) -> Option<String> {
    let is_verbatim = field
        .chars()
        .all(|ch| ch.is_ascii() && !matches!(ch, '"' | '%' | '\r' | '\n' | '\\'));
    if is_verbatim {
        Some(format!("name=\"{field}\""))
    } else {
        None
    }
}

/// Normalize a URL path for comparison: ensure a leading slash and drop any
/// trailing slash.
fn normalize_path(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

/// Canonicalize a path *template* for resolution: normalize it, then collapse
/// every `{param}` placeholder to a bare `{}` so two templates match
/// regardless of parameter *names*. The CLI can rename a path parameter (e.g.
/// to disambiguate it from a request-body field of the same name), so the
/// manifest's template (`…/{idTypePathParam}`) and the spec's template
/// (`…/{idType}`) describe the same route but differ only in placeholder name.
fn canonicalize_path_template(path: &str) -> String {
    let normalized = normalize_path(path);
    let mut out = String::with_capacity(normalized.len());
    let mut in_placeholder = false;
    for ch in normalized.chars() {
        match ch {
            '{' => {
                in_placeholder = true;
                out.push('{');
            }
            '}' => {
                in_placeholder = false;
                out.push('}');
            }
            _ if in_placeholder => {}
            _ => out.push(ch),
        }
    }
    out
}

/// The `{param}` placeholder names of a path template, in order of appearance.
fn ordered_placeholders(path: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut current: Option<String> = None;
    for ch in path.chars() {
        match ch {
            '{' => current = Some(String::new()),
            '}' => {
                if let Some(name) = current.take() {
                    names.push(name);
                }
            }
            _ => {
                if let Some(name) = current.as_mut() {
                    name.push(ch);
                }
            }
        }
    }
    names
}

/// Map each manifest path-param name to the wire name the generated CLI expects
/// in `--params`. They coincide for every ordinary parameter, but the CLI can
/// rename a path param in its IR identity to disambiguate it from a body field
/// of the same name (`idType` → `idTypePathParam`) while still reading it off
/// the baked spec by the original wire name. The manifest template carries the
/// renamed identity and the resolved spec route carries the wire name, and
/// `canonicalize_path_template` guarantees they have the same placeholders in
/// the same order — so pair them positionally.
fn path_param_wire_names(manifest_path: &str, spec_path: &str) -> std::collections::HashMap<String, String> {
    ordered_placeholders(manifest_path)
        .into_iter()
        .zip(ordered_placeholders(spec_path))
        .filter(|(manifest_name, spec_name)| manifest_name != spec_name)
        .collect()
}

/// A resolved command plus the body-input modality read straight off the
/// `RestMethod` the CLI itself uses — so the harness drives each endpoint the
/// same way the CLI expects, and can tell when it can't drive one at all.
#[derive(Clone)]
struct CommandInfo {
    chain: Vec<String>,
    http_method: String,
    path: String,
    /// Endpoint registers `--json` (an opaque JSON request body).
    has_json_body: bool,
    /// Binary request body — the CLI wants a real file via a typed flag.
    is_binary: bool,
    /// Flag name for the binary request-body file (`--<flag> <path>`), if any.
    binary_flag: Option<String>,
    /// multipart/form-data — per-field file/value flags, not `--json`.
    is_multipart: bool,
    /// The request body is `application/json` (vs
    /// `application/x-www-form-urlencoded`). Only JSON bodies are driven via
    /// `--json` and matched with a partial-JSON matcher; a form body wouldn't
    /// parse as JSON, so the harness skips those (see `run_case`). A JSON body
    /// the CLI *also* flattened into per-field flags is still driven here — the
    /// `--json` flag is registered alongside the per-field flags.
    body_is_json_encoded: bool,
    /// Streaming/SSE response — stdout is chunked, so a byte-exact comparison
    /// against the mock's single payload doesn't hold.
    is_streaming: bool,
}

/// Build a `CommandInfo` for a single method at the given command `chain`.
fn make_command(chain: Vec<String>, method: &fern_cli_sdk::openapi::discovery::RestMethod) -> CommandInfo {
    CommandInfo {
        chain,
        http_method: method.http_method.to_uppercase(),
        path: method.path.clone(),
        has_json_body: method.request.is_some(),
        is_binary: method.binary_request_body.is_some(),
        binary_flag: method.binary_request_body.as_ref().map(|b| b.flag_name.clone()),
        is_multipart: !method.multipart_fields.is_empty(),
        body_is_json_encoded: matches!(method.body_encoding, BodyEncoding::Json),
        is_streaming: method.streaming.is_some(),
    }
}

/// Recursively collect one `CommandInfo` per method from a resource tree,
/// prefixing every chain with `prefix` (root group + spec namespace).
fn collect_commands(resources: &HashMap<String, RestResource>, prefix: &[String], out: &mut Vec<CommandInfo>) {
    for (name, resource) in resources {
        let mut chain = prefix.to_vec();
        chain.push(name.clone());
        for (method_name, method) in &resource.methods {
            let mut full = chain.clone();
            full.push(method_name.clone());
            out.push(make_command(full, method));
        }
        collect_commands(&resource.resources, &chain, out);
    }
}

/// Collect commands for one spec, replicating the SDK's namespace-mount
/// semantics (`merge_into_path` in `openapi/app.rs`).
///
/// A spec bound with `.spec_under("<namespace>", …)` nests under the
/// namespace, but the SDK performs *stutter elision*: if the spec's discovery
/// tree has a top-level resource whose name equals the (leaf) namespace
/// segment, that resource's methods and sub-resources are hoisted directly
/// into the namespace node — so `<ns> <ns> <op>` collapses to `<ns> <op>`.
/// This happens whenever a spec is untagged and Fern groups every operation
/// under a resource derived from the shared path prefix (e.g. `v1`) that
/// matches the version namespace. Mirror it here so the resolved command
/// chain matches the binary's actual command tree.
fn collect_spec_commands(
    resources: &HashMap<String, RestResource>,
    root_prefix: &[String],
    namespace: Option<&str>,
    out: &mut Vec<CommandInfo>,
) {
    let segments: Vec<String> = match namespace {
        Some(ns) => ns.split('/').filter(|s| !s.is_empty()).map(str::to_string).collect(),
        None => Vec::new(),
    };
    if segments.is_empty() {
        collect_commands(resources, root_prefix, out);
        return;
    }

    // Prefix up to and including the namespace node.
    let mut ns_prefix = root_prefix.to_vec();
    ns_prefix.extend(segments.iter().cloned());
    let leaf = segments.last().expect("segments non-empty");

    // Non-matching top-level resources are ordinary children of the namespace.
    let others: HashMap<String, RestResource> = resources
        .iter()
        .filter(|(name, _)| name.as_str() != leaf.as_str())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    collect_commands(&others, &ns_prefix, out);

    // A resource matching the namespace leaf is hoisted into the namespace node.
    if let Some(matching) = resources.get(leaf.as_str()) {
        for (method_name, method) in &matching.methods {
            let mut full = ns_prefix.clone();
            full.push(method_name.clone());
            out.push(make_command(full, method));
        }
        collect_commands(&matching.resources, &ns_prefix, out);
    }
}

/// Resolve the CLI command for a `(method, path)` by loading the baked specs
/// the binary runs on and matching against the SDK's discovery tree.
fn resolve_command(manifest: &Manifest, method: &str, path: &str) -> CommandInfo {
    let mut commands: Vec<CommandInfo> = Vec::new();
    for spec in &manifest.specs {
        let spec_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), spec.file);
        let contents = std::fs::read_to_string(&spec_path)
            .unwrap_or_else(|e| panic!("failed to read baked spec {spec_path}: {e}"));
        let doc = load_openapi_spec(&contents, &manifest.binary_name)
            .unwrap_or_else(|e| panic!("failed to parse baked spec {spec_path}: {e:?}"));
        let mut root_prefix: Vec<String> = Vec::new();
        if let Some(root_group) = &manifest.root_group {
            root_prefix.push(root_group.clone());
        }
        collect_spec_commands(&doc.resources, &root_prefix, spec.namespace.as_deref(), &mut commands);
    }

    let want_method = method.to_uppercase();
    let want_path = canonicalize_path_template(path);

    // Exact match on method + canonicalized path template.
    let mut hits: Vec<CommandInfo> = commands
        .iter()
        .filter(|c| c.http_method == want_method && canonicalize_path_template(&c.path) == want_path)
        .cloned()
        .collect();

    // Fallback: tolerate base-path prefixes by matching path suffixes.
    if hits.is_empty() {
        hits = commands
            .iter()
            .filter(|c| {
                let np = canonicalize_path_template(&c.path);
                c.http_method == want_method && (np.ends_with(&want_path) || want_path.ends_with(&np))
            })
            .cloned()
            .collect();
    }

    match hits.len() {
        1 => hits.pop().unwrap(),
        0 => {
            let available: Vec<String> = commands
                .iter()
                .map(|c| {
                    format!("{} {} ({} {})", manifest.binary_name, c.chain.join(" "), c.http_method, c.path)
                })
                .collect();
            panic!(
                "no CLI command found for {method} {path}.
available commands:
  {}",
                available.join("
  ")
            )
        }
        // Ambiguous (multi-spec with the same method+path). Take the first
        // deterministically; multi-spec disambiguation is best-effort.
        _ => hits.remove(0),
    }
}

/// Whether two JSON values share the same top-level kind (both objects, both
/// arrays, …). Used to gate the exact body comparison: when the CLI re-shapes
/// output (streaming/NDJSON collected into an array) it diverges in kind from
/// the mock's single payload, and an exact match would be meaningless.
fn same_json_kind(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    use serde_json::Value::{Array, Bool, Null, Number, Object, String as JsonString};
    matches!(
        (a, b),
        (Object(_), Object(_))
            | (Array(_), Array(_))
            | (JsonString(_), JsonString(_))
            | (Number(_), Number(_))
            | (Bool(_), Bool(_))
            | (Null, Null)
    )
}

/// Substitute `{param}` placeholders in a path template with values from the
/// case params, so we can assert the request landed on the resolved path.
fn substitute_path(template: &str, params: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut path = template.to_string();
    for (key, value) in params {
        let placeholder = format!("{{{key}}}");
        if path.contains(&placeholder) {
            let rendered = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            path = path.replace(&placeholder, &rendered);
        }
    }
    path
}

/// Render the requests the mock server actually received, for diagnostics.
/// Renders the full URL, query string included: a stray or missing query
/// parameter is invisible when only the path is printed, and that is precisely
/// the class of difference a matcher rejects.
fn describe_received(requests: &[wiremock::Request]) -> String {
    if requests.is_empty() {
        return "  (the server received no requests at all)".to_string();
    }
    requests
        .iter()
        .enumerate()
        .map(|(index, request)| {
            let headers: Vec<String> = request
                .headers
                .iter()
                .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("<non-utf8>")))
                .collect();
            let body = String::from_utf8_lossy(&request.body);
            let body = if body.chars().count() > 2000 {
                format!("{}… <truncated>", body.chars().take(2000).collect::<String>())
            } else {
                body.to_string()
            };
            format!(
                "  #{} {} {}\n     headers: {}\n     body: {}",
                index + 1,
                request.method,
                request.url,
                headers.join(", "),
                body
            )
        })
        .collect::<Vec<_>>()
        .join("
")
}

/// Assert the case's mock matched exactly one request, with everything needed to
/// debug a miss: the command, the process outcome, and the requests the server
/// actually saw next to what the mock demanded.
///
/// Every case type routes through this, so a matcher miss reads the same whether
/// the CLI was expected to succeed or fail. Relying on `MockServer`'s drop-time
/// verification instead would report a bare "expected 1, got 0" with no command,
/// no stderr, and no way to tell "never sent the request" from "sent the wrong
/// one" — which is exactly the failure mode negative cases hit, since a CLI that
/// dies early still satisfies their non-zero-exit assertion.
#[allow(clippy::too_many_arguments)]
async fn assert_request_matched(
    id: &str,
    guard: &wiremock::MockGuard,
    server: &MockServer,
    binary_name: &str,
    args: &[String],
    exit_code: Option<i32>,
    stdout: &str,
    stderr: &str,
    expected_method: &str,
    expected_path: &str,
    spec_filled_body_properties: &[String],
) {
    let matched = guard.received_requests().await;
    if matched.len() == 1 {
        return;
    }
    let received = server.received_requests().await.unwrap_or_default();
    // Self-probe. The CLI's request and this one differ only in who sent them, so
    // if the mock matches this but not the CLI's, the difference is in the request
    // (something the dump below does not surface); if it matches neither, the mock
    // is not matching in this environment at all. Without this the two are
    // indistinguishable, which is exactly the wall a mock that misses only on CI
    // puts you against. Meaningless for cases carrying body matchers — a bodyless
    // probe cannot satisfy those — so it is reported, not asserted on.
    let probe_url = format!("{}{}", server.uri(), expected_path);
    let probe_method = reqwest::Method::from_bytes(expected_method.as_bytes())
        .unwrap_or(reqwest::Method::GET);
    let probe_outcome = match reqwest::Client::new()
        .request(probe_method, &probe_url)
        .send()
        .await
    {
        Ok(response) => format!("HTTP {}", response.status().as_u16()),
        Err(e) => format!("transport error: {e}"),
    };
    let matched_after_probe = guard.received_requests().await.len();
    // Byte-level comparison. `PathExactMatcher` is a plain
    // `request.url.path() == expected` string equality, so if these render
    // identically and still do not match, the difference is invisible
    // (whitespace, a zero-width character) or the mock is not in the server's
    // set at all — and those need different fixes. Rendering with `{:?}` plus a
    // length makes the first case obvious instead of a contradiction.
    let path_forensics = {
        let mut lines = vec![format!(
            "expected {:?} (len {})",
            expected_path,
            expected_path.len()
        )];
        for (index, request) in received.iter().enumerate() {
            lines.push(format!(
                "received[{index}] path {:?} (len {}), full url {:?}",
                request.url.path(),
                request.url.path().len(),
                request.url.as_str()
            ));
        }
        lines.join("
    ")
    };
    // Naming the spec-filled properties keeps a spec-derived value from being
    // mistaken for one an API author wrote — and points at the IR/spec
    // disagreement that made the repair necessary in the first place.
    let spec_filled_note = if spec_filled_body_properties.is_empty() {
        String::new()
    } else {
        format!(
            "\n  note: these required body properties came from the OpenAPI spec, not from the endpoint's example, because the example omitted them: {}",
            spec_filled_body_properties.join(", ")
        )
    };
    panic!(
        "{id}: mock matched {} requests, expected exactly 1.\n  expected: {expected_method} {expected_path}\n  command: {binary_name} {}\n  exit code: {exit_code:?}\n  stdout: {stdout}\n  stderr: {stderr}\n  requests the server received:\n{}\n  note: a request listed above that was not matched means a query, header, or body matcher rejected it — diff it against this case in wiremock/wire-test-cases.json{spec_filled_note}
  self-probe: {expected_method} {probe_url} -> {probe_outcome}; mock matched {matched_after_probe} after probing (a match here means the mock is live and the CLI's request differed; no match means the mock is not matching in this environment)
  path forensics:
    {path_forensics}",
        matched.len(),
        args.join(" "),
        describe_received(&received)
    );
}

async fn run_case(id: &str) {
    let manifest = load_manifest();
    let case = manifest
        .cases
        .iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("wire-test case {id} not found in manifest"));

    let command = resolve_command(&manifest, &case.method, &case.path);

    // A non-JSON (form-url-encoded) body can't be sent via `--json` and there
    // is no form-body matcher here — skip those (the test passes) and log why,
    // rather than emit a guaranteed failure. JSON bodies are driven below via
    // `--json`, *including* ones the CLI also flattened into per-field flags
    // (the `--json` flag is registered alongside them). Binary and multipart
    // uploads are driven with a real fixture file and we assert the bytes land
    // in the request body — exactly what catches an upload the runtime
    // mis-serializes (e.g. sending a filename as a text part).
    if command.has_json_body && !command.body_is_json_encoded {
        eprintln!(
            "skipping wire test {id} ({} {}): non-JSON request body encoding is not driven by the harness",
            case.method, case.path
        );
        return;
    }
    if command.is_multipart && case.multipart_fields.is_empty() {
        eprintln!(
            "skipping wire test {id} ({} {}): multipart endpoint has no IR field metadata to drive",
            case.method, case.path
        );
        return;
    }

    let expected_path = substitute_path(&case.path, &case.params);

    // A per-case scratch dir for any fixture file(s) the request needs.
    // Distinct from the login-keyring HOME dir, and removed at the end.
    let fixtures_dir = std::env::temp_dir().join(format!("{}-wire-files-{id}", manifest.binary_name));
    let _ = std::fs::remove_dir_all(&fixtures_dir);

    // Plan how to drive the request body: extra CLI flags to append, plus
    // substrings that MUST appear in the received request body. For an upload,
    // each file field gets a temp file whose unique marker bytes must show up
    // in the multipart body — so a runtime that sends the file's *path* as a
    // text part (the optional-file bug) fails to match and the test fails.
    let mut body_flag_args: Vec<String> = Vec::new();
    let mut required_body_substrings: Vec<String> = Vec::new();
    let mut forbidden_body_substrings: Vec<String> = Vec::new();

    if command.is_multipart {
        std::fs::create_dir_all(&fixtures_dir).expect("create multipart fixture dir");
        for field in &case.multipart_fields {
            // Exercise the valid "optional file omitted" shape: drop optional
            // file fields and send only what's required. The field must then be
            // *absent* from the wire body — asserted below via
            // `forbidden_body_substrings`, since a runtime that turns an absent
            // optional file into an empty (or path-valued) part would otherwise
            // pass unnoticed.
            if case.omit_optional_files && field.is_file && field.is_optional {
                if let Some(disposition) = content_disposition_name(&field.wire_name) {
                    forbidden_body_substrings.push(disposition);
                }
                continue;
            }
            let flag = fern_cli_sdk::to_kebab_flag(&field.wire_name);
            // Every part sent must name itself in its Content-Disposition, so a
            // dropped or misnamed part fails the match. Skipped for names
            // reqwest percent-encodes (`name*=utf-8''…`) rather than emitting
            // verbatim — there the value/marker checks below carry the assertion.
            let disposition = content_disposition_name(&field.wire_name);
            if field.is_file {
                let marker = format!("wire-fixture-{id}-{}-bytes", field.wire_name);
                // The fixture carries a `.txt` extension deliberately. A part's
                // media type is resolved as declared-`encoding` → extension →
                // `application/octet-stream`, and an extensionless fixture would
                // only ever exercise that last fallback — which is precisely the
                // regression worth guarding: labelling every upload
                // `application/octet-stream` makes servers that validate a part's
                // media type reject it — a real API answered `Invalid file type`
                // for a `.txt` mislabelled that way.
                let file_name = format!("{}.txt", field.wire_name);
                let file_path = fixtures_dir.join(&file_name);
                std::fs::write(&file_path, marker.as_bytes()).expect("write multipart fixture file");
                body_flag_args.push(format!("--{flag}"));
                body_flag_args.push(file_path.to_string_lossy().into_owned());
                // A file part carries `filename="..."` *on its own
                // Content-Disposition* and the file's bytes. Requiring the
                // `name="…"; filename="` pair (not a bare `filename="`) binds the
                // attribute to *this* field, so a sibling file part can't satisfy
                // it. The optional-file bug sends the path as a text part — no
                // filename on that part and none of the marker bytes — so both
                // checks fail and the test catches it.
                // The part's own `Content-Type` is pinned in the same substring as
                // its disposition, because reqwest emits them on consecutive
                // lines. Asserting `Content-Type` alone would be satisfied by any
                // sibling part; this binds it to *this* field. Expected value: the
                // spec's declared `encoding` type when there is one, otherwise the
                // type inferred from the fixture's extension.
                let expected_mime = field.content_type.clone().unwrap_or_else(|| "text/plain".to_string());
                if let Some(disposition) = &disposition {
                    required_body_substrings.push(format!(
                        "{disposition}; filename=\"{file_name}\"\r\nContent-Type: {expected_mime}"
                    ));
                } else {
                    required_body_substrings.push("filename=\"".to_string());
                    required_body_substrings.push(format!("Content-Type: {expected_mime}"));
                }
                required_body_substrings.push(marker);
            } else {
                if let Some(disposition) = disposition {
                    required_body_substrings.push(disposition);
                }
                // Prefer the IR example's value. Non-string scalars (and nested
                // objects) are rendered as JSON text — a CLI flag only ever
                // carries a string, and asserting the rendered form is what
                // catches a runtime that reformats it.
                let value = match case.body.get(&field.wire_name) {
                    Some(serde_json::Value::String(text)) => text.clone(),
                    Some(serde_json::Value::Null) | None => {
                        format!("wire-fixture-{id}-{}", field.wire_name)
                    }
                    Some(other) => other.to_string()
                };
                body_flag_args.push(format!("--{flag}"));
                body_flag_args.push(value.clone());
                required_body_substrings.push(value);
            }
        }
    } else if command.is_binary {
        if let Some(flag) = &command.binary_flag {
            std::fs::create_dir_all(&fixtures_dir).expect("create binary fixture dir");
            let marker = format!("wire-fixture-{id}-body-bytes");
            let file_path = fixtures_dir.join("body");
            std::fs::write(&file_path, marker.as_bytes()).expect("write binary fixture file");
            body_flag_args.push(format!("--{flag}"));
            body_flag_args.push(file_path.to_string_lossy().into_owned());
            required_body_substrings.push(marker);
        }
    }

    // Public-client login flows (PKCE / device-code) authenticate from a keyring token that
    // `auth login` populates. We can't drive the interactive browser/device login headlessly, so
    // seed a token via the universal `--with-token` paste into an isolated, file-backed keyring
    // (`FERN_CLI_CREDENTIAL_STORE=file` + a temp `HOME` — no OS-keyring prompt, hermetic per test).
    // The request-time provider then injects it as `Authorization: Bearer <token>`, which the
    // business mock asserts below.
    // Every case gets its own hermetic, file-backed credential store — not just
    // login-flow cases. Any CLI whose auth caches a token (client-credentials
    // exchanges cache one) otherwise resolves `auto_store()` against the *ambient*
    // environment: the OS keyring if one is reachable, else a file under the real
    // `HOME`. That makes the test depend on the machine's keyring state and lets the
    // cases in one fixture race each other over a single shared store — green on a
    // developer box, red on a CI runner, and it writes to the developer's real
    // keyring either way. A per-case temp `HOME` plus
    // `FERN_CLI_CREDENTIAL_STORE=file` removes both.
    let auth_home: std::path::PathBuf =
        std::env::temp_dir().join(format!("{}-wire-{id}", manifest.binary_name));
    let _ = std::fs::remove_dir_all(&auth_home);
    std::fs::create_dir_all(&auth_home).expect("create isolated keyring HOME");

    if let Some(setup) = &manifest.login_token_setup {
        use tokio::io::AsyncWriteExt;
        let home = &auth_home;
        let mut login = tokio::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
        login
            .args(["auth", "login", "--with-token", "--scheme", setup.scheme_name.as_str()])
            .env("HOME", &home)
            .env("FERN_CLI_CREDENTIAL_STORE", "file")
            .env("NO_COLOR", "1")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        let mut child = login.spawn().expect("spawn auth login --with-token");
        {
            let mut stdin = child.stdin.take().expect("login stdin");
            stdin
                .write_all(setup.token.as_bytes())
                .await
                .expect("write token to auth login stdin");
        }
        let login_out = child.wait_with_output().await.expect("auth login --with-token output");
        assert!(
            login_out.status.success(),
            "auth login --with-token failed: {}",
            String::from_utf8_lossy(&login_out.stderr)
        );
    }

    let server = MockServer::start().await;

    // OAuth client-credentials CLIs perform a token exchange before every
    // authenticated request, and the token URL honors the --base-url override
    // — so the exchange lands on this mock. Mount a canned token stub (unless
    // the token endpoint IS the case under test) so the exchange succeeds and
    // the request reaches the endpoint we're actually testing. No count
    // assertion: the token may be fetched zero or more times depending on
    // caching.
    if let Some(auth_mock) = &manifest.auth_mock {
        let is_token_case = auth_mock.method.eq_ignore_ascii_case(&case.method)
            && normalize_path(&auth_mock.path) == normalize_path(&case.path);
        if !is_token_case {
            Mock::given(match_method(auth_mock.method.as_str()))
                .and(match_path(normalize_path(&auth_mock.path)))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(auth_mock.response_body.clone().into_bytes(), "application/json"),
                )
                .mount(&server)
                .await;
        }
    }

    let mut template = ResponseTemplate::new(case.response.status);
    if !case.response.body.is_empty() {
        template = template.set_body_raw(case.response.body.clone().into_bytes(), "application/json");
    }
    // Match on method + path AND the scalar query params + auth headers the
    // request must carry. A request that omits or mis-serializes any of them
    // won't match this mock — the CLI then gets no response and the test fails,
    // rather than passing on a path-only match. This gives the same
    // request-shape verification the SDK wire tests get from WireMock stub
    // matching, without a WireMock container.
    let mut mock = Mock::given(match_method(case.method.as_str())).and(match_path(expected_path.clone()));
    for q in &case.query_matchers {
        mock = mock.and(match_query_param(q.name.as_str(), q.value.as_str()));
    }
    for h in &case.header_matchers {
        if let Some(equal_to) = &h.equal_to {
            mock = mock.and(match_header(h.name.as_str(), equal_to.as_str()));
        } else if let Some(pattern) = &h.matches {
            mock = mock.and(match_header_regex(h.name.as_str(), pattern.as_str()));
        }
    }
    // For login-flow schemes, require the exact bearer we seeded — this is what verifies the
    // request-time keyring → `Authorization: Bearer <token>` injection end to end.
    //
    // Only on endpoints that declare auth. An unauthenticated operation is not
    // expected to carry the bearer, so demanding one would make its mock
    // unmatchable and fail the case for a reason unrelated to what it tests. An
    // OAuth token endpoint exposed as a normal command is the canonical case:
    // it sends client credentials in the body and no `Authorization` header.
    if case.requires_auth && !manifest.endpoint_security_auth {
        if let Some(setup) = &manifest.login_token_setup {
            mock = mock.and(match_header("authorization", format!("Bearer {}", setup.token).as_str()));
        }
    }
    // Assert the *request body* the CLI sends, not just its method/path. For an
    // opaque JSON body, require the example fields to be present with the right
    // values (partial match — tolerant of extra transport fields). For an
    // upload, require each part's marker/value to appear in the multipart body.
    // A request with a wrong, missing, or mis-serialized body won't match this
    // mock, so the CLI gets no response and the test fails.
    if command.is_multipart || command.is_binary {
        if command.is_multipart {
            // The parts are worthless under the wrong media type, and reqwest
            // only sets this when the body was actually built as a form.
            mock = mock.and(match_header_regex("content-type", "^multipart/form-data"));
        }
        for substring in &required_body_substrings {
            mock = mock.and(match_body_contains(substring.clone()));
        }
        // Fields the case deliberately omitted must not appear at all.
        for substring in &forbidden_body_substrings {
            mock = mock.and(BodyDoesNotContain(substring.clone()));
        }
    } else if command.has_json_body && !case.body.is_null() {
        mock = mock.and(match_body_partial_json(case.body.clone()));
    }
    // Registered *scoped* rather than mounted: a `MockGuard` exposes the
    // per-mock matched count, which `assert_request_matched` turns into a
    // failure message carrying the command and process output. The guard's own
    // drop-time check stays as a backstop, and it no-ops while panicking, so a
    // failed assertion below surfaces once and cleanly.
    let mock_guard = server
        .register_as_scoped(mock.respond_with(template).expect(1).named(format!("case {id}")))
        .await;

    let mut args: Vec<String> = command.chain.clone();
    args.push("--base-url".to_string());
    args.push(server.uri());
    args.push("--no-pager".to_string());
    if !case.params.is_empty() {
        // The CLI reads path params off the baked spec by their wire name, which
        // can differ from the manifest's (IR-renamed) name — remap those keys so
        // a renamed path param isn't reported as a missing required parameter.
        let wire_names = path_param_wire_names(&case.path, &command.path);
        let cli_params: serde_json::Map<String, serde_json::Value> = case
            .params
            .iter()
            .map(|(key, value)| {
                let cli_key = wire_names.get(key).cloned().unwrap_or_else(|| key.clone());
                (cli_key, value.clone())
            })
            .collect();
        args.push("--params".to_string());
        args.push(serde_json::to_string(&cli_params).expect("params serialize"));
    }
    // Feed the request body the way the endpoint expects it: per-field upload
    // flags for binary/multipart (planned above), else the opaque --json body.
    if command.is_multipart || command.is_binary {
        args.extend(body_flag_args.iter().cloned());
    } else if command.has_json_body && !case.body.is_null() {
        args.push("--json".to_string());
        args.push(serde_json::to_string(&case.body).expect("body serialize"));
    }

    let mut cmd = tokio::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(&args);
    // Dummy credentials so auth-gated endpoints don't bail on a missing secret.
    for var in &manifest.auth_env_vars {
        cmd.env(&var.name, &var.value);
    }
    cmd.env("NO_COLOR", "1");
    // The per-case isolated store created above: hermetic for every auth style, and
    // the same one a login-flow token was seeded into so the request-time provider
    // resolves the bearer we pasted.
    cmd.env("HOME", &auth_home);
    cmd.env("FERN_CLI_CREDENTIAL_STORE", "file");

    let output = cmd
        .output()
        .await
        .unwrap_or_else(|e| panic!("failed to spawn generated CLI binary: {e}"));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Best-effort cleanup of the per-case fixture dir (ignore errors — a leaked
    // temp dir must never fail a test).
    let _ = std::fs::remove_dir_all(&fixtures_dir);

    if case.expect_error {
        // Negative twin: the mock served a non-2xx with a non-empty JSON body,
        // and `Mock::expect(1)` still requires exactly one correctly-matched
        // request. The CLI must surface the error — a non-zero exit — rather
        // than deserialize the error body and report it as success (exit 0).
        assert!(
            !output.status.success(),
            "expected CLI to fail on a {} response, but it exited 0
  command: {} {}
  stdout: {stdout}
  stderr: {stderr}",
            case.response.status,
            manifest.binary_name,
            args.join(" ")
        );
        assert!(
            !stderr.trim().is_empty(),
            "expected an error message on stderr for {id} (a {} response), got empty stderr",
            case.response.status
        );
        // A non-zero exit alone proves nothing: the CLI must have failed
        // *because of the mocked response*, not before it ever made the call.
        // Without this, a bad command line, a rejected body, or a startup panic
        // all pass this branch silently.
        assert_request_matched(
            id,
            &mock_guard,
            &server,
            &manifest.binary_name,
            &args,
            output.status.code(),
            &stdout,
            &stderr,
            &case.method,
            &expected_path,
            &case.spec_filled_body_properties,
        )
        .await;
        return;
    }

    // Request shape first, process outcome second. When a matcher rejects the
    // request the mock server answers 404, the CLI faithfully reports that
    // 404, and it exits non-zero — so checking the exit code first would blame
    // the CLI for a mock that never matched, and report "exited 1" with no hint
    // that the request itself was the problem. Asserting the match first names
    // the root cause and shows the request next to what was expected; a genuine
    // CLI failure still falls through to the exit-code assertion below.
    assert_request_matched(
        id,
        &mock_guard,
        &server,
        &manifest.binary_name,
        &args,
        output.status.code(),
        &stdout,
        &stderr,
        &case.method,
        &expected_path,
        &case.spec_filled_body_properties,
    )
    .await;

    // Primary assertion (mirrors the SDK wire tests: the call succeeds, and
    // `Mock::expect(1)` verifies exactly one matching request).
    assert!(
        output.status.success(),
        "CLI exited with {:?}
  command: {} {}
  stdout: {stdout}
  stderr: {stderr}",
        output.status.code(),
        manifest.binary_name,
        args.join(" ")
    );

    // Rendering check. For a non-streaming endpoint whose stdout parses to the
    // same JSON kind as the mocked body, require a byte-exact render — the
    // strongest signal, and what the common case exercises. Streaming/NDJSON
    // responses re-shape the payload (chunks collected into an array), so there
    // we fall back to asserting the call produced output.
    if case.response.body.trim().is_empty() {
        assert!(stdout.trim().is_empty(), "expected empty output for {id}, got: {stdout}");
    } else if let Ok(expected) = serde_json::from_str::<serde_json::Value>(&case.response.body) {
        match serde_json::from_str::<serde_json::Value>(stdout.trim()) {
            Ok(actual) if !command.is_streaming && same_json_kind(&actual, &expected) => {
                assert_eq!(actual, expected, "rendered response body mismatch for {id}");
            }
            _ => {
                assert!(!stdout.trim().is_empty(), "expected rendered output for {id}, got empty stdout");
            }
        }
    }
}

#[tokio::test]
async fn wire_list() {
    run_case("list").await;
}

#[tokio::test]
async fn wire_get() {
    run_case("get").await;
}

#[tokio::test]
async fn wire_delete() {
    run_case("delete").await;
}

#[tokio::test]
async fn wire_get_audio() {
    run_case("get_audio").await;
}

#[tokio::test]
async fn wire_download() {
    run_case("download").await;
}

#[tokio::test]
async fn wire_convert() {
    run_case("convert").await;
}

#[tokio::test]
async fn wire_list_1() {
    run_case("list_1").await;
}

#[tokio::test]
async fn wire_delete_1() {
    run_case("delete_1").await;
}

#[tokio::test]
async fn wire_delete_2() {
    run_case("delete_2").await;
}

#[tokio::test]
async fn wire_convert_1() {
    run_case("convert_1").await;
}

#[tokio::test]
async fn wire_convert_with_timestamps() {
    run_case("convert_with_timestamps").await;
}

#[tokio::test]
async fn wire_stream() {
    run_case("stream").await;
}

#[tokio::test]
async fn wire_stream_with_timestamps() {
    run_case("stream_with_timestamps").await;
}

#[tokio::test]
async fn wire_convert_2() {
    run_case("convert_2").await;
}

#[tokio::test]
async fn wire_stream_1() {
    run_case("stream_1").await;
}

#[tokio::test]
async fn wire_stream_with_timestamps_1() {
    run_case("stream_with_timestamps_1").await;
}

#[tokio::test]
async fn wire_convert_with_timestamps_1() {
    run_case("convert_with_timestamps_1").await;
}

#[tokio::test]
async fn wire_convert_3() {
    run_case("convert_3").await;
}

#[tokio::test]
async fn wire_stream_2() {
    run_case("stream_2").await;
}

#[tokio::test]
async fn wire_create_previews() {
    run_case("create_previews").await;
}

#[tokio::test]
async fn wire_create() {
    run_case("create").await;
}

#[tokio::test]
async fn wire_design() {
    run_case("design").await;
}

#[tokio::test]
async fn wire_remix() {
    run_case("remix").await;
}

#[tokio::test]
async fn wire_get_1() {
    run_case("get_1").await;
}

#[tokio::test]
async fn wire_get_all() {
    run_case("get_all").await;
}

#[tokio::test]
async fn wire_get_2() {
    run_case("get_2").await;
}

#[tokio::test]
async fn wire_delete_3() {
    run_case("delete_3").await;
}

#[tokio::test]
async fn wire_update() {
    run_case("update").await;
}

#[tokio::test]
async fn wire_search() {
    run_case("search").await;
}

#[tokio::test]
async fn wire_replicate_to_isolated_environment() {
    run_case("replicate_to_isolated_environment").await;
}

#[tokio::test]
async fn wire_share() {
    run_case("share").await;
}

#[tokio::test]
async fn wire_find_similar_voices() {
    run_case("find_similar_voices").await;
}

#[tokio::test]
async fn wire_get_shared() {
    run_case("get_shared").await;
}

#[tokio::test]
async fn wire_create_podcast() {
    run_case("create_podcast").await;
}

#[tokio::test]
async fn wire_list_2() {
    run_case("list_2").await;
}

#[tokio::test]
async fn wire_create_1() {
    run_case("create_1").await;
}

#[tokio::test]
async fn wire_get_3() {
    run_case("get_3").await;
}

#[tokio::test]
async fn wire_delete_4() {
    run_case("delete_4").await;
}

#[tokio::test]
async fn wire_list_3() {
    run_case("list_3").await;
}

#[tokio::test]
async fn wire_create_2() {
    run_case("create_2").await;
}

#[tokio::test]
async fn wire_get_settings() {
    run_case("get_settings").await;
}

#[tokio::test]
async fn wire_update_1() {
    run_case("update_1").await;
}

#[tokio::test]
async fn wire_update_content_from_url() {
    run_case("update_content_from_url").await;
}

#[tokio::test]
async fn wire_get_4() {
    run_case("get_4").await;
}

#[tokio::test]
async fn wire_create_from_file() {
    run_case("create_from_file").await;
}

#[tokio::test]
async fn wire_create_from_rules() {
    run_case("create_from_rules").await;
}

#[tokio::test]
async fn wire_get_5() {
    run_case("get_5").await;
}

#[tokio::test]
async fn wire_update_2() {
    run_case("update_2").await;
}

#[tokio::test]
async fn wire_download_1() {
    run_case("download_1").await;
}

#[tokio::test]
async fn wire_list_4() {
    run_case("list_4").await;
}

#[tokio::test]
async fn wire_set_third_party_disabling_policy() {
    run_case("set_third_party_disabling_policy").await;
}

#[tokio::test]
async fn wire_list_5() {
    run_case("list_5").await;
}

#[tokio::test]
async fn wire_create_3() {
    run_case("create_3").await;
}

#[tokio::test]
async fn wire_list_6() {
    run_case("list_6").await;
}

#[tokio::test]
async fn wire_create_4() {
    run_case("create_4").await;
}

#[tokio::test]
async fn wire_delete_5() {
    run_case("delete_5").await;
}

#[tokio::test]
async fn wire_update_3() {
    run_case("update_3").await;
}

#[tokio::test]
async fn wire_compose() {
    run_case("compose").await;
}

#[tokio::test]
async fn wire_compose_detailed() {
    run_case("compose_detailed").await;
}

#[tokio::test]
async fn wire_compose_detailed_stream() {
    run_case("compose_detailed_stream").await;
}

#[tokio::test]
async fn wire_stream_3() {
    run_case("stream_3").await;
}

#[tokio::test]
async fn wire_upload() {
    run_case("upload").await;
}

#[tokio::test]
async fn wire_convert_4() {
    run_case("convert_4").await;
}

#[tokio::test]
async fn wire_create_5() {
    run_case("create_5").await;
}

#[tokio::test]
async fn wire_create_6() {
    run_case("create_6").await;
}

#[tokio::test]
async fn wire_get_6() {
    run_case("get_6").await;
}

#[tokio::test]
async fn wire_delete_6() {
    run_case("delete_6").await;
}

#[tokio::test]
async fn wire_update_4() {
    run_case("update_4").await;
}

#[tokio::test]
async fn wire_list_7() {
    run_case("list_7").await;
}

#[tokio::test]
async fn wire_duplicate() {
    run_case("duplicate").await;
}

#[tokio::test]
async fn wire_simulate_conversation() {
    run_case("simulate_conversation").await;
}

#[tokio::test]
async fn wire_simulate_conversation_stream() {
    run_case("simulate_conversation_stream").await;
}

#[tokio::test]
async fn wire_run_tests() {
    run_case("run_tests").await;
}

#[tokio::test]
async fn wire_add_to_knowledge_base() {
    run_case("add_to_knowledge_base").await;
}

#[tokio::test]
async fn wire_rag_index_overview() {
    run_case("rag_index_overview").await;
}

#[tokio::test]
async fn wire_get_document_rag_indexes() {
    run_case("get_document_rag_indexes").await;
}

#[tokio::test]
async fn wire_delete_document_rag_index() {
    run_case("delete_document_rag_index").await;
}

#[tokio::test]
async fn wire_list_8() {
    run_case("list_8").await;
}

#[tokio::test]
async fn wire_create_7() {
    run_case("create_7").await;
}

#[tokio::test]
async fn wire_get_7() {
    run_case("get_7").await;
}

#[tokio::test]
async fn wire_delete_7() {
    run_case("delete_7").await;
}

#[tokio::test]
async fn wire_update_5() {
    run_case("update_5").await;
}

#[tokio::test]
async fn wire_list_9() {
    run_case("list_9").await;
}

#[tokio::test]
async fn wire_create_8() {
    run_case("create_8").await;
}

#[tokio::test]
async fn wire_get_8() {
    run_case("get_8").await;
}

#[tokio::test]
async fn wire_update_6() {
    run_case("update_6").await;
}

#[tokio::test]
async fn wire_list_10() {
    run_case("list_10").await;
}

#[tokio::test]
async fn wire_create_9() {
    run_case("create_9").await;
}

#[tokio::test]
async fn wire_get_9() {
    run_case("get_9").await;
}

#[tokio::test]
async fn wire_delete_8() {
    run_case("delete_8").await;
}

#[tokio::test]
async fn wire_get_signed_url() {
    run_case("get_signed_url").await;
}

#[tokio::test]
async fn wire_get_webrtc_token() {
    run_case("get_webrtc_token").await;
}

#[tokio::test]
async fn wire_list_11() {
    run_case("list_11").await;
}

#[tokio::test]
async fn wire_resolve() {
    run_case("resolve").await;
}

#[tokio::test]
async fn wire_get_10() {
    run_case("get_10").await;
}

#[tokio::test]
async fn wire_delete_9() {
    run_case("delete_9").await;
}

#[tokio::test]
async fn wire_get_summary() {
    run_case("get_summary").await;
}

#[tokio::test]
async fn wire_get_sip_messages() {
    run_case("get_sip_messages").await;
}

#[tokio::test]
async fn wire_outbound_call() {
    run_case("outbound_call").await;
}

#[tokio::test]
async fn wire_register_call() {
    run_case("register_call").await;
}

#[tokio::test]
async fn wire_outbound_call_1() {
    run_case("outbound_call_1").await;
}

#[tokio::test]
async fn wire_outbound_call_2() {
    run_case("outbound_call_2").await;
}

#[tokio::test]
async fn wire_outbound_message() {
    run_case("outbound_message").await;
}

#[tokio::test]
async fn wire_get_11() {
    run_case("get_11").await;
}

#[tokio::test]
async fn wire_get_12() {
    run_case("get_12").await;
}

#[tokio::test]
async fn wire_get_13() {
    run_case("get_13").await;
}

#[tokio::test]
async fn wire_size() {
    run_case("size").await;
}

#[tokio::test]
async fn wire_list_12() {
    run_case("list_12").await;
}

#[tokio::test]
async fn wire_get_or_create_rag_indexes() {
    run_case("get_or_create_rag_indexes").await;
}

#[tokio::test]
async fn wire_search_1() {
    run_case("search_1").await;
}

#[tokio::test]
async fn wire_create_10() {
    run_case("create_10").await;
}

#[tokio::test]
async fn wire_move() {
    run_case("move").await;
}

#[tokio::test]
async fn wire_get_14() {
    run_case("get_14").await;
}

#[tokio::test]
async fn wire_update_7() {
    run_case("update_7").await;
}

#[tokio::test]
async fn wire_delete_10() {
    run_case("delete_10").await;
}

#[tokio::test]
async fn wire_summaries() {
    run_case("summaries").await;
}

#[tokio::test]
async fn wire_list_13() {
    run_case("list_13").await;
}

#[tokio::test]
async fn wire_list_14() {
    run_case("list_14").await;
}

#[tokio::test]
async fn wire_list_15() {
    run_case("list_15").await;
}

#[tokio::test]
async fn wire_create_11() {
    run_case("create_11").await;
}

#[tokio::test]
async fn wire_get_15() {
    run_case("get_15").await;
}

#[tokio::test]
async fn wire_delete_11() {
    run_case("delete_11").await;
}

#[tokio::test]
async fn wire_update_8() {
    run_case("update_8").await;
}

#[tokio::test]
async fn wire_get_sip_messages_1() {
    run_case("get_sip_messages_1").await;
}

#[tokio::test]
async fn wire_calculate() {
    run_case("calculate").await;
}

#[tokio::test]
async fn wire_list_16() {
    run_case("list_16").await;
}

#[tokio::test]
async fn wire_list_17() {
    run_case("list_17").await;
}

#[tokio::test]
async fn wire_create_12() {
    run_case("create_12").await;
}

#[tokio::test]
async fn wire_get_16() {
    run_case("get_16").await;
}

#[tokio::test]
async fn wire_delete_12() {
    run_case("delete_12").await;
}

#[tokio::test]
async fn wire_update_9() {
    run_case("update_9").await;
}

#[tokio::test]
async fn wire_get_dependent_agents() {
    run_case("get_dependent_agents").await;
}

#[tokio::test]
async fn wire_get_17() {
    run_case("get_17").await;
}

#[tokio::test]
async fn wire_update_10() {
    run_case("update_10").await;
}

#[tokio::test]
async fn wire_list_18() {
    run_case("list_18").await;
}

#[tokio::test]
async fn wire_create_13() {
    run_case("create_13").await;
}

#[tokio::test]
async fn wire_get_18() {
    run_case("get_18").await;
}

#[tokio::test]
async fn wire_delete_13() {
    run_case("delete_13").await;
}

#[tokio::test]
async fn wire_update_11() {
    run_case("update_11").await;
}

#[tokio::test]
async fn wire_get_dependencies() {
    run_case("get_dependencies").await;
}

#[tokio::test]
async fn wire_create_14() {
    run_case("create_14").await;
}

#[tokio::test]
async fn wire_list_19() {
    run_case("list_19").await;
}

#[tokio::test]
async fn wire_get_19() {
    run_case("get_19").await;
}

#[tokio::test]
async fn wire_delete_14() {
    run_case("delete_14").await;
}

#[tokio::test]
async fn wire_cancel() {
    run_case("cancel").await;
}

#[tokio::test]
async fn wire_retry() {
    run_case("retry").await;
}

#[tokio::test]
async fn wire_export() {
    run_case("export").await;
}

#[tokio::test]
async fn wire_outbound_call_3() {
    run_case("outbound_call_3").await;
}

#[tokio::test]
async fn wire_list_20() {
    run_case("list_20").await;
}

#[tokio::test]
async fn wire_create_15() {
    run_case("create_15").await;
}

#[tokio::test]
async fn wire_get_20() {
    run_case("get_20").await;
}

#[tokio::test]
async fn wire_delete_15() {
    run_case("delete_15").await;
}

#[tokio::test]
async fn wire_update_12() {
    run_case("update_12").await;
}

#[tokio::test]
async fn wire_get_21() {
    run_case("get_21").await;
}

#[tokio::test]
async fn wire_delete_16() {
    run_case("delete_16").await;
}

#[tokio::test]
async fn wire_update_13() {
    run_case("update_13").await;
}

#[tokio::test]
async fn wire_list_21() {
    run_case("list_21").await;
}

#[tokio::test]
async fn wire_list_22() {
    run_case("list_22").await;
}

#[tokio::test]
async fn wire_create_16() {
    run_case("create_16").await;
}

#[tokio::test]
async fn wire_get_22() {
    run_case("get_22").await;
}

#[tokio::test]
async fn wire_update_14() {
    run_case("update_14").await;
}

#[tokio::test]
async fn wire_preview_merge() {
    run_case("preview_merge").await;
}

#[tokio::test]
async fn wire_merge() {
    run_case("merge").await;
}

#[tokio::test]
async fn wire_preview_rebase() {
    run_case("preview_rebase").await;
}

#[tokio::test]
async fn wire_rebase() {
    run_case("rebase").await;
}

#[tokio::test]
async fn wire_get_23() {
    run_case("get_23").await;
}

#[tokio::test]
async fn wire_create_17() {
    run_case("create_17").await;
}

#[tokio::test]
async fn wire_create_18() {
    run_case("create_18").await;
}

#[tokio::test]
async fn wire_delete_17() {
    run_case("delete_17").await;
}

#[tokio::test]
async fn wire_list_23() {
    run_case("list_23").await;
}

#[tokio::test]
async fn wire_create_19() {
    run_case("create_19").await;
}

#[tokio::test]
async fn wire_compile() {
    run_case("compile").await;
}

#[tokio::test]
async fn wire_get_24() {
    run_case("get_24").await;
}

#[tokio::test]
async fn wire_remove() {
    run_case("remove").await;
}

#[tokio::test]
async fn wire_calculate_1() {
    run_case("calculate_1").await;
}

#[tokio::test]
async fn wire_get_25() {
    run_case("get_25").await;
}

#[tokio::test]
async fn wire_get_26() {
    run_case("get_26").await;
}

#[tokio::test]
async fn wire_create_20() {
    run_case("create_20").await;
}

#[tokio::test]
async fn wire_text_search() {
    run_case("text_search").await;
}

#[tokio::test]
async fn wire_search_2() {
    run_case("search_2").await;
}

#[tokio::test]
async fn wire_assign() {
    run_case("assign").await;
}

#[tokio::test]
async fn wire_unassign() {
    run_case("unassign").await;
}

#[tokio::test]
async fn wire_list_24() {
    run_case("list_24").await;
}

#[tokio::test]
async fn wire_create_21() {
    run_case("create_21").await;
}

#[tokio::test]
async fn wire_get_27() {
    run_case("get_27").await;
}

#[tokio::test]
async fn wire_delete_18() {
    run_case("delete_18").await;
}

#[tokio::test]
async fn wire_update_15() {
    run_case("update_15").await;
}

#[tokio::test]
async fn wire_create_22() {
    run_case("create_22").await;
}

#[tokio::test]
async fn wire_delete_19() {
    run_case("delete_19").await;
}

#[tokio::test]
async fn wire_get_28() {
    run_case("get_28").await;
}

#[tokio::test]
async fn wire_run() {
    run_case("run").await;
}

#[tokio::test]
async fn wire_runevaluation() {
    run_case("runevaluation").await;
}

#[tokio::test]
async fn wire_get_29() {
    run_case("get_29").await;
}

#[tokio::test]
async fn wire_update_16() {
    run_case("update_16").await;
}

#[tokio::test]
async fn wire_create_from_url() {
    run_case("create_from_url").await;
}

#[tokio::test]
async fn wire_create_from_file_1() {
    run_case("create_from_file_1").await;
}

#[tokio::test]
async fn wire_create_from_text() {
    run_case("create_from_text").await;
}

#[tokio::test]
async fn wire_create_folder() {
    run_case("create_folder").await;
}

#[tokio::test]
async fn wire_get_30() {
    run_case("get_30").await;
}

#[tokio::test]
async fn wire_delete_20() {
    run_case("delete_20").await;
}

#[tokio::test]
async fn wire_update_17() {
    run_case("update_17").await;
}

#[tokio::test]
async fn wire_get_agents() {
    run_case("get_agents").await;
}

#[tokio::test]
async fn wire_get_bulk_agents() {
    run_case("get_bulk_agents").await;
}

#[tokio::test]
async fn wire_get_content() {
    run_case("get_content").await;
}

#[tokio::test]
async fn wire_get_source_file_url() {
    run_case("get_source_file_url").await;
}

#[tokio::test]
async fn wire_move_1() {
    run_case("move_1").await;
}

#[tokio::test]
async fn wire_bulk_move() {
    run_case("bulk_move").await;
}

#[tokio::test]
async fn wire_bulk_delete() {
    run_case("bulk_delete").await;
}

#[tokio::test]
async fn wire_list_25() {
    run_case("list_25").await;
}

#[tokio::test]
async fn wire_create_23() {
    run_case("create_23").await;
}

#[tokio::test]
async fn wire_get_31() {
    run_case("get_31").await;
}

#[tokio::test]
async fn wire_cancel_1() {
    run_case("cancel_1").await;
}

#[tokio::test]
async fn wire_update_file() {
    run_case("update_file").await;
}

#[tokio::test]
async fn wire_refresh() {
    run_case("refresh").await;
}

#[tokio::test]
async fn wire_compute_rag_index() {
    run_case("compute_rag_index").await;
}

#[tokio::test]
async fn wire_get_32() {
    run_case("get_32").await;
}

#[tokio::test]
async fn wire_get_33() {
    run_case("get_33").await;
}

#[tokio::test]
async fn wire_list_26() {
    run_case("list_26").await;
}

#[tokio::test]
async fn wire_list_27() {
    run_case("list_27").await;
}

#[tokio::test]
async fn wire_update_18() {
    run_case("update_18").await;
}

#[tokio::test]
async fn wire_create_24() {
    run_case("create_24").await;
}

#[tokio::test]
async fn wire_delete_21() {
    run_case("delete_21").await;
}

#[tokio::test]
async fn wire_create_25() {
    run_case("create_25").await;
}

#[tokio::test]
async fn wire_get_34() {
    run_case("get_34").await;
}

#[tokio::test]
async fn wire_delete_22() {
    run_case("delete_22").await;
}

#[tokio::test]
async fn wire_update_19() {
    run_case("update_19").await;
}

#[tokio::test]
async fn wire_get_35() {
    run_case("get_35").await;
}

#[tokio::test]
async fn wire_delete_23() {
    run_case("delete_23").await;
}

#[tokio::test]
async fn wire_update_20() {
    run_case("update_20").await;
}

#[tokio::test]
async fn wire_create_26() {
    run_case("create_26").await;
}

#[tokio::test]
async fn wire_get_36() {
    run_case("get_36").await;
}

#[tokio::test]
async fn wire_delete_24() {
    run_case("delete_24").await;
}

#[tokio::test]
async fn wire_update_21() {
    run_case("update_21").await;
}

#[tokio::test]
async fn wire_list_28() {
    run_case("list_28").await;
}

#[tokio::test]
async fn wire_get_37() {
    run_case("get_37").await;
}

#[tokio::test]
async fn wire_resubmit() {
    run_case("resubmit").await;
}

#[tokio::test]
async fn wire_get_38() {
    run_case("get_38").await;
}

#[tokio::test]
async fn wire_create_27() {
    run_case("create_27").await;
}

#[tokio::test]
async fn wire_list_29() {
    run_case("list_29").await;
}

#[tokio::test]
async fn wire_create_28() {
    run_case("create_28").await;
}

#[tokio::test]
async fn wire_get_39() {
    run_case("get_39").await;
}

#[tokio::test]
async fn wire_delete_25() {
    run_case("delete_25").await;
}

#[tokio::test]
async fn wire_get_40() {
    run_case("get_40").await;
}

#[tokio::test]
async fn wire_migrate_segments() {
    run_case("migrate_segments").await;
}

#[tokio::test]
async fn wire_transcribe() {
    run_case("transcribe").await;
}

#[tokio::test]
async fn wire_translate() {
    run_case("translate").await;
}

#[tokio::test]
async fn wire_dub() {
    run_case("dub").await;
}

#[tokio::test]
async fn wire_render() {
    run_case("render").await;
}

#[tokio::test]
async fn wire_get_41() {
    run_case("get_41").await;
}

#[tokio::test]
async fn wire_get_transcript_for_dub() {
    run_case("get_transcript_for_dub").await;
}

#[tokio::test]
async fn wire_get_42() {
    run_case("get_42").await;
}

#[tokio::test]
async fn wire_list_30() {
    run_case("list_30").await;
}

#[tokio::test]
async fn wire_create_29() {
    run_case("create_29").await;
}

#[tokio::test]
async fn wire_get_43() {
    run_case("get_43").await;
}

#[tokio::test]
async fn wire_delete_26() {
    run_case("delete_26").await;
}

#[tokio::test]
async fn wire_get_44() {
    run_case("get_44").await;
}

#[tokio::test]
async fn wire_delete_segment() {
    run_case("delete_segment").await;
}

#[tokio::test]
async fn wire_update_segment() {
    run_case("update_segment").await;
}

#[tokio::test]
async fn wire_update_segments() {
    run_case("update_segments").await;
}

#[tokio::test]
async fn wire_create_segment() {
    run_case("create_segment").await;
}

#[tokio::test]
async fn wire_get_45() {
    run_case("get_45").await;
}

#[tokio::test]
async fn wire_update_segment_1() {
    run_case("update_segment_1").await;
}

#[tokio::test]
async fn wire_update_segments_1() {
    run_case("update_segments_1").await;
}

#[tokio::test]
async fn wire_regenerate() {
    run_case("regenerate").await;
}

#[tokio::test]
async fn wire_add() {
    run_case("add").await;
}

#[tokio::test]
async fn wire_update_22() {
    run_case("update_22").await;
}

#[tokio::test]
async fn wire_delete_27() {
    run_case("delete_27").await;
}

#[tokio::test]
async fn wire_update_23() {
    run_case("update_23").await;
}

#[tokio::test]
async fn wire_create_30() {
    run_case("create_30").await;
}

#[tokio::test]
async fn wire_find_similar_voices_1() {
    run_case("find_similar_voices_1").await;
}

#[tokio::test]
async fn wire_create_31() {
    run_case("create_31").await;
}

#[tokio::test]
async fn wire_list_31() {
    run_case("list_31").await;
}

#[tokio::test]
async fn wire_create_32() {
    run_case("create_32").await;
}

#[tokio::test]
async fn wire_get_46() {
    run_case("get_46").await;
}

#[tokio::test]
async fn wire_list_32() {
    run_case("list_32").await;
}

#[tokio::test]
async fn wire_create_33() {
    run_case("create_33").await;
}

#[tokio::test]
async fn wire_get_47() {
    run_case("get_47").await;
}

#[tokio::test]
async fn wire_list_33() {
    run_case("list_33").await;
}

#[tokio::test]
async fn wire_create_34() {
    run_case("create_34").await;
}

#[tokio::test]
async fn wire_get_48() {
    run_case("get_48").await;
}

#[tokio::test]
async fn wire_create_35() {
    run_case("create_35").await;
}

#[tokio::test]
async fn wire_list_34() {
    run_case("list_34").await;
}

#[tokio::test]
async fn wire_create_36() {
    run_case("create_36").await;
}

#[tokio::test]
async fn wire_get_49() {
    run_case("get_49").await;
}

#[tokio::test]
async fn wire_delete_28() {
    run_case("delete_28").await;
}

#[tokio::test]
async fn wire_update_24() {
    run_case("update_24").await;
}

#[tokio::test]
async fn wire_list_35() {
    run_case("list_35").await;
}

#[tokio::test]
async fn wire_create_37() {
    run_case("create_37").await;
}

#[tokio::test]
async fn wire_get_50() {
    run_case("get_50").await;
}

#[tokio::test]
async fn wire_update_25() {
    run_case("update_25").await;
}

#[tokio::test]
async fn wire_submit() {
    run_case("submit").await;
}

#[tokio::test]
async fn wire_register() {
    run_case("register").await;
}

#[tokio::test]
async fn wire_get_51() {
    run_case("get_51").await;
}

#[tokio::test]
async fn wire_upsert() {
    run_case("upsert").await;
}

#[tokio::test]
async fn wire_remove_1() {
    run_case("remove_1").await;
}

#[tokio::test]
async fn wire_list_36() {
    run_case("list_36").await;
}

#[tokio::test]
async fn wire_list_37() {
    run_case("list_37").await;
}

#[tokio::test]
async fn wire_set() {
    run_case("set").await;
}

#[tokio::test]
async fn wire_add_1() {
    run_case("add_1").await;
}

#[tokio::test]
async fn wire_remove_2() {
    run_case("remove_2").await;
}

#[tokio::test]
async fn wire_list_38() {
    run_case("list_38").await;
}

#[tokio::test]
async fn wire_create_38() {
    run_case("create_38").await;
}

#[tokio::test]
async fn wire_delete_29() {
    run_case("delete_29").await;
}

#[tokio::test]
async fn wire_update_26() {
    run_case("update_26").await;
}

#[tokio::test]
async fn wire_get_52() {
    run_case("get_52").await;
}

#[tokio::test]
async fn wire_delete_30() {
    run_case("delete_30").await;
}

#[tokio::test]
async fn wire_list_39() {
    run_case("list_39").await;
}

#[tokio::test]
async fn wire_create_39() {
    run_case("create_39").await;
}

#[tokio::test]
async fn wire_get_53() {
    run_case("get_53").await;
}

#[tokio::test]
async fn wire_update_27() {
    run_case("update_27").await;
}

#[tokio::test]
async fn wire_delete_31() {
    run_case("delete_31").await;
}

#[tokio::test]
async fn wire_convert_5() {
    run_case("convert_5").await;
}

#[tokio::test]
async fn wire_get_muted_tracks() {
    run_case("get_muted_tracks").await;
}

#[tokio::test]
async fn wire_create_40() {
    run_case("create_40").await;
}

#[tokio::test]
async fn wire_update_28() {
    run_case("update_28").await;
}

#[tokio::test]
async fn wire_list_40() {
    run_case("list_40").await;
}

#[tokio::test]
async fn wire_get_54() {
    run_case("get_54").await;
}

#[tokio::test]
async fn wire_stream_4() {
    run_case("stream_4").await;
}

#[tokio::test]
async fn wire_stream_archive() {
    run_case("stream_archive").await;
}

#[tokio::test]
async fn wire_list_41() {
    run_case("list_41").await;
}

#[tokio::test]
async fn wire_create_41() {
    run_case("create_41").await;
}

#[tokio::test]
async fn wire_get_55() {
    run_case("get_55").await;
}

#[tokio::test]
async fn wire_update_29() {
    run_case("update_29").await;
}

#[tokio::test]
async fn wire_delete_32() {
    run_case("delete_32").await;
}

#[tokio::test]
async fn wire_convert_6() {
    run_case("convert_6").await;
}

#[tokio::test]
async fn wire_list_42() {
    run_case("list_42").await;
}

#[tokio::test]
async fn wire_get_56() {
    run_case("get_56").await;
}

#[tokio::test]
async fn wire_stream_5() {
    run_case("stream_5").await;
}

#[tokio::test]
async fn wire_stream_6() {
    run_case("stream_6").await;
}

#[tokio::test]
async fn wire_create_42() {
    run_case("create_42").await;
}

#[tokio::test]
async fn wire_get_57() {
    run_case("get_57").await;
}

#[tokio::test]
async fn wire_get_default() {
    run_case("get_default").await;
}

#[tokio::test]
async fn wire_get_58() {
    run_case("get_58").await;
}

#[tokio::test]
async fn wire_update_30() {
    run_case("update_30").await;
}

#[tokio::test]
async fn wire_get_59() {
    run_case("get_59").await;
}

#[tokio::test]
async fn wire_create_43() {
    run_case("create_43").await;
}

#[tokio::test]
async fn wire_create_44() {
    run_case("create_44").await;
}

#[tokio::test]
async fn wire_update_31() {
    run_case("update_31").await;
}

#[tokio::test]
async fn wire_train() {
    run_case("train").await;
}

#[tokio::test]
async fn wire_create_45() {
    run_case("create_45").await;
}

#[tokio::test]
async fn wire_update_32() {
    run_case("update_32").await;
}

#[tokio::test]
async fn wire_delete_33() {
    run_case("delete_33").await;
}

#[tokio::test]
async fn wire_request() {
    run_case("request").await;
}

#[tokio::test]
async fn wire_get_60() {
    run_case("get_60").await;
}

#[tokio::test]
async fn wire_get_61() {
    run_case("get_61").await;
}

#[tokio::test]
async fn wire_get_62() {
    run_case("get_62").await;
}

#[tokio::test]
async fn wire_separate() {
    run_case("separate").await;
}

#[tokio::test]
async fn wire_get_63() {
    run_case("get_63").await;
}

#[tokio::test]
async fn wire_get_64() {
    run_case("get_64").await;
}

#[tokio::test]
async fn wire_verify() {
    run_case("verify").await;
}

#[tokio::test]
async fn wire_get_65() {
    run_case("get_65").await;
}

#[tokio::test]
async fn wire_list_43() {
    run_case("list_43").await;
}

#[tokio::test]
async fn wire_list_44() {
    run_case("list_44").await;
}

#[tokio::test]
async fn wire_create_46() {
    run_case("create_46").await;
}

#[tokio::test]
async fn wire_delete_34() {
    run_case("delete_34").await;
}

#[tokio::test]
async fn wire_update_33() {
    run_case("update_33").await;
}

#[tokio::test]
async fn wire_list_45() {
    run_case("list_45").await;
}

#[tokio::test]
async fn wire_search_3() {
    run_case("search_3").await;
}

#[tokio::test]
async fn wire_create_47() {
    run_case("create_47").await;
}

#[tokio::test]
async fn wire_create_batch() {
    run_case("create_batch").await;
}

#[tokio::test]
async fn wire_delete_35() {
    run_case("delete_35").await;
}

#[tokio::test]
async fn wire_list_46() {
    run_case("list_46").await;
}

#[tokio::test]
async fn wire_update_34() {
    run_case("update_34").await;
}

#[tokio::test]
async fn wire_get_66() {
    run_case("get_66").await;
}

#[tokio::test]
async fn wire_share_1() {
    run_case("share_1").await;
}

#[tokio::test]
async fn wire_unshare() {
    run_case("unshare").await;
}

#[tokio::test]
async fn wire_get_usage_by_product_over_time() {
    run_case("get_usage_by_product_over_time").await;
}

#[tokio::test]
async fn wire_get_67() {
    run_case("get_67").await;
}

#[tokio::test]
async fn wire_remove_3() {
    run_case("remove_3").await;
}

#[tokio::test]
async fn wire_add_2() {
    run_case("add_2").await;
}

#[tokio::test]
async fn wire_disable() {
    run_case("disable").await;
}

#[tokio::test]
async fn wire_list_error() {
    run_case("list_error").await;
}

#[tokio::test]
async fn wire_get_error() {
    run_case("get_error").await;
}

#[tokio::test]
async fn wire_delete_error() {
    run_case("delete_error").await;
}

#[tokio::test]
async fn wire_get_audio_error() {
    run_case("get_audio_error").await;
}

#[tokio::test]
async fn wire_download_error() {
    run_case("download_error").await;
}

#[tokio::test]
async fn wire_convert_error() {
    run_case("convert_error").await;
}

#[tokio::test]
async fn wire_list_1_error() {
    run_case("list_1_error").await;
}

#[tokio::test]
async fn wire_delete_1_error() {
    run_case("delete_1_error").await;
}

#[tokio::test]
async fn wire_delete_2_error() {
    run_case("delete_2_error").await;
}

#[tokio::test]
async fn wire_convert_1_error() {
    run_case("convert_1_error").await;
}

#[tokio::test]
async fn wire_convert_with_timestamps_error() {
    run_case("convert_with_timestamps_error").await;
}

#[tokio::test]
async fn wire_stream_error() {
    run_case("stream_error").await;
}

#[tokio::test]
async fn wire_stream_with_timestamps_error() {
    run_case("stream_with_timestamps_error").await;
}

#[tokio::test]
async fn wire_convert_2_error() {
    run_case("convert_2_error").await;
}

#[tokio::test]
async fn wire_stream_1_error() {
    run_case("stream_1_error").await;
}

#[tokio::test]
async fn wire_stream_with_timestamps_1_error() {
    run_case("stream_with_timestamps_1_error").await;
}

#[tokio::test]
async fn wire_convert_with_timestamps_1_error() {
    run_case("convert_with_timestamps_1_error").await;
}

#[tokio::test]
async fn wire_convert_3_error() {
    run_case("convert_3_error").await;
}

#[tokio::test]
async fn wire_stream_2_error() {
    run_case("stream_2_error").await;
}

#[tokio::test]
async fn wire_create_previews_error() {
    run_case("create_previews_error").await;
}

#[tokio::test]
async fn wire_create_error() {
    run_case("create_error").await;
}

#[tokio::test]
async fn wire_design_error() {
    run_case("design_error").await;
}

#[tokio::test]
async fn wire_remix_error() {
    run_case("remix_error").await;
}

#[tokio::test]
async fn wire_get_1_error() {
    run_case("get_1_error").await;
}

#[tokio::test]
async fn wire_get_all_error() {
    run_case("get_all_error").await;
}

#[tokio::test]
async fn wire_get_2_error() {
    run_case("get_2_error").await;
}

#[tokio::test]
async fn wire_delete_3_error() {
    run_case("delete_3_error").await;
}

#[tokio::test]
async fn wire_update_error() {
    run_case("update_error").await;
}

#[tokio::test]
async fn wire_search_error() {
    run_case("search_error").await;
}

#[tokio::test]
async fn wire_replicate_to_isolated_environment_error() {
    run_case("replicate_to_isolated_environment_error").await;
}

#[tokio::test]
async fn wire_share_error() {
    run_case("share_error").await;
}

#[tokio::test]
async fn wire_find_similar_voices_error() {
    run_case("find_similar_voices_error").await;
}

#[tokio::test]
async fn wire_get_shared_error() {
    run_case("get_shared_error").await;
}

#[tokio::test]
async fn wire_create_podcast_error() {
    run_case("create_podcast_error").await;
}

#[tokio::test]
async fn wire_list_2_error() {
    run_case("list_2_error").await;
}

#[tokio::test]
async fn wire_create_1_error() {
    run_case("create_1_error").await;
}

#[tokio::test]
async fn wire_get_3_error() {
    run_case("get_3_error").await;
}

#[tokio::test]
async fn wire_delete_4_error() {
    run_case("delete_4_error").await;
}

#[tokio::test]
async fn wire_list_3_error() {
    run_case("list_3_error").await;
}

#[tokio::test]
async fn wire_create_2_error() {
    run_case("create_2_error").await;
}

#[tokio::test]
async fn wire_get_settings_error() {
    run_case("get_settings_error").await;
}

#[tokio::test]
async fn wire_update_1_error() {
    run_case("update_1_error").await;
}

#[tokio::test]
async fn wire_update_content_from_url_error() {
    run_case("update_content_from_url_error").await;
}

#[tokio::test]
async fn wire_get_4_error() {
    run_case("get_4_error").await;
}

#[tokio::test]
async fn wire_create_from_file_error() {
    run_case("create_from_file_error").await;
}

#[tokio::test]
async fn wire_create_from_rules_error() {
    run_case("create_from_rules_error").await;
}

#[tokio::test]
async fn wire_get_5_error() {
    run_case("get_5_error").await;
}

#[tokio::test]
async fn wire_update_2_error() {
    run_case("update_2_error").await;
}

#[tokio::test]
async fn wire_download_1_error() {
    run_case("download_1_error").await;
}

#[tokio::test]
async fn wire_list_4_error() {
    run_case("list_4_error").await;
}

#[tokio::test]
async fn wire_set_third_party_disabling_policy_error() {
    run_case("set_third_party_disabling_policy_error").await;
}

#[tokio::test]
async fn wire_list_5_error() {
    run_case("list_5_error").await;
}

#[tokio::test]
async fn wire_create_3_error() {
    run_case("create_3_error").await;
}

#[tokio::test]
async fn wire_list_6_error() {
    run_case("list_6_error").await;
}

#[tokio::test]
async fn wire_create_4_error() {
    run_case("create_4_error").await;
}

#[tokio::test]
async fn wire_delete_5_error() {
    run_case("delete_5_error").await;
}

#[tokio::test]
async fn wire_update_3_error() {
    run_case("update_3_error").await;
}

#[tokio::test]
async fn wire_compose_error() {
    run_case("compose_error").await;
}

#[tokio::test]
async fn wire_compose_detailed_error() {
    run_case("compose_detailed_error").await;
}

#[tokio::test]
async fn wire_compose_detailed_stream_error() {
    run_case("compose_detailed_stream_error").await;
}

#[tokio::test]
async fn wire_stream_3_error() {
    run_case("stream_3_error").await;
}

#[tokio::test]
async fn wire_upload_error() {
    run_case("upload_error").await;
}

#[tokio::test]
async fn wire_convert_4_error() {
    run_case("convert_4_error").await;
}

#[tokio::test]
async fn wire_create_5_error() {
    run_case("create_5_error").await;
}

#[tokio::test]
async fn wire_create_6_error() {
    run_case("create_6_error").await;
}

#[tokio::test]
async fn wire_get_6_error() {
    run_case("get_6_error").await;
}

#[tokio::test]
async fn wire_delete_6_error() {
    run_case("delete_6_error").await;
}

#[tokio::test]
async fn wire_update_4_error() {
    run_case("update_4_error").await;
}

#[tokio::test]
async fn wire_list_7_error() {
    run_case("list_7_error").await;
}

#[tokio::test]
async fn wire_duplicate_error() {
    run_case("duplicate_error").await;
}

#[tokio::test]
async fn wire_simulate_conversation_error() {
    run_case("simulate_conversation_error").await;
}

#[tokio::test]
async fn wire_simulate_conversation_stream_error() {
    run_case("simulate_conversation_stream_error").await;
}

#[tokio::test]
async fn wire_run_tests_error() {
    run_case("run_tests_error").await;
}

#[tokio::test]
async fn wire_add_to_knowledge_base_error() {
    run_case("add_to_knowledge_base_error").await;
}

#[tokio::test]
async fn wire_rag_index_overview_error() {
    run_case("rag_index_overview_error").await;
}

#[tokio::test]
async fn wire_get_document_rag_indexes_error() {
    run_case("get_document_rag_indexes_error").await;
}

#[tokio::test]
async fn wire_delete_document_rag_index_error() {
    run_case("delete_document_rag_index_error").await;
}

#[tokio::test]
async fn wire_list_8_error() {
    run_case("list_8_error").await;
}

#[tokio::test]
async fn wire_create_7_error() {
    run_case("create_7_error").await;
}

#[tokio::test]
async fn wire_get_7_error() {
    run_case("get_7_error").await;
}

#[tokio::test]
async fn wire_delete_7_error() {
    run_case("delete_7_error").await;
}

#[tokio::test]
async fn wire_update_5_error() {
    run_case("update_5_error").await;
}

#[tokio::test]
async fn wire_list_9_error() {
    run_case("list_9_error").await;
}

#[tokio::test]
async fn wire_create_8_error() {
    run_case("create_8_error").await;
}

#[tokio::test]
async fn wire_get_8_error() {
    run_case("get_8_error").await;
}

#[tokio::test]
async fn wire_update_6_error() {
    run_case("update_6_error").await;
}

#[tokio::test]
async fn wire_list_10_error() {
    run_case("list_10_error").await;
}

#[tokio::test]
async fn wire_create_9_error() {
    run_case("create_9_error").await;
}

#[tokio::test]
async fn wire_get_9_error() {
    run_case("get_9_error").await;
}

#[tokio::test]
async fn wire_delete_8_error() {
    run_case("delete_8_error").await;
}

#[tokio::test]
async fn wire_get_signed_url_error() {
    run_case("get_signed_url_error").await;
}

#[tokio::test]
async fn wire_get_webrtc_token_error() {
    run_case("get_webrtc_token_error").await;
}

#[tokio::test]
async fn wire_list_11_error() {
    run_case("list_11_error").await;
}

#[tokio::test]
async fn wire_resolve_error() {
    run_case("resolve_error").await;
}

#[tokio::test]
async fn wire_get_10_error() {
    run_case("get_10_error").await;
}

#[tokio::test]
async fn wire_delete_9_error() {
    run_case("delete_9_error").await;
}

#[tokio::test]
async fn wire_get_summary_error() {
    run_case("get_summary_error").await;
}

#[tokio::test]
async fn wire_get_sip_messages_error() {
    run_case("get_sip_messages_error").await;
}

#[tokio::test]
async fn wire_outbound_call_error() {
    run_case("outbound_call_error").await;
}

#[tokio::test]
async fn wire_register_call_error() {
    run_case("register_call_error").await;
}

#[tokio::test]
async fn wire_outbound_call_1_error() {
    run_case("outbound_call_1_error").await;
}

#[tokio::test]
async fn wire_outbound_call_2_error() {
    run_case("outbound_call_2_error").await;
}

#[tokio::test]
async fn wire_outbound_message_error() {
    run_case("outbound_message_error").await;
}

#[tokio::test]
async fn wire_get_11_error() {
    run_case("get_11_error").await;
}

#[tokio::test]
async fn wire_get_12_error() {
    run_case("get_12_error").await;
}

#[tokio::test]
async fn wire_get_13_error() {
    run_case("get_13_error").await;
}

#[tokio::test]
async fn wire_size_error() {
    run_case("size_error").await;
}

#[tokio::test]
async fn wire_list_12_error() {
    run_case("list_12_error").await;
}

#[tokio::test]
async fn wire_get_or_create_rag_indexes_error() {
    run_case("get_or_create_rag_indexes_error").await;
}

#[tokio::test]
async fn wire_search_1_error() {
    run_case("search_1_error").await;
}

#[tokio::test]
async fn wire_create_10_error() {
    run_case("create_10_error").await;
}

#[tokio::test]
async fn wire_move_error() {
    run_case("move_error").await;
}

#[tokio::test]
async fn wire_get_14_error() {
    run_case("get_14_error").await;
}

#[tokio::test]
async fn wire_update_7_error() {
    run_case("update_7_error").await;
}

#[tokio::test]
async fn wire_delete_10_error() {
    run_case("delete_10_error").await;
}

#[tokio::test]
async fn wire_summaries_error() {
    run_case("summaries_error").await;
}

#[tokio::test]
async fn wire_list_13_error() {
    run_case("list_13_error").await;
}

#[tokio::test]
async fn wire_list_14_error() {
    run_case("list_14_error").await;
}

#[tokio::test]
async fn wire_list_15_error() {
    run_case("list_15_error").await;
}

#[tokio::test]
async fn wire_create_11_error() {
    run_case("create_11_error").await;
}

#[tokio::test]
async fn wire_get_15_error() {
    run_case("get_15_error").await;
}

#[tokio::test]
async fn wire_delete_11_error() {
    run_case("delete_11_error").await;
}

#[tokio::test]
async fn wire_update_8_error() {
    run_case("update_8_error").await;
}

#[tokio::test]
async fn wire_get_sip_messages_1_error() {
    run_case("get_sip_messages_1_error").await;
}

#[tokio::test]
async fn wire_calculate_error() {
    run_case("calculate_error").await;
}

#[tokio::test]
async fn wire_list_16_error() {
    run_case("list_16_error").await;
}

#[tokio::test]
async fn wire_list_17_error() {
    run_case("list_17_error").await;
}

#[tokio::test]
async fn wire_create_12_error() {
    run_case("create_12_error").await;
}

#[tokio::test]
async fn wire_get_16_error() {
    run_case("get_16_error").await;
}

#[tokio::test]
async fn wire_delete_12_error() {
    run_case("delete_12_error").await;
}

#[tokio::test]
async fn wire_update_9_error() {
    run_case("update_9_error").await;
}

#[tokio::test]
async fn wire_get_dependent_agents_error() {
    run_case("get_dependent_agents_error").await;
}

#[tokio::test]
async fn wire_get_17_error() {
    run_case("get_17_error").await;
}

#[tokio::test]
async fn wire_update_10_error() {
    run_case("update_10_error").await;
}

#[tokio::test]
async fn wire_list_18_error() {
    run_case("list_18_error").await;
}

#[tokio::test]
async fn wire_create_13_error() {
    run_case("create_13_error").await;
}

#[tokio::test]
async fn wire_get_18_error() {
    run_case("get_18_error").await;
}

#[tokio::test]
async fn wire_delete_13_error() {
    run_case("delete_13_error").await;
}

#[tokio::test]
async fn wire_update_11_error() {
    run_case("update_11_error").await;
}

#[tokio::test]
async fn wire_get_dependencies_error() {
    run_case("get_dependencies_error").await;
}

#[tokio::test]
async fn wire_create_14_error() {
    run_case("create_14_error").await;
}

#[tokio::test]
async fn wire_list_19_error() {
    run_case("list_19_error").await;
}

#[tokio::test]
async fn wire_get_19_error() {
    run_case("get_19_error").await;
}

#[tokio::test]
async fn wire_delete_14_error() {
    run_case("delete_14_error").await;
}

#[tokio::test]
async fn wire_cancel_error() {
    run_case("cancel_error").await;
}

#[tokio::test]
async fn wire_retry_error() {
    run_case("retry_error").await;
}

#[tokio::test]
async fn wire_export_error() {
    run_case("export_error").await;
}

#[tokio::test]
async fn wire_outbound_call_3_error() {
    run_case("outbound_call_3_error").await;
}

#[tokio::test]
async fn wire_list_20_error() {
    run_case("list_20_error").await;
}

#[tokio::test]
async fn wire_create_15_error() {
    run_case("create_15_error").await;
}

#[tokio::test]
async fn wire_get_20_error() {
    run_case("get_20_error").await;
}

#[tokio::test]
async fn wire_delete_15_error() {
    run_case("delete_15_error").await;
}

#[tokio::test]
async fn wire_update_12_error() {
    run_case("update_12_error").await;
}

#[tokio::test]
async fn wire_get_21_error() {
    run_case("get_21_error").await;
}

#[tokio::test]
async fn wire_delete_16_error() {
    run_case("delete_16_error").await;
}

#[tokio::test]
async fn wire_update_13_error() {
    run_case("update_13_error").await;
}

#[tokio::test]
async fn wire_list_21_error() {
    run_case("list_21_error").await;
}

#[tokio::test]
async fn wire_list_22_error() {
    run_case("list_22_error").await;
}

#[tokio::test]
async fn wire_create_16_error() {
    run_case("create_16_error").await;
}

#[tokio::test]
async fn wire_get_22_error() {
    run_case("get_22_error").await;
}

#[tokio::test]
async fn wire_update_14_error() {
    run_case("update_14_error").await;
}

#[tokio::test]
async fn wire_preview_merge_error() {
    run_case("preview_merge_error").await;
}

#[tokio::test]
async fn wire_merge_error() {
    run_case("merge_error").await;
}

#[tokio::test]
async fn wire_preview_rebase_error() {
    run_case("preview_rebase_error").await;
}

#[tokio::test]
async fn wire_rebase_error() {
    run_case("rebase_error").await;
}

#[tokio::test]
async fn wire_get_23_error() {
    run_case("get_23_error").await;
}

#[tokio::test]
async fn wire_create_17_error() {
    run_case("create_17_error").await;
}

#[tokio::test]
async fn wire_create_18_error() {
    run_case("create_18_error").await;
}

#[tokio::test]
async fn wire_delete_17_error() {
    run_case("delete_17_error").await;
}

#[tokio::test]
async fn wire_list_23_error() {
    run_case("list_23_error").await;
}

#[tokio::test]
async fn wire_create_19_error() {
    run_case("create_19_error").await;
}

#[tokio::test]
async fn wire_compile_error() {
    run_case("compile_error").await;
}

#[tokio::test]
async fn wire_get_24_error() {
    run_case("get_24_error").await;
}

#[tokio::test]
async fn wire_remove_error() {
    run_case("remove_error").await;
}

#[tokio::test]
async fn wire_calculate_1_error() {
    run_case("calculate_1_error").await;
}

#[tokio::test]
async fn wire_get_25_error() {
    run_case("get_25_error").await;
}

#[tokio::test]
async fn wire_get_26_error() {
    run_case("get_26_error").await;
}

#[tokio::test]
async fn wire_create_20_error() {
    run_case("create_20_error").await;
}

#[tokio::test]
async fn wire_text_search_error() {
    run_case("text_search_error").await;
}

#[tokio::test]
async fn wire_search_2_error() {
    run_case("search_2_error").await;
}

#[tokio::test]
async fn wire_assign_error() {
    run_case("assign_error").await;
}

#[tokio::test]
async fn wire_unassign_error() {
    run_case("unassign_error").await;
}

#[tokio::test]
async fn wire_list_24_error() {
    run_case("list_24_error").await;
}

#[tokio::test]
async fn wire_create_21_error() {
    run_case("create_21_error").await;
}

#[tokio::test]
async fn wire_get_27_error() {
    run_case("get_27_error").await;
}

#[tokio::test]
async fn wire_delete_18_error() {
    run_case("delete_18_error").await;
}

#[tokio::test]
async fn wire_update_15_error() {
    run_case("update_15_error").await;
}

#[tokio::test]
async fn wire_create_22_error() {
    run_case("create_22_error").await;
}

#[tokio::test]
async fn wire_delete_19_error() {
    run_case("delete_19_error").await;
}

#[tokio::test]
async fn wire_get_28_error() {
    run_case("get_28_error").await;
}

#[tokio::test]
async fn wire_run_error() {
    run_case("run_error").await;
}

#[tokio::test]
async fn wire_runevaluation_error() {
    run_case("runevaluation_error").await;
}

#[tokio::test]
async fn wire_get_29_error() {
    run_case("get_29_error").await;
}

#[tokio::test]
async fn wire_update_16_error() {
    run_case("update_16_error").await;
}

#[tokio::test]
async fn wire_create_from_url_error() {
    run_case("create_from_url_error").await;
}

#[tokio::test]
async fn wire_create_from_file_1_error() {
    run_case("create_from_file_1_error").await;
}

#[tokio::test]
async fn wire_create_from_text_error() {
    run_case("create_from_text_error").await;
}

#[tokio::test]
async fn wire_create_folder_error() {
    run_case("create_folder_error").await;
}

#[tokio::test]
async fn wire_get_30_error() {
    run_case("get_30_error").await;
}

#[tokio::test]
async fn wire_delete_20_error() {
    run_case("delete_20_error").await;
}

#[tokio::test]
async fn wire_update_17_error() {
    run_case("update_17_error").await;
}

#[tokio::test]
async fn wire_get_agents_error() {
    run_case("get_agents_error").await;
}

#[tokio::test]
async fn wire_get_bulk_agents_error() {
    run_case("get_bulk_agents_error").await;
}

#[tokio::test]
async fn wire_get_content_error() {
    run_case("get_content_error").await;
}

#[tokio::test]
async fn wire_get_source_file_url_error() {
    run_case("get_source_file_url_error").await;
}

#[tokio::test]
async fn wire_move_1_error() {
    run_case("move_1_error").await;
}

#[tokio::test]
async fn wire_bulk_move_error() {
    run_case("bulk_move_error").await;
}

#[tokio::test]
async fn wire_bulk_delete_error() {
    run_case("bulk_delete_error").await;
}

#[tokio::test]
async fn wire_list_25_error() {
    run_case("list_25_error").await;
}

#[tokio::test]
async fn wire_create_23_error() {
    run_case("create_23_error").await;
}

#[tokio::test]
async fn wire_get_31_error() {
    run_case("get_31_error").await;
}

#[tokio::test]
async fn wire_cancel_1_error() {
    run_case("cancel_1_error").await;
}

#[tokio::test]
async fn wire_update_file_error() {
    run_case("update_file_error").await;
}

#[tokio::test]
async fn wire_refresh_error() {
    run_case("refresh_error").await;
}

#[tokio::test]
async fn wire_compute_rag_index_error() {
    run_case("compute_rag_index_error").await;
}

#[tokio::test]
async fn wire_get_32_error() {
    run_case("get_32_error").await;
}

#[tokio::test]
async fn wire_get_33_error() {
    run_case("get_33_error").await;
}

#[tokio::test]
async fn wire_list_26_error() {
    run_case("list_26_error").await;
}

#[tokio::test]
async fn wire_list_27_error() {
    run_case("list_27_error").await;
}

#[tokio::test]
async fn wire_update_18_error() {
    run_case("update_18_error").await;
}

#[tokio::test]
async fn wire_create_24_error() {
    run_case("create_24_error").await;
}

#[tokio::test]
async fn wire_delete_21_error() {
    run_case("delete_21_error").await;
}

#[tokio::test]
async fn wire_create_25_error() {
    run_case("create_25_error").await;
}

#[tokio::test]
async fn wire_get_34_error() {
    run_case("get_34_error").await;
}

#[tokio::test]
async fn wire_delete_22_error() {
    run_case("delete_22_error").await;
}

#[tokio::test]
async fn wire_update_19_error() {
    run_case("update_19_error").await;
}

#[tokio::test]
async fn wire_get_35_error() {
    run_case("get_35_error").await;
}

#[tokio::test]
async fn wire_delete_23_error() {
    run_case("delete_23_error").await;
}

#[tokio::test]
async fn wire_update_20_error() {
    run_case("update_20_error").await;
}

#[tokio::test]
async fn wire_create_26_error() {
    run_case("create_26_error").await;
}

#[tokio::test]
async fn wire_get_36_error() {
    run_case("get_36_error").await;
}

#[tokio::test]
async fn wire_delete_24_error() {
    run_case("delete_24_error").await;
}

#[tokio::test]
async fn wire_update_21_error() {
    run_case("update_21_error").await;
}

#[tokio::test]
async fn wire_list_28_error() {
    run_case("list_28_error").await;
}

#[tokio::test]
async fn wire_get_37_error() {
    run_case("get_37_error").await;
}

#[tokio::test]
async fn wire_resubmit_error() {
    run_case("resubmit_error").await;
}

#[tokio::test]
async fn wire_get_38_error() {
    run_case("get_38_error").await;
}

#[tokio::test]
async fn wire_create_27_error() {
    run_case("create_27_error").await;
}

#[tokio::test]
async fn wire_list_29_error() {
    run_case("list_29_error").await;
}

#[tokio::test]
async fn wire_create_28_error() {
    run_case("create_28_error").await;
}

#[tokio::test]
async fn wire_get_39_error() {
    run_case("get_39_error").await;
}

#[tokio::test]
async fn wire_delete_25_error() {
    run_case("delete_25_error").await;
}

#[tokio::test]
async fn wire_get_40_error() {
    run_case("get_40_error").await;
}

#[tokio::test]
async fn wire_migrate_segments_error() {
    run_case("migrate_segments_error").await;
}

#[tokio::test]
async fn wire_transcribe_error() {
    run_case("transcribe_error").await;
}

#[tokio::test]
async fn wire_translate_error() {
    run_case("translate_error").await;
}

#[tokio::test]
async fn wire_dub_error() {
    run_case("dub_error").await;
}

#[tokio::test]
async fn wire_render_error() {
    run_case("render_error").await;
}

#[tokio::test]
async fn wire_get_41_error() {
    run_case("get_41_error").await;
}

#[tokio::test]
async fn wire_get_transcript_for_dub_error() {
    run_case("get_transcript_for_dub_error").await;
}

#[tokio::test]
async fn wire_get_42_error() {
    run_case("get_42_error").await;
}

#[tokio::test]
async fn wire_list_30_error() {
    run_case("list_30_error").await;
}

#[tokio::test]
async fn wire_create_29_error() {
    run_case("create_29_error").await;
}

#[tokio::test]
async fn wire_get_43_error() {
    run_case("get_43_error").await;
}

#[tokio::test]
async fn wire_delete_26_error() {
    run_case("delete_26_error").await;
}

#[tokio::test]
async fn wire_get_44_error() {
    run_case("get_44_error").await;
}

#[tokio::test]
async fn wire_delete_segment_error() {
    run_case("delete_segment_error").await;
}

#[tokio::test]
async fn wire_update_segment_error() {
    run_case("update_segment_error").await;
}

#[tokio::test]
async fn wire_update_segments_error() {
    run_case("update_segments_error").await;
}

#[tokio::test]
async fn wire_create_segment_error() {
    run_case("create_segment_error").await;
}

#[tokio::test]
async fn wire_get_45_error() {
    run_case("get_45_error").await;
}

#[tokio::test]
async fn wire_update_segment_1_error() {
    run_case("update_segment_1_error").await;
}

#[tokio::test]
async fn wire_update_segments_1_error() {
    run_case("update_segments_1_error").await;
}

#[tokio::test]
async fn wire_regenerate_error() {
    run_case("regenerate_error").await;
}

#[tokio::test]
async fn wire_add_error() {
    run_case("add_error").await;
}

#[tokio::test]
async fn wire_update_22_error() {
    run_case("update_22_error").await;
}

#[tokio::test]
async fn wire_delete_27_error() {
    run_case("delete_27_error").await;
}

#[tokio::test]
async fn wire_update_23_error() {
    run_case("update_23_error").await;
}

#[tokio::test]
async fn wire_create_30_error() {
    run_case("create_30_error").await;
}

#[tokio::test]
async fn wire_find_similar_voices_1_error() {
    run_case("find_similar_voices_1_error").await;
}

#[tokio::test]
async fn wire_create_31_error() {
    run_case("create_31_error").await;
}

#[tokio::test]
async fn wire_list_31_error() {
    run_case("list_31_error").await;
}

#[tokio::test]
async fn wire_create_32_error() {
    run_case("create_32_error").await;
}

#[tokio::test]
async fn wire_get_46_error() {
    run_case("get_46_error").await;
}

#[tokio::test]
async fn wire_list_32_error() {
    run_case("list_32_error").await;
}

#[tokio::test]
async fn wire_create_33_error() {
    run_case("create_33_error").await;
}

#[tokio::test]
async fn wire_get_47_error() {
    run_case("get_47_error").await;
}

#[tokio::test]
async fn wire_list_33_error() {
    run_case("list_33_error").await;
}

#[tokio::test]
async fn wire_create_34_error() {
    run_case("create_34_error").await;
}

#[tokio::test]
async fn wire_get_48_error() {
    run_case("get_48_error").await;
}

#[tokio::test]
async fn wire_create_35_error() {
    run_case("create_35_error").await;
}

#[tokio::test]
async fn wire_list_34_error() {
    run_case("list_34_error").await;
}

#[tokio::test]
async fn wire_create_36_error() {
    run_case("create_36_error").await;
}

#[tokio::test]
async fn wire_get_49_error() {
    run_case("get_49_error").await;
}

#[tokio::test]
async fn wire_delete_28_error() {
    run_case("delete_28_error").await;
}

#[tokio::test]
async fn wire_update_24_error() {
    run_case("update_24_error").await;
}

#[tokio::test]
async fn wire_list_35_error() {
    run_case("list_35_error").await;
}

#[tokio::test]
async fn wire_create_37_error() {
    run_case("create_37_error").await;
}

#[tokio::test]
async fn wire_get_50_error() {
    run_case("get_50_error").await;
}

#[tokio::test]
async fn wire_update_25_error() {
    run_case("update_25_error").await;
}

#[tokio::test]
async fn wire_submit_error() {
    run_case("submit_error").await;
}

#[tokio::test]
async fn wire_register_error() {
    run_case("register_error").await;
}

#[tokio::test]
async fn wire_get_51_error() {
    run_case("get_51_error").await;
}

#[tokio::test]
async fn wire_upsert_error() {
    run_case("upsert_error").await;
}

#[tokio::test]
async fn wire_remove_1_error() {
    run_case("remove_1_error").await;
}

#[tokio::test]
async fn wire_list_36_error() {
    run_case("list_36_error").await;
}

#[tokio::test]
async fn wire_list_37_error() {
    run_case("list_37_error").await;
}

#[tokio::test]
async fn wire_set_error() {
    run_case("set_error").await;
}

#[tokio::test]
async fn wire_add_1_error() {
    run_case("add_1_error").await;
}

#[tokio::test]
async fn wire_remove_2_error() {
    run_case("remove_2_error").await;
}

#[tokio::test]
async fn wire_list_38_error() {
    run_case("list_38_error").await;
}

#[tokio::test]
async fn wire_create_38_error() {
    run_case("create_38_error").await;
}

#[tokio::test]
async fn wire_delete_29_error() {
    run_case("delete_29_error").await;
}

#[tokio::test]
async fn wire_update_26_error() {
    run_case("update_26_error").await;
}

#[tokio::test]
async fn wire_get_52_error() {
    run_case("get_52_error").await;
}

#[tokio::test]
async fn wire_delete_30_error() {
    run_case("delete_30_error").await;
}

#[tokio::test]
async fn wire_list_39_error() {
    run_case("list_39_error").await;
}

#[tokio::test]
async fn wire_create_39_error() {
    run_case("create_39_error").await;
}

#[tokio::test]
async fn wire_get_53_error() {
    run_case("get_53_error").await;
}

#[tokio::test]
async fn wire_update_27_error() {
    run_case("update_27_error").await;
}

#[tokio::test]
async fn wire_delete_31_error() {
    run_case("delete_31_error").await;
}

#[tokio::test]
async fn wire_convert_5_error() {
    run_case("convert_5_error").await;
}

#[tokio::test]
async fn wire_get_muted_tracks_error() {
    run_case("get_muted_tracks_error").await;
}

#[tokio::test]
async fn wire_create_40_error() {
    run_case("create_40_error").await;
}

#[tokio::test]
async fn wire_update_28_error() {
    run_case("update_28_error").await;
}

#[tokio::test]
async fn wire_list_40_error() {
    run_case("list_40_error").await;
}

#[tokio::test]
async fn wire_get_54_error() {
    run_case("get_54_error").await;
}

#[tokio::test]
async fn wire_stream_4_error() {
    run_case("stream_4_error").await;
}

#[tokio::test]
async fn wire_stream_archive_error() {
    run_case("stream_archive_error").await;
}

#[tokio::test]
async fn wire_list_41_error() {
    run_case("list_41_error").await;
}

#[tokio::test]
async fn wire_create_41_error() {
    run_case("create_41_error").await;
}

#[tokio::test]
async fn wire_get_55_error() {
    run_case("get_55_error").await;
}

#[tokio::test]
async fn wire_update_29_error() {
    run_case("update_29_error").await;
}

#[tokio::test]
async fn wire_delete_32_error() {
    run_case("delete_32_error").await;
}

#[tokio::test]
async fn wire_convert_6_error() {
    run_case("convert_6_error").await;
}

#[tokio::test]
async fn wire_list_42_error() {
    run_case("list_42_error").await;
}

#[tokio::test]
async fn wire_get_56_error() {
    run_case("get_56_error").await;
}

#[tokio::test]
async fn wire_stream_5_error() {
    run_case("stream_5_error").await;
}

#[tokio::test]
async fn wire_stream_6_error() {
    run_case("stream_6_error").await;
}

#[tokio::test]
async fn wire_create_42_error() {
    run_case("create_42_error").await;
}

#[tokio::test]
async fn wire_get_57_error() {
    run_case("get_57_error").await;
}

#[tokio::test]
async fn wire_get_default_error() {
    run_case("get_default_error").await;
}

#[tokio::test]
async fn wire_get_58_error() {
    run_case("get_58_error").await;
}

#[tokio::test]
async fn wire_update_30_error() {
    run_case("update_30_error").await;
}

#[tokio::test]
async fn wire_get_59_error() {
    run_case("get_59_error").await;
}

#[tokio::test]
async fn wire_create_43_error() {
    run_case("create_43_error").await;
}

#[tokio::test]
async fn wire_create_44_error() {
    run_case("create_44_error").await;
}

#[tokio::test]
async fn wire_update_31_error() {
    run_case("update_31_error").await;
}

#[tokio::test]
async fn wire_train_error() {
    run_case("train_error").await;
}

#[tokio::test]
async fn wire_create_45_error() {
    run_case("create_45_error").await;
}

#[tokio::test]
async fn wire_update_32_error() {
    run_case("update_32_error").await;
}

#[tokio::test]
async fn wire_delete_33_error() {
    run_case("delete_33_error").await;
}

#[tokio::test]
async fn wire_request_error() {
    run_case("request_error").await;
}

#[tokio::test]
async fn wire_get_60_error() {
    run_case("get_60_error").await;
}

#[tokio::test]
async fn wire_get_61_error() {
    run_case("get_61_error").await;
}

#[tokio::test]
async fn wire_get_62_error() {
    run_case("get_62_error").await;
}

#[tokio::test]
async fn wire_separate_error() {
    run_case("separate_error").await;
}

#[tokio::test]
async fn wire_get_63_error() {
    run_case("get_63_error").await;
}

#[tokio::test]
async fn wire_get_64_error() {
    run_case("get_64_error").await;
}

#[tokio::test]
async fn wire_verify_error() {
    run_case("verify_error").await;
}

#[tokio::test]
async fn wire_get_65_error() {
    run_case("get_65_error").await;
}

#[tokio::test]
async fn wire_list_43_error() {
    run_case("list_43_error").await;
}

#[tokio::test]
async fn wire_list_44_error() {
    run_case("list_44_error").await;
}

#[tokio::test]
async fn wire_create_46_error() {
    run_case("create_46_error").await;
}

#[tokio::test]
async fn wire_delete_34_error() {
    run_case("delete_34_error").await;
}

#[tokio::test]
async fn wire_update_33_error() {
    run_case("update_33_error").await;
}

#[tokio::test]
async fn wire_list_45_error() {
    run_case("list_45_error").await;
}

#[tokio::test]
async fn wire_search_3_error() {
    run_case("search_3_error").await;
}

#[tokio::test]
async fn wire_create_47_error() {
    run_case("create_47_error").await;
}

#[tokio::test]
async fn wire_create_batch_error() {
    run_case("create_batch_error").await;
}

#[tokio::test]
async fn wire_delete_35_error() {
    run_case("delete_35_error").await;
}

#[tokio::test]
async fn wire_list_46_error() {
    run_case("list_46_error").await;
}

#[tokio::test]
async fn wire_update_34_error() {
    run_case("update_34_error").await;
}

#[tokio::test]
async fn wire_get_66_error() {
    run_case("get_66_error").await;
}

#[tokio::test]
async fn wire_share_1_error() {
    run_case("share_1_error").await;
}

#[tokio::test]
async fn wire_unshare_error() {
    run_case("unshare_error").await;
}

#[tokio::test]
async fn wire_get_usage_by_product_over_time_error() {
    run_case("get_usage_by_product_over_time_error").await;
}

#[tokio::test]
async fn wire_get_67_error() {
    run_case("get_67_error").await;
}

#[tokio::test]
async fn wire_remove_3_error() {
    run_case("remove_3_error").await;
}

#[tokio::test]
async fn wire_add_2_error() {
    run_case("add_2_error").await;
}

#[tokio::test]
async fn wire_disable_error() {
    run_case("disable_error").await;
}

#[tokio::test]
async fn wire_update_optfileomitted() {
    run_case("update_optfileomitted").await;
}

#[tokio::test]
async fn wire_find_similar_voices_optfileomitted() {
    run_case("find_similar_voices_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_1_optfileomitted() {
    run_case("create_1_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_2_optfileomitted() {
    run_case("create_2_optfileomitted").await;
}

#[tokio::test]
async fn wire_update_1_optfileomitted() {
    run_case("update_1_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_from_file_optfileomitted() {
    run_case("create_from_file_optfileomitted").await;
}

#[tokio::test]
async fn wire_convert_4_optfileomitted() {
    run_case("convert_4_optfileomitted").await;
}

#[tokio::test]
async fn wire_add_to_knowledge_base_optfileomitted() {
    run_case("add_to_knowledge_base_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_28_optfileomitted() {
    run_case("create_28_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_36_optfileomitted() {
    run_case("create_36_optfileomitted").await;
}

#[tokio::test]
async fn wire_register_optfileomitted() {
    run_case("register_optfileomitted").await;
}

#[tokio::test]
async fn wire_create_39_optfileomitted() {
    run_case("create_39_optfileomitted").await;
}

#[tokio::test]
async fn wire_update_28_optfileomitted() {
    run_case("update_28_optfileomitted").await;
}
