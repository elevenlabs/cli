//! The RFC 8693 exchange at POST /v1/oidc/token, with the spec's retries.

use std::time::Duration;

use base64::Engine;
use rand::Rng;

use super::error::{describe_http_error, exchange_rejected, exchange_unavailable, WifError};
use super::sources::TokenSource;

const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";
const SUBJECT_TOKEN_TYPE: &str = "urn:ietf:params:oauth:token-type:jwt";
const EXCHANGE_TIMEOUT: Duration = Duration::from_secs(10);
const SOURCE_TIMEOUT: Duration = Duration::from_secs(5);
const TRANSIENT_WAITS: [f64; 2] = [0.5, 1.0];
const RATE_LIMIT_WAITS: [f64; 3] = [1.0, 2.0, 4.0];
const MAX_RETRY_AFTER_SECONDS: f64 = 10.0;
const JITTER: f64 = 0.2;

pub struct ExchangeResult {
    pub access_token: String,
    pub expires_in: f64,
    pub scope: String,
}

#[derive(Debug)]
pub struct Exchanger {
    pub url: String,
    pub pool: String,
    pub resource: String,
    pub scope: Option<String>,
    pub source: std::sync::Arc<TokenSource>,
}

enum Failure {
    /// Retried twice: network errors, timeouts, 5xx and source failures.
    Transient(WifError),
    RateLimited(Option<f64>),
    Final(WifError),
}

impl Exchanger {
    pub async fn exchange(&self) -> Result<ExchangeResult, WifError> {
        let exchange_http = client(EXCHANGE_TIMEOUT)?;
        let source_http = client(SOURCE_TIMEOUT)?;
        let (mut transient, mut rate_limited) = (0, 0);
        loop {
            let wait = match self.attempt(&exchange_http, &source_http).await {
                Ok(result) => return Ok(result),
                Err(Failure::Final(e)) => return Err(e),
                Err(Failure::RateLimited(retry_after)) => {
                    let Some(base) = RATE_LIMIT_WAITS.get(rate_limited) else {
                        return Err(exchange_unavailable("rate limited (HTTP 429)"));
                    };
                    rate_limited += 1;
                    retry_after.unwrap_or_else(|| jitter(*base))
                }
                Err(Failure::Transient(e)) => {
                    let Some(base) = TRANSIENT_WAITS.get(transient) else {
                        return Err(e);
                    };
                    transient += 1;
                    jitter(*base)
                }
            };
            tokio::time::sleep(Duration::from_secs_f64(wait)).await;
        }
    }

    async fn attempt(
        &self,
        exchange_http: &reqwest::Client,
        source_http: &reqwest::Client,
    ) -> Result<ExchangeResult, Failure> {
        let subject_token = self
            .source
            .fetch(source_http)
            .await
            .map_err(Failure::Transient)?;
        let mut form = vec![
            ("grant_type", GRANT_TYPE),
            ("subject_token", subject_token.as_str()),
            ("subject_token_type", SUBJECT_TOKEN_TYPE),
            ("audience", self.pool.as_str()),
            ("resource", self.resource.as_str()),
        ];
        if let Some(scope) = &self.scope {
            form.push(("scope", scope.as_str()));
        }
        let response = exchange_http
            .post(&self.url)
            .form(&form)
            .send()
            .await
            .map_err(|e| Failure::Transient(exchange_unavailable(describe_http_error(&e))))?;
        let status = response.status().as_u16();
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok()?.trim().parse::<f64>().ok())
            .filter(|v| v.is_finite())
            .map(|v| v.clamp(0.0, MAX_RETRY_AFTER_SECONDS));
        let body = response.text().await.unwrap_or_default();
        match status {
            200 => parse_success(&body).map_err(Failure::Final),
            400 => Err(Failure::Final(rejected(&body, &subject_token))),
            429 => Err(Failure::RateLimited(retry_after)),
            s if s >= 500 => Err(Failure::Transient(exchange_unavailable(format!(
                "HTTP {s}"
            )))),
            s => Err(Failure::Final(exchange_unavailable(format!("HTTP {s}")))),
        }
    }
}

/// No redirects; TLS verification always on, with any CA bundle the CLI's env names added to the roots.
fn client(timeout: Duration) -> Result<reqwest::Client, WifError> {
    let mut builder = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(timeout)
        .user_agent(concat!("elevenlabs-cli-wif/", env!("CARGO_PKG_VERSION")));
    let bundle = [
        "ELEVENLABS_CA_BUNDLE",
        "ELEVENLABS_EXTRA_CA_CERTS",
        "SSL_CERT_FILE",
    ]
    .iter()
    .find_map(|name| std::env::var(name).ok().filter(|v| !v.is_empty()));
    if let Some(path) = bundle {
        let pem = std::fs::read(&path)
            .map_err(|e| exchange_unavailable(format!("cannot read CA bundle {path}: {e}")))?;
        let certs = reqwest::Certificate::from_pem_bundle(&pem)
            .map_err(|_| exchange_unavailable(format!("CA bundle {path} is not PEM")))?;
        for cert in certs {
            builder = builder.add_root_certificate(cert);
        }
    }
    builder.build().map_err(|e| {
        exchange_unavailable(format!(
            "cannot build HTTP client: {}",
            describe_http_error(&e)
        ))
    })
}

fn jitter(seconds: f64) -> f64 {
    seconds * rand::thread_rng().gen_range(1.0 - JITTER..=1.0 + JITTER)
}

fn parse_success(body: &str) -> Result<ExchangeResult, WifError> {
    let malformed = || exchange_unavailable("malformed 200 response");
    let v: serde_json::Value = serde_json::from_str(body).map_err(|_| malformed())?;
    let text = |k: &str| v.get(k).and_then(|x| x.as_str());
    let token = text("access_token")
        .filter(|t| !t.is_empty())
        .ok_or_else(malformed)?;
    let expires_in = v
        .get("expires_in")
        .and_then(|x| x.as_f64())
        .filter(|e| *e > 0.0)
        .ok_or_else(malformed)?;
    if text("issued_token_type").is_none() || text("token_type").is_none() {
        return Err(malformed());
    }
    let scope = v.get("scope").ok_or_else(malformed)?;
    Ok(ExchangeResult {
        access_token: token.to_string(),
        expires_in,
        scope: scope
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| scope.to_string()),
    })
}

fn rejected(body: &str, subject_token: &str) -> WifError {
    let v: Option<serde_json::Value> = serde_json::from_str(body).ok();
    let field = |k: &str| v.as_ref()?.get(k)?.as_str().map(str::to_owned);
    let (error, description) = (field("error"), field("error_description"));
    let shown = |s: &Option<String>| s.clone().unwrap_or_else(|| "None".into());
    let mut message = format!(
        "ElevenLabs WIF token exchange rejected ({}): {}",
        shown(&error),
        shown(&description)
    );
    if let Some(claims) = unverified_claims(subject_token) {
        let claim = |k: &str| match claims.get(k) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(other) => other.to_string(),
            None => "None".into(),
        };
        message.push_str(&format!(
            " Source token: iss={} aud={} sub={}",
            claim("iss"),
            claim("aud"),
            claim("sub")
        ));
    }
    exchange_rejected(message, error, description)
}

/// The source token's payload, decoded without verification, for error messages only.
fn unverified_claims(token: &str) -> Option<serde_json::Map<String, serde_json::Value>> {
    let payload = token.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .ok()?;
    match serde_json::from_slice(&bytes).ok()? {
        serde_json::Value::Object(map) => Some(map),
        _ => None,
    }
}
