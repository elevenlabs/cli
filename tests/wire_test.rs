//! Auto-generated wire tests by @fern-api/cli-generator.
//!
//! Each test stands up an in-process `wiremock` server, points the
//! generated CLI at it via `--base-url`, drives one endpoint from an IR
//! example (`--params` / `--json`), and asserts the CLI (a) hit the mock
//! exactly once with the right method + path and (b) rendered the mocked
//! response body to stdout.
//!
//! No docker, no network — runs under a plain `cargo test`. Regenerated on
//! every `fern generate`; do not edit by hand.
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;
use wiremock::matchers::{
    header as match_header, header_regex as match_header_regex, method as match_method, path as match_path,
    query_param as match_query_param,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

use fern_cli_sdk::openapi::discovery::RestResource;
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
    auth_env_vars: Vec<String>,
    #[serde(rename = "authMock")]
    auth_mock: Option<AuthMock>,
    cases: Vec<Case>,
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
    response: ExpectedResponse,
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
    /// multipart/form-data — per-field file/value flags, not `--json`.
    is_multipart: bool,
    /// Body was flattened into per-field flags (params carry location "body").
    /// Such endpoints reject a whole-body `--json`, so the generic
    /// `--params`/`--json` driver can't feed them.
    has_body_field_flags: bool,
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
        is_multipart: !method.multipart_fields.is_empty(),
        has_body_field_flags: method
            .parameters
            .values()
            .any(|p| p.location.as_deref() == Some("body")),
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
    let want_path = normalize_path(path);

    // Exact match on method + normalized path.
    let mut hits: Vec<CommandInfo> = commands
        .iter()
        .filter(|c| c.http_method == want_method && normalize_path(&c.path) == want_path)
        .cloned()
        .collect();

    // Fallback: tolerate base-path prefixes by matching path suffixes.
    if hits.is_empty() {
        hits = commands
            .iter()
            .filter(|c| {
                let np = normalize_path(&c.path);
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

async fn run_case(id: &str) {
    let manifest = load_manifest();
    let case = manifest
        .cases
        .iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("wire-test case {id} not found in manifest"));

    let command = resolve_command(&manifest, &case.method, &case.path);

    // Some endpoints can't be driven by the generic --params / --json
    // mechanism: file & multipart uploads need a real file, and bodies the CLI
    // flattened into per-field flags reject a whole-body --json. Skip those
    // (the test passes) and log why, rather than emit a guaranteed failure —
    // mirrors the SDK wire-test generator, which skips endpoints it can't
    // synthesize a call for.
    if command.is_binary || command.is_multipart || command.has_body_field_flags {
        let reason = if command.is_binary {
            "binary/file-upload request body"
        } else if command.is_multipart {
            "multipart/form-data request body"
        } else {
            "request body is exposed as per-field flags, not --json"
        };
        eprintln!("skipping wire test {id} ({} {}): {reason}", case.method, case.path);
        return;
    }

    let expected_path = substitute_path(&case.path, &case.params);

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
    mock.respond_with(template).expect(1).mount(&server).await;

    let mut args: Vec<String> = command.chain.clone();
    args.push("--base-url".to_string());
    args.push(server.uri());
    args.push("--no-pager".to_string());
    if !case.params.is_empty() {
        args.push("--params".to_string());
        args.push(serde_json::to_string(&case.params).expect("params serialize"));
    }
    // Feed --json only when the endpoint actually registers it (opaque JSON
    // body). Endpoints without it were filtered out by the skip above.
    if command.has_json_body && !case.body.is_null() {
        args.push("--json".to_string());
        args.push(serde_json::to_string(&case.body).expect("body serialize"));
    }

    let mut cmd = tokio::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(&args);
    // Dummy credentials so auth-gated endpoints don't bail on a missing secret.
    for var in &manifest.auth_env_vars {
        cmd.env(var, "test");
    }
    cmd.env("NO_COLOR", "1");

    let output = cmd
        .output()
        .await
        .unwrap_or_else(|e| panic!("failed to spawn generated CLI binary: {e}"));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Primary assertion (mirrors the SDK wire tests: the call succeeds, and
    // `Mock::expect(1)` verifies exactly one matching request on server drop).
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
async fn wire_share() {
    run_case("share").await;
}

#[tokio::test]
async fn wire_get_shared() {
    run_case("get_shared").await;
}

#[tokio::test]
async fn wire_find_similar_voices() {
    run_case("find_similar_voices").await;
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
async fn wire_get_signed_url() {
    run_case("get_signed_url").await;
}

#[tokio::test]
async fn wire_get_webrtc_token() {
    run_case("get_webrtc_token").await;
}

#[tokio::test]
async fn wire_list_10() {
    run_case("list_10").await;
}

#[tokio::test]
async fn wire_resolve() {
    run_case("resolve").await;
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
async fn wire_get_10() {
    run_case("get_10").await;
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
async fn wire_size() {
    run_case("size").await;
}

#[tokio::test]
async fn wire_list_11() {
    run_case("list_11").await;
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
async fn wire_create_9() {
    run_case("create_9").await;
}

#[tokio::test]
async fn wire_move() {
    run_case("move").await;
}

#[tokio::test]
async fn wire_get_13() {
    run_case("get_13").await;
}

#[tokio::test]
async fn wire_update_7() {
    run_case("update_7").await;
}

#[tokio::test]
async fn wire_delete_9() {
    run_case("delete_9").await;
}

#[tokio::test]
async fn wire_summaries() {
    run_case("summaries").await;
}

#[tokio::test]
async fn wire_list_12() {
    run_case("list_12").await;
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
async fn wire_create_10() {
    run_case("create_10").await;
}

#[tokio::test]
async fn wire_get_14() {
    run_case("get_14").await;
}

#[tokio::test]
async fn wire_delete_10() {
    run_case("delete_10").await;
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
async fn wire_list_15() {
    run_case("list_15").await;
}

#[tokio::test]
async fn wire_list_16() {
    run_case("list_16").await;
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
async fn wire_update_9() {
    run_case("update_9").await;
}

#[tokio::test]
async fn wire_get_dependent_agents() {
    run_case("get_dependent_agents").await;
}

#[tokio::test]
async fn wire_get_16() {
    run_case("get_16").await;
}

#[tokio::test]
async fn wire_update_10() {
    run_case("update_10").await;
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
async fn wire_get_17() {
    run_case("get_17").await;
}

#[tokio::test]
async fn wire_delete_12() {
    run_case("delete_12").await;
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
async fn wire_export() {
    run_case("export").await;
}

#[tokio::test]
async fn wire_create_13() {
    run_case("create_13").await;
}

#[tokio::test]
async fn wire_list_18() {
    run_case("list_18").await;
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
async fn wire_cancel() {
    run_case("cancel").await;
}

#[tokio::test]
async fn wire_retry() {
    run_case("retry").await;
}

#[tokio::test]
async fn wire_outbound_call_3() {
    run_case("outbound_call_3").await;
}

#[tokio::test]
async fn wire_list_19() {
    run_case("list_19").await;
}

#[tokio::test]
async fn wire_create_14() {
    run_case("create_14").await;
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
async fn wire_update_12() {
    run_case("update_12").await;
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
async fn wire_update_13() {
    run_case("update_13").await;
}

#[tokio::test]
async fn wire_list_20() {
    run_case("list_20").await;
}

#[tokio::test]
async fn wire_list_21() {
    run_case("list_21").await;
}

#[tokio::test]
async fn wire_create_15() {
    run_case("create_15").await;
}

#[tokio::test]
async fn wire_get_21() {
    run_case("get_21").await;
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
async fn wire_get_22() {
    run_case("get_22").await;
}

#[tokio::test]
async fn wire_create_16() {
    run_case("create_16").await;
}

#[tokio::test]
async fn wire_create_17() {
    run_case("create_17").await;
}

#[tokio::test]
async fn wire_delete_16() {
    run_case("delete_16").await;
}

#[tokio::test]
async fn wire_calculate_1() {
    run_case("calculate_1").await;
}

#[tokio::test]
async fn wire_get_23() {
    run_case("get_23").await;
}

#[tokio::test]
async fn wire_get_24() {
    run_case("get_24").await;
}

#[tokio::test]
async fn wire_create_18() {
    run_case("create_18").await;
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
async fn wire_list_22() {
    run_case("list_22").await;
}

#[tokio::test]
async fn wire_create_19() {
    run_case("create_19").await;
}

#[tokio::test]
async fn wire_get_25() {
    run_case("get_25").await;
}

#[tokio::test]
async fn wire_delete_17() {
    run_case("delete_17").await;
}

#[tokio::test]
async fn wire_update_15() {
    run_case("update_15").await;
}

#[tokio::test]
async fn wire_create_20() {
    run_case("create_20").await;
}

#[tokio::test]
async fn wire_delete_18() {
    run_case("delete_18").await;
}

#[tokio::test]
async fn wire_get_26() {
    run_case("get_26").await;
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
async fn wire_get_27() {
    run_case("get_27").await;
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
async fn wire_get_28() {
    run_case("get_28").await;
}

#[tokio::test]
async fn wire_delete_19() {
    run_case("delete_19").await;
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
async fn wire_list_23() {
    run_case("list_23").await;
}

#[tokio::test]
async fn wire_create_21() {
    run_case("create_21").await;
}

#[tokio::test]
async fn wire_get_29() {
    run_case("get_29").await;
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
async fn wire_get_30() {
    run_case("get_30").await;
}

#[tokio::test]
async fn wire_get_31() {
    run_case("get_31").await;
}

#[tokio::test]
async fn wire_list_24() {
    run_case("list_24").await;
}

#[tokio::test]
async fn wire_list_25() {
    run_case("list_25").await;
}

#[tokio::test]
async fn wire_update_18() {
    run_case("update_18").await;
}

#[tokio::test]
async fn wire_create_22() {
    run_case("create_22").await;
}

#[tokio::test]
async fn wire_delete_20() {
    run_case("delete_20").await;
}

#[tokio::test]
async fn wire_create_23() {
    run_case("create_23").await;
}

#[tokio::test]
async fn wire_get_32() {
    run_case("get_32").await;
}

#[tokio::test]
async fn wire_delete_21() {
    run_case("delete_21").await;
}

#[tokio::test]
async fn wire_update_19() {
    run_case("update_19").await;
}

#[tokio::test]
async fn wire_create_24() {
    run_case("create_24").await;
}

#[tokio::test]
async fn wire_get_33() {
    run_case("get_33").await;
}

#[tokio::test]
async fn wire_delete_22() {
    run_case("delete_22").await;
}

#[tokio::test]
async fn wire_update_20() {
    run_case("update_20").await;
}

#[tokio::test]
async fn wire_list_26() {
    run_case("list_26").await;
}

#[tokio::test]
async fn wire_get_34() {
    run_case("get_34").await;
}

#[tokio::test]
async fn wire_resubmit() {
    run_case("resubmit").await;
}

#[tokio::test]
async fn wire_get_35() {
    run_case("get_35").await;
}

#[tokio::test]
async fn wire_create_25() {
    run_case("create_25").await;
}

#[tokio::test]
async fn wire_list_27() {
    run_case("list_27").await;
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
async fn wire_delete_23() {
    run_case("delete_23").await;
}

#[tokio::test]
async fn wire_get_37() {
    run_case("get_37").await;
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
async fn wire_get_38() {
    run_case("get_38").await;
}

#[tokio::test]
async fn wire_get_transcript_for_dub() {
    run_case("get_transcript_for_dub").await;
}

#[tokio::test]
async fn wire_get_39() {
    run_case("get_39").await;
}

#[tokio::test]
async fn wire_list_28() {
    run_case("list_28").await;
}

#[tokio::test]
async fn wire_create_27() {
    run_case("create_27").await;
}

#[tokio::test]
async fn wire_get_40() {
    run_case("get_40").await;
}

#[tokio::test]
async fn wire_delete_24() {
    run_case("delete_24").await;
}

#[tokio::test]
async fn wire_get_41() {
    run_case("get_41").await;
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
async fn wire_create_segment() {
    run_case("create_segment").await;
}

#[tokio::test]
async fn wire_get_42() {
    run_case("get_42").await;
}

#[tokio::test]
async fn wire_update_segment_1() {
    run_case("update_segment_1").await;
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
async fn wire_update_21() {
    run_case("update_21").await;
}

#[tokio::test]
async fn wire_delete_25() {
    run_case("delete_25").await;
}

#[tokio::test]
async fn wire_update_22() {
    run_case("update_22").await;
}

#[tokio::test]
async fn wire_create_28() {
    run_case("create_28").await;
}

#[tokio::test]
async fn wire_find_similar_voices_1() {
    run_case("find_similar_voices_1").await;
}

#[tokio::test]
async fn wire_create_29() {
    run_case("create_29").await;
}

#[tokio::test]
async fn wire_create_30() {
    run_case("create_30").await;
}

#[tokio::test]
async fn wire_list_29() {
    run_case("list_29").await;
}

#[tokio::test]
async fn wire_create_31() {
    run_case("create_31").await;
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
async fn wire_update_23() {
    run_case("update_23").await;
}

#[tokio::test]
async fn wire_list_30() {
    run_case("list_30").await;
}

#[tokio::test]
async fn wire_create_32() {
    run_case("create_32").await;
}

#[tokio::test]
async fn wire_get_44() {
    run_case("get_44").await;
}

#[tokio::test]
async fn wire_update_24() {
    run_case("update_24").await;
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
async fn wire_get_45() {
    run_case("get_45").await;
}

#[tokio::test]
async fn wire_upsert() {
    run_case("upsert").await;
}

#[tokio::test]
async fn wire_remove() {
    run_case("remove").await;
}

#[tokio::test]
async fn wire_list_31() {
    run_case("list_31").await;
}

#[tokio::test]
async fn wire_list_32() {
    run_case("list_32").await;
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
async fn wire_remove_1() {
    run_case("remove_1").await;
}

#[tokio::test]
async fn wire_list_33() {
    run_case("list_33").await;
}

#[tokio::test]
async fn wire_create_33() {
    run_case("create_33").await;
}

#[tokio::test]
async fn wire_delete_27() {
    run_case("delete_27").await;
}

#[tokio::test]
async fn wire_update_25() {
    run_case("update_25").await;
}

#[tokio::test]
async fn wire_get_46() {
    run_case("get_46").await;
}

#[tokio::test]
async fn wire_delete_28() {
    run_case("delete_28").await;
}

#[tokio::test]
async fn wire_list_34() {
    run_case("list_34").await;
}

#[tokio::test]
async fn wire_create_34() {
    run_case("create_34").await;
}

#[tokio::test]
async fn wire_get_47() {
    run_case("get_47").await;
}

#[tokio::test]
async fn wire_update_26() {
    run_case("update_26").await;
}

#[tokio::test]
async fn wire_delete_29() {
    run_case("delete_29").await;
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
async fn wire_create_35() {
    run_case("create_35").await;
}

#[tokio::test]
async fn wire_update_27() {
    run_case("update_27").await;
}

#[tokio::test]
async fn wire_list_35() {
    run_case("list_35").await;
}

#[tokio::test]
async fn wire_get_48() {
    run_case("get_48").await;
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
async fn wire_list_36() {
    run_case("list_36").await;
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
async fn wire_update_28() {
    run_case("update_28").await;
}

#[tokio::test]
async fn wire_delete_30() {
    run_case("delete_30").await;
}

#[tokio::test]
async fn wire_convert_6() {
    run_case("convert_6").await;
}

#[tokio::test]
async fn wire_list_37() {
    run_case("list_37").await;
}

#[tokio::test]
async fn wire_get_50() {
    run_case("get_50").await;
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
async fn wire_create_37() {
    run_case("create_37").await;
}

#[tokio::test]
async fn wire_get_51() {
    run_case("get_51").await;
}

#[tokio::test]
async fn wire_get_default() {
    run_case("get_default").await;
}

#[tokio::test]
async fn wire_get_52() {
    run_case("get_52").await;
}

#[tokio::test]
async fn wire_update_29() {
    run_case("update_29").await;
}

#[tokio::test]
async fn wire_create_38() {
    run_case("create_38").await;
}

#[tokio::test]
async fn wire_create_39() {
    run_case("create_39").await;
}

#[tokio::test]
async fn wire_update_30() {
    run_case("update_30").await;
}

#[tokio::test]
async fn wire_train() {
    run_case("train").await;
}

#[tokio::test]
async fn wire_create_40() {
    run_case("create_40").await;
}

#[tokio::test]
async fn wire_update_31() {
    run_case("update_31").await;
}

#[tokio::test]
async fn wire_delete_31() {
    run_case("delete_31").await;
}

#[tokio::test]
async fn wire_request() {
    run_case("request").await;
}

#[tokio::test]
async fn wire_get_53() {
    run_case("get_53").await;
}

#[tokio::test]
async fn wire_get_54() {
    run_case("get_54").await;
}

#[tokio::test]
async fn wire_get_55() {
    run_case("get_55").await;
}

#[tokio::test]
async fn wire_separate() {
    run_case("separate").await;
}

#[tokio::test]
async fn wire_get_56() {
    run_case("get_56").await;
}

#[tokio::test]
async fn wire_get_57() {
    run_case("get_57").await;
}

#[tokio::test]
async fn wire_verify() {
    run_case("verify").await;
}

#[tokio::test]
async fn wire_get_58() {
    run_case("get_58").await;
}

#[tokio::test]
async fn wire_list_38() {
    run_case("list_38").await;
}

#[tokio::test]
async fn wire_list_39() {
    run_case("list_39").await;
}

#[tokio::test]
async fn wire_create_41() {
    run_case("create_41").await;
}

#[tokio::test]
async fn wire_delete_32() {
    run_case("delete_32").await;
}

#[tokio::test]
async fn wire_update_32() {
    run_case("update_32").await;
}

#[tokio::test]
async fn wire_list_40() {
    run_case("list_40").await;
}

#[tokio::test]
async fn wire_search_3() {
    run_case("search_3").await;
}

#[tokio::test]
async fn wire_create_42() {
    run_case("create_42").await;
}

#[tokio::test]
async fn wire_create_batch() {
    run_case("create_batch").await;
}

#[tokio::test]
async fn wire_delete_33() {
    run_case("delete_33").await;
}

#[tokio::test]
async fn wire_list_41() {
    run_case("list_41").await;
}

#[tokio::test]
async fn wire_update_33() {
    run_case("update_33").await;
}

#[tokio::test]
async fn wire_get_59() {
    run_case("get_59").await;
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
async fn wire_get_60() {
    run_case("get_60").await;
}

#[tokio::test]
async fn wire_remove_2() {
    run_case("remove_2").await;
}

#[tokio::test]
async fn wire_add_2() {
    run_case("add_2").await;
}

#[tokio::test]
async fn wire_disable() {
    run_case("disable").await;
}
