//! The sign-in: the same browser flow as `auth login`, plus the query
//! parameter that asks the approval page for a key, and the account check
//! that a key on the clipboard has to pass.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpListener};
use std::sync::{Arc, Mutex, OnceLock};

use fern_cli_sdk::auth::{
    active_store, set_active_store, KeyringStore, LoginContext, LoginFlow, PkceLoginFlow,
    TokenBundle,
};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;
use serde_json::Value;

use super::super::platform;
use super::super::say::run_async as block_on;
use super::super::settings;
use super::super::telemetry;

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

/// Where the sign-in is saved while onboard signs in: the usual place, but a
/// sign-in that can't be saved there still counts. `auth login` fails a
/// sign-in it can't save for later commands. Onboard only needs the token
/// now, to check the key the approval page handed over, so a keychain that
/// refuses this binary, or a sandbox, must not end a sign-in the developer
/// has already approved. It also remembers what this sign-in saved, so the
/// token is this sign-in's own, whatever another command writes meanwhile.
#[derive(Debug)]
pub(super) struct KeepsUnsaved {
    usual: Arc<dyn KeyringStore>,
    /// What the sign-in saved, and whether the usual place took it.
    saved: Mutex<Option<(String, bool)>>,
}

impl KeepsUnsaved {
    pub(super) fn over(usual: Arc<dyn KeyringStore>) -> Self {
        Self {
            usual,
            saved: Mutex::new(None),
        }
    }

    /// The access token of the sign-in saved through this store.
    pub(super) fn access_token(&self) -> Option<String> {
        let (raw, _) = self.saved.lock().ok()?.clone()?;
        let token = TokenBundle::parse_or_raw(&raw).access_token;
        (!token.is_empty()).then_some(token)
    }
}

impl KeyringStore for KeepsUnsaved {
    fn get(&self, service: &str, account: &str) -> Result<Option<String>, CliError> {
        self.usual.get(service, account)
    }

    fn set(&self, service: &str, account: &str, value: &str) -> Result<(), CliError> {
        let kept = self.usual.set(service, account, value);
        if let Err(e) = &kept {
            eprintln!("Signed in, but the sign-in couldn't be saved for later commands: {e}");
        }
        if let Ok(mut saved) = self.saved.lock() {
            *saved = Some((value.to_string(), kept.is_ok()));
        }
        Ok(())
    }

    fn delete(&self, service: &str, account: &str) -> Result<(), CliError> {
        self.usual.delete(service, account)
    }

    fn backend_label(&self) -> String {
        match self.saved.lock().ok().and_then(|saved| saved.clone()) {
            Some((_, false)) => "this command only".into(),
            _ => self.usual.backend_label(),
        }
    }
}

/// The registered ports where nothing listens on either loopback address, in
/// order. `auth login`'s listener takes one address of a port, so a second
/// sign-in on the same machine gets the same port number on the other address,
/// and the browser's callback for one reaches the other, which ends on a
/// `state` that isn't its own. A port free on both keeps each sign-in on a
/// port of its own.
pub(super) fn ports_free_on_both(registered: &[u16]) -> Vec<u16> {
    let free = |ip: IpAddr, port: u16| match TcpListener::bind((ip, port)) {
        Ok(_) => true,
        // Anything else is an address this machine doesn't have (IPv6 turned
        // off), where nothing can be listening either.
        Err(e) => !matches!(
            e.kind(),
            std::io::ErrorKind::AddrInUse | std::io::ErrorKind::PermissionDenied
        ),
    };
    registered
        .iter()
        .copied()
        .filter(|&port| {
            free(IpAddr::V4(Ipv4Addr::LOCALHOST), port)
                && free(IpAddr::V6(Ipv6Addr::LOCALHOST), port)
        })
        .collect()
}

/// The port to sign in on: the first one free on both addresses. The retry of
/// a timed-out approval takes the one after it, so the page the first attempt
/// left open can't answer the retry and end it. With none free on both, the
/// registered ports as they are.
pub(super) fn sign_in_ports(registered: &[u16], free: &[u16], retry: bool) -> Vec<u16> {
    match free {
        [] => registered.to_vec(),
        [only] => vec![*only],
        [first, second, ..] => vec![if retry { *second } else { *first }],
    }
}

/// Sign in through the browser exactly as `auth login` does, asking the page
/// for a key. It waits for the approval as `auth login` waits, prints the
/// link to stderr, and stores the sign-in where `auth login` stores it when
/// that place takes it. Returns the access token.
pub(super) fn sign_in(
    ctx: &AppContext,
    authorize_url: &str,
    no_browser: bool,
    retry: bool,
) -> Result<String, CliError> {
    let config = sign_in_config()?;
    // A local --base-url also moves the token endpoint, for testing against a local backend.
    let oauth_base = ctx
        .base_url_override()
        .and_then(loopback)
        .unwrap_or_else(|| config.token_url.trim_end_matches("/v1/oauth/token").into());
    let token_url = format!("{oauth_base}/v1/oauth/token");
    let on = telemetry::enabled();
    let params = authorize_params(
        on.then(|| (platform::detect(), platform::install_source())),
        on.then(platform::agent).flatten(),
        (on && retry).then_some("timed-out"),
    );
    let ports = sign_in_ports(
        &config.redirect_ports,
        &ports_free_on_both(&config.redirect_ports),
        retry,
    );
    // `auth login` saves through the active store; onboard's stands in front of it.
    let store = Arc::new(KeepsUnsaved::over(active_store()));
    set_active_store(store.clone());
    PkceLoginFlow::new(&config.scheme)
        .client_id(&config.client_id)
        .authorization_url(authorize_url)
        .token_url(&token_url)
        .scopes(&config.scopes)
        .redirect_host(&config.redirect_host)
        .redirect_ports(ports)
        .success_redirect_url(&config.success_url)
        .error_redirect_url(&config.error_url)
        .authorization_params(params)
        .run(&LoginContext {
            cli_name: CLI_NAME.to_string(),
            no_browser,
        })?;
    store.access_token().ok_or_else(|| {
        CliError::Other(anyhow::anyhow!(
            "the sign-in finished, but it returned no token"
        ))
    })
}

/// What the sign-in adds to the approval link: the page should hand over a
/// key, and, with telemetry on, where the CLI runs, how it was installed and
/// which coding agent ran it. The page records them for a sign-in that never
/// gets to make an API call. `previous` is how the sign-in this one retries
/// ended, a fixed word.
pub(super) fn authorize_params(
    platform: Option<((&'static str, Option<&'static str>), &'static str)>,
    agent: Option<&'static str>,
    previous: Option<&'static str>,
) -> Vec<(&'static str, String)> {
    let mut params = vec![("key_handoff", "clipboard".to_string())];
    if let Some(((os, env), install)) = platform {
        params.push(("cli_os", os.into()));
        if let Some(env) = env {
            params.push(("cli_env", env.into()));
        }
        params.push(("cli_install", install.into()));
    }
    if let Some(agent) = agent {
        params.push(("cli_agent", agent.into()));
    }
    if let Some(previous) = previous {
        params.push(("cli_prev", previous.into()));
    }
    params
}

/// `Ok(false)` only when the key opens a different account than the sign-in.
/// `Err` when either account can't be identified: a key without `user_read`
/// is refused here, yet may work for everything else. `token` is the
/// sign-in's access token.
pub(super) fn key_matches_sign_in(
    client: &reqwest::Client,
    base: &str,
    token: &str,
    key: &str,
) -> Result<bool, CliError> {
    let unknown = |whose: &str| CliError::Other(anyhow::anyhow!("{whose} could not be identified"));
    let bearer = format!("Bearer {token}");
    let signed_in = account_for(client, base, "authorization", &bearer)?
        .ok_or_else(|| unknown("the signed-in account"))?;
    let owner = account_for(client, base, "xi-api-key", key)?
        .ok_or_else(|| unknown("the key's account"))?;
    Ok(owner == signed_in)
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
