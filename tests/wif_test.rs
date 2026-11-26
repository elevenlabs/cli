//! Binary-level WIF tests: the real CLI against a mock API serving the token exchange.

use std::path::Path;
use std::process::Output;

use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const KEY: &str = "sk_0123456789abcdef0123456789abcdef01234567";
/// A made-up unsigned JWS; the mock exchange checks only that it is forwarded.
const SOURCE_TOKEN: &str = "eyJhbGciOiJub25lIn0.eyJzdWIiOiJ3aWYtdGVzdCJ9.c2ln";
const ACCESS_TOKEN: &str = "wif-test-access-token";
const POOL: &str = "urn:elevenlabs-xi:workload_identity_federation:wif_test_pool";
const RESOURCE: &str = "urn:elevenlabs-xi:api_key:test_resource";
const BEARER: &str = "Bearer wif-test-access-token";

struct Run<'a> {
    dir: &'a Path,
    server: &'a str,
    args: &'a [&'a str],
    env: &'a [(&'a str, &'a str)],
}

fn cli(run: Run) -> Output {
    let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    cmd.args(["--base-url", run.server])
        .args(run.args)
        .current_dir(run.dir)
        .env("HOME", run.dir)
        .env("TMPDIR", run.dir)
        .env("NO_COLOR", "1")
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("ELEVENLABS_TELEMETRY", "0");
    for var in [
        "XDG_CONFIG_HOME",
        "APPDATA",
        "ELEVENLABS_API_KEY",
        "ELEVENLABS_WIF_CONFIG",
        "ELEVENLABS_BASE_URL",
        "ELEVENLABS_VIA",
        "ELEVENLABS_OUTPUT",
        "ELEVENLABS_PROXY",
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "http_proxy",
        "https_proxy",
        "all_proxy",
    ] {
        cmd.env_remove(var);
    }
    cmd.envs(run.env.iter().copied());
    cmd.output().expect("failed to spawn the elevenlabs binary")
}

fn describe(out: &Output) -> String {
    format!(
        "exit {:?}\nstdout: {}\nstderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn write(dir: &Path, file: &str, text: &str) {
    let p = dir.join(file);
    std::fs::create_dir_all(p.parent().unwrap()).unwrap();
    std::fs::write(p, text).unwrap();
}

/// Writes wif.json with a file source, and returns its path.
fn write_config(dir: &Path) -> String {
    write(dir, "source-token", SOURCE_TOKEN);
    let config = json!({
        "type": "elevenlabs_workload_identity_federation",
        "version": 1,
        "pool": POOL,
        "resource": RESOURCE,
        "token_source": {"type": "file", "path": "source-token"}
    });
    write(dir, "wif.json", &config.to_string());
    dir.join("wif.json").display().to_string()
}

/// A value as it appears in a form-encoded body.
fn form(value: &str) -> String {
    value.replace(':', "%3A")
}

/// The exchange, answering only a request that forwards the source token for this pool.
async fn mount_exchange(server: &MockServer, calls: u64) {
    Mock::given(method("POST"))
        .and(path("/v1/oidc/token"))
        .and(body_string_contains(format!(
            "subject_token={SOURCE_TOKEN}"
        )))
        .and(body_string_contains(format!("audience={}", form(POOL))))
        .and(body_string_contains(format!("resource={}", form(RESOURCE))))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": ACCESS_TOKEN,
            "issued_token_type": "urn:ietf:params:oauth:token-type:access_token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "scope": "user_read"
        })))
        .expect(calls)
        .mount(server)
        .await;
}

async fn mount_user(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/v1/user"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"user_id": "u1"})))
        .mount(server)
        .await;
}

async fn api_requests(server: &MockServer) -> Vec<Request> {
    server
        .received_requests()
        .await
        .unwrap()
        .into_iter()
        .filter(|r| r.url.path() != "/v1/oidc/token")
        .collect()
}

fn auth_of(request: &Request) -> (Option<String>, Option<String>) {
    let get = |name: &str| {
        request
            .headers
            .get(name)
            .map(|v| v.to_str().unwrap().to_string())
    };
    (get("authorization"), get("xi-api-key"))
}

/// Every API request carries the WIF bearer token and no API key.
async fn assert_all_wif(server: &MockServer, out: &Output) {
    let requests = api_requests(server).await;
    assert!(!requests.is_empty(), "no API request:\n{}", describe(out));
    for r in &requests {
        assert_eq!(
            auth_of(r),
            (Some(BEARER.into()), None),
            "{} {}",
            r.method,
            r.url.path()
        );
    }
}

#[tokio::test]
async fn generated_command_uses_wif_from_flag() {
    let server = MockServer::start().await;
    mount_exchange(&server, 1).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get", "--wif-config", &config],
        env: &[],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert_all_wif(&server, &out).await;
}

#[tokio::test]
async fn generated_command_uses_wif_from_env() {
    let server = MockServer::start().await;
    mount_exchange(&server, 1).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get"],
        env: &[("ELEVENLABS_WIF_CONFIG", &config)],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert_all_wif(&server, &out).await;
}

#[tokio::test]
async fn workflow_command_uses_wif() {
    let server = MockServer::start().await;
    mount_exchange(&server, 1).await;
    Mock::given(method("PATCH"))
        .and(path("/v1/convai/agents/agent_1"))
        .and(header("authorization", BEARER))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agent_id": "agent_1"})))
        .expect(1)
        .mount(&server)
        .await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());
    let agent = json!({
        "name": "Support",
        "conversation_config": {"agent": {"first_message": "Hello",
            "prompt": {"prompt": "You are helpful.", "llm": "gemini-2.5-flash"}}}
    });
    write(dir.path(), "agent_configs/support.json", &agent.to_string());
    write(
        dir.path(),
        "agents.json",
        &json!({"agents": [{"config": "agent_configs/support.json", "id": "agent_1"}]}).to_string(),
    );

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["agents", "push"],
        env: &[("ELEVENLABS_WIF_CONFIG", &config)],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert_all_wif(&server, &out).await;
}

#[tokio::test]
async fn env_api_key_beats_env_wif_config() {
    let server = MockServer::start().await;
    mount_exchange(&server, 0).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get"],
        env: &[
            ("ELEVENLABS_API_KEY", KEY),
            ("ELEVENLABS_WIF_CONFIG", &config),
        ],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("ELEVENLABS_WIF_CONFIG is ignored"),
        "{}",
        describe(&out)
    );
    let requests = api_requests(&server).await;
    assert_eq!(requests.len(), 1, "{}", describe(&out));
    assert_eq!(auth_of(&requests[0]).1.as_deref(), Some(KEY));
}

#[tokio::test]
async fn wif_flag_beats_env_api_key() {
    let server = MockServer::start().await;
    mount_exchange(&server, 1).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get", "--wif-config", &config],
        env: &[("ELEVENLABS_API_KEY", KEY)],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert_all_wif(&server, &out).await;
}

#[tokio::test]
async fn api_key_flag_and_wif_flag_conflict() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get", "--api-key", KEY, "--wif-config", &config],
        env: &[],
    });
    assert_eq!(out.status.code(), Some(2), "{}", describe(&out));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn dotenv_api_key_does_not_replace_wif() {
    let server = MockServer::start().await;
    mount_exchange(&server, 1).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());
    write(dir.path(), ".env", &format!("ELEVENLABS_API_KEY={KEY}\n"));

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get"],
        env: &[("ELEVENLABS_WIF_CONFIG", &config)],
    });
    assert!(out.status.success(), "{}", describe(&out));
    assert_all_wif(&server, &out).await;
}

#[tokio::test]
async fn dotenv_wif_config_is_ignored() {
    let server = MockServer::start().await;
    mount_exchange(&server, 0).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());
    write(
        dir.path(),
        ".env",
        &format!("ELEVENLABS_WIF_CONFIG={config}\n"),
    );

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get"],
        env: &[("ELEVENLABS_API_KEY", KEY)],
    });
    assert!(out.status.success(), "{}", describe(&out));
    let requests = api_requests(&server).await;
    assert_eq!(requests.len(), 1, "{}", describe(&out));
    assert_eq!(auth_of(&requests[0]).1.as_deref(), Some(KEY));
}

#[tokio::test]
async fn rejected_exchange_stops_the_api_request() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/oidc/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "invalid_grant",
            "error_description": "subject token rejected"
        })))
        .expect(1)
        .mount(&server)
        .await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let out = cli(Run {
        dir: dir.path(),
        server: &server.uri(),
        args: &["user", "get", "--wif-config", &config],
        env: &[],
    });
    assert_eq!(out.status.code(), Some(2), "{}", describe(&out));
    assert!(
        describe(&out).contains("invalid_grant"),
        "{}",
        describe(&out)
    );
    assert!(api_requests(&server).await.is_empty());
}

#[tokio::test]
async fn stored_login_and_wif_conflict() {
    let server = MockServer::start().await;
    mount_exchange(&server, 0).await;
    mount_user(&server).await;
    let dir = tempfile::tempdir().unwrap();
    let config = write_config(dir.path());

    let mut login = std::process::Command::new(env!("CARGO_BIN_EXE_elevenlabs"));
    login
        .args(["auth", "login", "--with-token"])
        .current_dir(dir.path())
        .env("HOME", dir.path())
        .env("FERN_CLI_CREDENTIAL_STORE", "file")
        .env("ELEVENLABS_TELEMETRY", "0")
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("ELEVENLABS_API_KEY")
        .env_remove("ELEVENLABS_WIF_CONFIG")
        .stdin(std::process::Stdio::piped());
    let mut child = login.spawn().unwrap();
    use std::io::Write;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"fake-oauth-token\n")
        .unwrap();
    assert!(child.wait().unwrap().success(), "auth login failed");

    for args in [
        &["user", "get", "--wif-config", &config][..],
        &["wif", "print-identity-token", "--wif-config", &config][..],
    ] {
        let out = cli(Run {
            dir: dir.path(),
            server: &server.uri(),
            args,
            env: &[],
        });
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", describe(&out));
        assert!(
            describe(&out).contains("auth login"),
            "{args:?}: {}",
            describe(&out)
        );
    }
    assert!(server.received_requests().await.unwrap().is_empty());
}
