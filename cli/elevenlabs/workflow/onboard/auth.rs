//! The sign-in: the same browser flow as `auth login`, plus the two query
//! parameters that ask the approval page for a key, and the account check
//! that a key on the clipboard has to pass.

use std::sync::OnceLock;

use fern_cli_sdk::auth::{active_store, LoginContext, LoginFlow, PkceLoginFlow, TokenBundle};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::super::say::run_async as block_on;
use super::super::settings;

pub(super) const CLI_NAME: &str = "elevenlabs";

/// How `auth login` signs in. The runtime keeps its copy private, so this is
/// read out of the generated `main.rs`, the one place it is declared.
pub(super) struct SignIn {
    pub scheme: String,
    pub client_id: String,
    pub authorize_url: String,
    pub token_url: String,
    pub redirect_host: String,
    pub redirect_ports: Vec<u16>,
    pub success_url: String,
    pub error_url: String,
    pub scopes: Vec<String>,
}

pub(super) fn sign_in_config() -> Result<&'static SignIn, CliError> {
    static CONFIG: OnceLock<Result<SignIn, String>> = OnceLock::new();
    CONFIG
        .get_or_init(|| parse_sign_in(include_str!("../../main.rs")))
        .as_ref()
        .map_err(|e| CliError::Other(anyhow::anyhow!("main.rs sign-in: {e}")))
}

fn parse_sign_in(main_rs: &str) -> Result<SignIn, String> {
    let flow = main_rs
        .split("PkceLoginFlow::new(")
        .nth(1)
        .ok_or("no PkceLoginFlow")?;
    // The text inside `.method(`…`)`; no argument in that chain contains a `)`.
    let arg = |method: &str| -> Result<&str, String> {
        let needle = format!(".{method}(");
        let start = if method.is_empty() {
            0
        } else {
            flow.find(&needle).ok_or(format!("no {needle}"))? + needle.len()
        };
        flow[start..]
            .split(')')
            .next()
            .ok_or(format!("unterminated {needle}"))
    };
    let strings = |method: &str| -> Result<Vec<String>, String> {
        let found: Vec<String> = arg(method)?
            .split('"')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect();
        (!found.is_empty())
            .then_some(found)
            .ok_or(format!(".{method}( has no string"))
    };
    let one = |method: &str| strings(method).map(|mut v| v.remove(0));
    Ok(SignIn {
        scheme: one("")?,
        client_id: one("client_id")?,
        authorize_url: one("authorization_url")?,
        token_url: one("token_url")?,
        redirect_host: one("redirect_host")?,
        redirect_ports: arg("redirect_ports")?
            .split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .map(|p| p.parse().map_err(|_| format!("port {p}")))
            .collect::<Result<_, _>>()?,
        success_url: one("success_redirect_url")?,
        error_url: one("error_redirect_url")?,
        scopes: strings("scopes")?,
    })
}

/// Sign in through the browser, asking the page for a key.
pub(super) fn sign_in(
    ctx: &AppContext,
    authorize_url: &str,
    no_browser: bool,
) -> Result<(), CliError> {
    let config = sign_in_config()?;
    // A local --base-url also moves the token endpoint, for testing against a local backend.
    let oauth_base = ctx
        .base_url_override()
        .and_then(loopback)
        .unwrap_or_else(|| config.token_url.trim_end_matches("/v1/oauth/token").into());
    let token_url = format!("{oauth_base}/v1/oauth/token");
    PkceLoginFlow::new(&config.scheme)
        .client_id(&config.client_id)
        .authorization_url(authorize_url)
        .token_url(&token_url)
        .scopes(&config.scopes)
        .redirect_host(&config.redirect_host)
        .redirect_ports(config.redirect_ports.iter().copied())
        .success_redirect_url(&config.success_url)
        .error_redirect_url(&config.error_url)
        .authorization_params(vec![("key_handoff", "clipboard".to_string())])
        .run(&LoginContext {
            cli_name: CLI_NAME.to_string(),
            no_browser,
        })?;
    Ok(())
}

pub(super) fn stored_access_token() -> Option<String> {
    let scheme = &sign_in_config().ok()?.scheme;
    let raw = active_store().get(CLI_NAME, scheme).ok().flatten()?;
    let token = TokenBundle::parse_or_raw(&raw).access_token;
    (!token.is_empty()).then_some(token)
}

/// `Err` when the signed-in account itself cannot be identified; `Ok(false)` only
/// when it can and the key does not belong to it.
pub(super) fn key_matches_sign_in(
    client: &reqwest::Client,
    base: &str,
    key: &str,
) -> Result<bool, CliError> {
    let unknown = || {
        CliError::Other(anyhow::anyhow!(
            "the signed-in account could not be identified"
        ))
    };
    let token = stored_access_token().ok_or_else(unknown)?;
    let bearer = format!("Bearer {token}");
    let signed_in = account_for(client, base, "authorization", &bearer)?.ok_or_else(unknown)?;
    Ok(account_for(client, base, "xi-api-key", key)? == Some(signed_in))
}

/// The `user_id` a credential opens; `Ok(None)` when the API rejects it.
pub(super) fn account_for(
    client: &reqwest::Client,
    base: &str,
    header: &str,
    value: &str,
) -> Result<Option<String>, CliError> {
    let url = format!("{base}/v1/user");
    let fail = |e: reqwest::Error| CliError::Other(anyhow::anyhow!("reach {url}: {e}"));
    let response = block_on(
        client
            .get(&url)
            .header(header, value)
            .timeout(std::time::Duration::from_secs(15))
            .send(),
    )
    .map_err(fail)?;
    match response.status() {
        s if s == 401 || s == 403 => Ok(None),
        s if !s.is_success() => Err(CliError::Other(anyhow::anyhow!("{url} answered {s}"))),
        _ => {
            let body: Value = block_on(response.json()).map_err(fail)?;
            Ok(body["user_id"].as_str().map(str::to_string))
        }
    }
}
/// What ElevenLabs says about an API key.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum KeyCheck {
    Works,
    Rejected,
    /// No answer about the key: unreachable, or an error such as a 5xx.
    Unanswered(String),
}

/// Ask `GET /v1/user` about `key`. That endpoint needs the key's `user_read`
/// permission, which a key made by hand on the API keys page may lack: such a
/// key still works, it just can't read the account, so it counts as working.
pub(super) fn check_key(ctx: &AppContext, key: &str) -> KeyCheck {
    let Ok(client) = ctx.http_config().build_client() else {
        return KeyCheck::Unanswered("the HTTP client could not start".into());
    };
    let url = format!("{}/v1/user", api_base(ctx));
    let sent = block_on(
        client
            .get(&url)
            .header("xi-api-key", key)
            .timeout(std::time::Duration::from_secs(15))
            .send(),
    );
    let response = match sent {
        Ok(r) => r,
        Err(_) => return KeyCheck::Unanswered("ElevenLabs couldn't be reached".into()),
    };
    let status = response.status();
    if status.is_success() {
        return KeyCheck::Works;
    }
    if status == 401 || status == 403 {
        let body: Value = block_on(response.json()).unwrap_or(Value::Null);
        return if body["detail"]["status"] == "missing_permissions" {
            KeyCheck::Works
        } else {
            KeyCheck::Rejected
        };
    }
    KeyCheck::Unanswered(format!("ElevenLabs answered {}", status.as_u16()))
}

/// `base` without its trailing slash when its host is this machine.
pub(super) fn loopback(base: &str) -> Option<String> {
    let url = reqwest::Url::parse(base).ok()?;
    let host = url.host_str()?;
    let local = host.eq_ignore_ascii_case("localhost")
        || host
            .trim_matches(|c| c == '[' || c == ']')
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
    local.then(|| base.trim_end_matches('/').to_string())
}

/// The API host for this run: a local `--base-url` (for testing against a
/// local backend), else the configured residency. A key and a bearer token
/// go to this host, so, like the token endpoint, nothing else may redirect it.
pub(super) fn api_base(ctx: &AppContext) -> String {
    ctx.base_url_override()
        .and_then(loopback)
        .unwrap_or_else(|| settings::base_url_for(&settings::read_residency()).to_string())
}
