//! Workload identity federation: exchanges a workload's OIDC token for a short-lived
//! ElevenLabs token, per the WIF client behavior spec. Hand-written; see `.fernignore`.

mod aws;
mod clock;
mod config;
mod env;
mod error;
mod exchange;
mod sources;
mod token;
mod urls;

#[cfg(test)]
mod conformance_tests;

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, Once, OnceLock};

use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::auth::{AuthProvider, AuthSchemeBuilder, EndpointAuthMetadata, SchemeBinding};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::discovery::{
    GlobalParameter, GlobalParameterApplyMode, GlobalParameterLocation,
};
use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};

use config::{WifConfig, CONFIG_ENV};
use error::{config_invalid, WifError};
use exchange::Exchanger;
use token::TokenManager;

const API_KEY_ENV: &str = "ELEVENLABS_API_KEY";
const WIF_CONFIG_FLAG: &str = "--wif-config";
/// Flags that pass an API key explicitly: the global one and the per-operation header parameter.
const API_KEY_FLAGS: &[&str] = &["--api-key", "--xi-api-key"];
const SCHEME_NAME: &str = "WorkloadIdentityFederation";
const CLI_NAME: &str = "elevenlabs";
/// The scheme name the generated `main.rs` gives its `PkceLoginFlow`.
const OAUTH_SCHEME: &str = "OAuth";

/// The WIF credentials this process uses, once precedence picked them.
static ACTIVE: OnceLock<Arc<WifAuth>> = OnceLock::new();

/// Adds the `--wif-config` flag, the WIF auth scheme when WIF wins precedence, and `wif print-identity-token`.
pub fn register(app: CliApp) -> CliApp {
    let app = app
        .global_parameter(GlobalParameter {
            name: "wif-config".into(),
            location: GlobalParameterLocation::Header,
            // Explicit mode with no operation opting in: the flag is parsed but never sent.
            target: "x-elevenlabs-wif-config".into(),
            env: None,
            default: None,
            optional: true,
            apply: GlobalParameterApplyMode::Explicit,
            parameter_name: Some("wif-config".into()),
            docs: Some(format!(
                "Path to a workload identity federation config (overrides {CONFIG_ENV})"
            )),
        })
        .command_under(
            &["wif"],
            clap::Command::new("print-identity-token")
                .about("Print an ElevenLabs access token minted from the WIF config"),
            OpenApiBinding::handler(print_identity_token),
        )
        .describe(&["wif"], "Workload identity federation");

    // Before anything else, and before `CliApp::run` loads `.env`: WIF never reads that file.
    env::capture();
    let argv: Vec<OsString> = std::env::args_os().collect();
    // `auth login/logout/status` list the raw bindings and would see the login scheme twice.
    // They never send an API request, so WIF stays out of them.
    if first_positional(&argv).as_deref() == Some("auth") {
        return app;
    }
    match select(&argv, &env::var) {
        Ok(Selection::None) => app,
        Ok(Selection::Wif { path }) => {
            shadow_env_api_key();
            let auth = Arc::new(WifAuth::new(path));
            let _ = ACTIVE.set(auth.clone());
            app.auth(WifScheme(auth))
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(CliError::EXIT_CODE_AUTH);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Selection {
    None,
    Wif { path: PathBuf },
}

/// Precedence: an explicit flag, then ELEVENLABS_API_KEY, then ELEVENLABS_WIF_CONFIG.
fn select(argv: &[OsString], env: &dyn Fn(&str) -> Option<String>) -> Result<Selection, WifError> {
    let flag_config = flag_value(argv, WIF_CONFIG_FLAG);
    let flag_key = API_KEY_FLAGS.iter().any(|f| flag_value(argv, f).is_some());
    if let Some(path) = flag_config {
        if flag_key {
            return Err(config_invalid(
                "pass either --api-key or --wif-config, not both",
            ));
        }
        return Ok(Selection::Wif {
            path: PathBuf::from(path),
        });
    }
    if flag_key {
        return Ok(Selection::None);
    }
    let env_config = env(CONFIG_ENV);
    if env(API_KEY_ENV).is_some() {
        if env_config.is_some() {
            warn(&format!("{API_KEY_ENV} is set, so {CONFIG_ENV} is ignored"));
        }
        return Ok(Selection::None);
    }
    Ok(match env_config {
        Some(path) => Selection::Wif {
            path: PathBuf::from(path),
        },
        None => Selection::None,
    })
}

/// Keeps ELEVENLABS_API_KEY from reaching requests once WIF is selected.
///
/// Without it a key from an untrusted `.env` would replace the operator's WIF identity. An empty
/// value rather than removal blocks that, since the CLI's `.env` loader never overrides a set variable.
fn shadow_env_api_key() {
    std::env::set_var(API_KEY_ENV, "");
}

/// The first non-flag argument, skipping flag values like the CLI's own early dispatch does.
///
/// An unlisted boolean flag makes this skip the token after it, which can only miss `auth`, never
/// mistake an API command for it.
fn first_positional(argv: &[OsString]) -> Option<String> {
    const BOOLEAN_FLAGS: &[&str] = &[
        "debug",
        "dry-run",
        "help",
        "human",
        "no-extract",
        "no-pager",
        "no-retry",
        "no-stream",
        "page-all",
        "q",
        "quiet",
        "version",
    ];
    let mut skip_next = false;
    for arg in argv.iter().skip(1).map(|a| a.to_string_lossy()) {
        if std::mem::take(&mut skip_next) {
            continue;
        }
        if arg == "--" {
            return None;
        }
        if let Some(flag) = arg.strip_prefix('-') {
            skip_next =
                !arg.contains('=') && !BOOLEAN_FLAGS.contains(&flag.trim_start_matches('-'));
            continue;
        }
        return Some(arg.into_owned());
    }
    None
}

/// The value of `--flag v` or `--flag=v` before any `--`, if non-empty.
fn flag_value(argv: &[OsString], flag: &str) -> Option<String> {
    let args: Vec<String> = argv
        .iter()
        .skip(1)
        .map(|a| a.to_string_lossy().into_owned())
        .take_while(|a| a != "--")
        .collect();
    let prefix = format!("{flag}=");
    args.iter()
        .enumerate()
        .find_map(|(i, a)| {
            if a == flag {
                args.get(i + 1).cloned()
            } else {
                a.strip_prefix(&prefix).map(str::to_owned)
            }
        })
        .filter(|v| !v.is_empty())
}

/// A warning the user should see even with logging off (the CLI logs to stderr only on request).
fn warn(message: &str) {
    tracing::warn!("{message}");
    eprintln!("warning: {message}");
}

/// The selected WIF config; read once on first use so errors surface there, before any request.
#[derive(Debug)]
pub struct WifAuth {
    path: PathBuf,
    loaded: OnceLock<Result<Arc<Loaded>, WifError>>,
}

#[derive(Debug)]
struct Loaded {
    config: WifConfig,
    /// One token per API origin; the source token only ever goes to the host it authenticates.
    managers: Mutex<HashMap<String, Arc<TokenManager>>>,
    logged: Once,
}

impl WifAuth {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            loaded: OnceLock::new(),
        }
    }

    fn loaded(&self) -> Result<&Arc<Loaded>, WifError> {
        self.loaded
            .get_or_init(|| {
                let config = config::load(&self.path)?;
                Ok(Arc::new(Loaded {
                    config,
                    managers: Mutex::default(),
                    logged: Once::new(),
                }))
            })
            .as_ref()
            .map_err(Clone::clone)
    }

    /// A valid access token for the API at `base_url`.
    pub fn token_for(&self, base_url: &str) -> Result<String, WifError> {
        let loaded = self.loaded()?;
        let origin = origin(base_url)?;
        loaded.logged.call_once(|| {
            tracing::info!(
                "ElevenLabs credentials: WIF config {} (pool {})",
                self.path.display(),
                loaded.config.pool
            );
        });
        let manager = {
            let mut managers = loaded.managers.lock().unwrap_or_else(|p| p.into_inner());
            managers
                .entry(origin.clone())
                .or_insert_with(|| {
                    Arc::new(TokenManager::new(Exchanger {
                        url: format!("{origin}/v1/oidc/token"),
                        pool: loaded.config.pool.clone(),
                        resource: loaded.config.resource.clone(),
                        scope: loaded.config.scope.clone(),
                        source: loaded.config.source.clone(),
                    }))
                })
                .clone()
        };
        manager.get_token()
    }

    /// Puts the WIF token on `request` and drops any `xi-api-key`.
    pub fn authorize(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, CliError> {
        let (client, request) = request.build_split();
        let mut request = request.map_err(|e| CliError::Other(e.into()))?;
        let token = self.token_for(request.url().as_str())?;
        set_bearer(request.headers_mut(), &token)?;
        Ok(reqwest::RequestBuilder::from_parts(client, request))
    }
}

fn set_bearer(headers: &mut reqwest::header::HeaderMap, token: &str) -> Result<(), WifError> {
    let mut value = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
        .map_err(|_| error::exchange_unavailable("malformed access token"))?;
    value.set_sensitive(true);
    headers.remove("xi-api-key");
    headers.insert(reqwest::header::AUTHORIZATION, value);
    Ok(())
}

/// `scheme://host[:port]` of an https (or loopback http) URL.
fn origin(url: &str) -> Result<String, WifError> {
    urls::require_https_or_loopback(url, "the API base URL")?;
    let u =
        reqwest::Url::parse(url).map_err(|_| config_invalid("the API base URL is not a URL"))?;
    Ok(u.origin().ascii_serialization())
}

struct WifScheme(Arc<WifAuth>);

impl AuthSchemeBuilder for WifScheme {
    /// Bound under the OAuth login's scheme: the builder keeps the last binding of a name, so this
    /// replaces the stored-login provider for requests (which would otherwise win, being first),
    /// while `auth login`, `logout` and `status` keep working through the login flow.
    fn into_binding(self) -> (String, SchemeBinding) {
        (
            OAUTH_SCHEME.to_string(),
            SchemeBinding::Custom(Arc::new(WifAuthProvider(self.0))),
        )
    }
}

/// Whether `elevenlabs auth login` stored a credential for the active profile.
fn has_stored_login() -> bool {
    fern_cli_sdk::auth::active_store()
        .get(
            CLI_NAME,
            &fern_cli_sdk::profiles::keyring_account(OAUTH_SCHEME),
        )
        .ok()
        .flatten()
        .is_some_and(|v| !v.is_empty())
}

/// Refuses to choose between a WIF config and a stored login: either may be the identity meant.
fn check_no_stored_login() -> Result<(), WifError> {
    static STORED_LOGIN: OnceLock<bool> = OnceLock::new();
    if *STORED_LOGIN.get_or_init(has_stored_login) {
        return Err(config_invalid(format!(
            "a WIF config is set and so is an `{CLI_NAME} auth login` session; run `{CLI_NAME} auth logout` or drop the WIF config"
        )));
    }
    Ok(())
}

#[derive(Debug)]
struct WifAuthProvider(Arc<WifAuth>);

impl AuthProvider for WifAuthProvider {
    fn name(&self) -> &str {
        SCHEME_NAME
    }

    /// Registered only when WIF won precedence, so it always claims the request (fail closed).
    fn has_credentials(&self) -> bool {
        true
    }

    fn credential_hints(&self) -> Vec<String> {
        vec![format!(
            "WIF config {} ({WIF_CONFIG_FLAG} or {CONFIG_ENV})",
            self.0.path.display()
        )]
    }

    fn apply(
        &self,
        request: reqwest::RequestBuilder,
        _endpoint: &EndpointAuthMetadata,
    ) -> Result<reqwest::RequestBuilder, CliError> {
        check_no_stored_login()?;
        self.0.authorize(request)
    }
}

/// `elevenlabs wif print-identity-token`: the only command allowed to print a token.
fn print_identity_token(_matches: &clap::ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let auth = ACTIVE.get().ok_or_else(|| {
        CliError::Auth(format!(
            "ElevenLabs WIF config_invalid: no WIF config; pass {WIF_CONFIG_FLAG} or set {CONFIG_ENV} (and unset {API_KEY_ENV})"
        ))
    })?;
    println!("{}", auth.token_for(&ctx.effective_base_url())?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fern_cli_sdk::auth::{active_store, set_active_store, KeyringStore, MockKeyringStore};

    #[test]
    fn first_positional_skips_flags_and_their_values() {
        assert_eq!(
            first_positional(&argv(&["auth", "logout"])).as_deref(),
            Some("auth")
        );
        assert_eq!(
            first_positional(&argv(&[
                "--wif-config",
                "w.json",
                "--human",
                "auth",
                "status"
            ]))
            .as_deref(),
            Some("auth")
        );
        assert_eq!(
            first_positional(&argv(&["--format=json", "user", "get"])).as_deref(),
            Some("user")
        );
        // A flag value that happens to read "auth" is not the command.
        assert_eq!(
            first_positional(&argv(&["--base-url", "auth", "user"])).as_deref(),
            Some("user")
        );
        assert_eq!(first_positional(&argv(&["--", "auth"])), None);
    }

    /// WIF replaces the stored-login provider only while it binds the same scheme name.
    #[test]
    fn generated_main_still_names_the_login_scheme_oauth() {
        let main = include_str!("../main.rs");
        assert!(
            main.contains(&format!("PkceLoginFlow::new(\"{OAUTH_SCHEME}\")")),
            "the generated login scheme was renamed; update OAUTH_SCHEME"
        );
    }

    #[test]
    #[serial_test::serial]
    fn a_stored_login_is_detected() {
        let previous = active_store();
        let store = Arc::new(MockKeyringStore::new());
        set_active_store(store.clone());
        let before = has_stored_login();
        store
            .set(CLI_NAME, OAUTH_SCHEME, "{\"access_token\":\"t\"}")
            .unwrap();
        let after = has_stored_login();
        set_active_store(previous);
        assert!(!before && after, "before {before}, after {after}");
    }

    fn argv(args: &[&str]) -> Vec<OsString> {
        std::iter::once("elevenlabs")
            .chain(args.iter().copied())
            .map(OsString::from)
            .collect()
    }

    fn env<'a>(vars: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            vars.iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn explicit_flag_wins_over_env_key() {
        let got = select(
            &argv(&["user", "get", "--wif-config", "w.json"]),
            &env(&[(API_KEY_ENV, "sk")]),
        );
        assert_eq!(
            got.unwrap(),
            Selection::Wif {
                path: "w.json".into()
            }
        );
        let got = select(&argv(&["--wif-config=w.json", "user", "get"]), &env(&[]));
        assert_eq!(
            got.unwrap(),
            Selection::Wif {
                path: "w.json".into()
            }
        );
    }

    #[test]
    fn two_explicit_credentials_are_config_invalid() {
        for key_flag in API_KEY_FLAGS {
            let err = select(
                &argv(&["--wif-config", "w.json", key_flag, "sk"]),
                &env(&[]),
            )
            .unwrap_err();
            assert_eq!(err.category, error::CONFIG_INVALID);
        }
    }

    #[test]
    fn env_key_wins_over_env_config() {
        let vars = [(API_KEY_ENV, "sk"), (CONFIG_ENV, "w.json")];
        assert_eq!(
            select(&argv(&["user", "get"]), &env(&vars)).unwrap(),
            Selection::None
        );
        let vars = [(CONFIG_ENV, "w.json")];
        assert_eq!(
            select(&argv(&["user", "get"]), &env(&vars)).unwrap(),
            Selection::Wif {
                path: "w.json".into()
            }
        );
        let vars = [(CONFIG_ENV, "w.json")];
        assert_eq!(
            select(&argv(&["--api-key", "sk"]), &env(&vars)).unwrap(),
            Selection::None
        );
    }

    #[test]
    fn flags_after_double_dash_are_ignored() {
        assert_eq!(
            select(&argv(&["say", "--", "--wif-config", "x"]), &env(&[])).unwrap(),
            Selection::None
        );
    }
}
