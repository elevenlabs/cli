//! Token sources: each call returns one fresh source ID token (a compact JWS).

use std::io::Read;
use std::path::PathBuf;

use base64::Engine;
use reqwest::Url;

use super::error::{describe_http_error, source_unavailable, WifError};

pub const MAX_SOURCE_TOKEN_BYTES: usize = 16 * 1024;

const GCE_METADATA_HOST_ENV: &str = "GCE_METADATA_HOST";
const GCE_METADATA_DEFAULT_HOST: &str = "metadata.google.internal";
const AZURE_IMDS_ENV: &str = "AZURE_POD_IDENTITY_AUTHORITY_HOST";
const AZURE_IMDS_DEFAULT: &str = "http://169.254.169.254";

#[derive(Debug)]
pub enum TokenSource {
    File {
        path: PathBuf,
        json_field: Option<String>,
    },
    Env {
        name: String,
    },
    GitHubActions {
        audience: String,
    },
    Gcp {
        audience: String,
    },
    Azure {
        audience: String,
        client_id: Option<String>,
    },
    Aws {
        audience: String,
    },
}

impl TokenSource {
    pub fn describe(&self) -> String {
        match self {
            TokenSource::File { path, .. } => format!("file source {}", path.display()),
            TokenSource::Env { name } => format!("env source {name}"),
            TokenSource::GitHubActions { .. } => "github_actions source".into(),
            TokenSource::Gcp { .. } => "gcp source".into(),
            TokenSource::Azure { .. } => "azure source".into(),
            TokenSource::Aws { .. } => "aws source".into(),
        }
    }

    pub async fn fetch(&self, http: &reqwest::Client) -> Result<String, WifError> {
        let raw = match self {
            TokenSource::File { json_field, .. } => self.read_file(json_field.as_deref())?,
            TokenSource::Env { name } => super::env::var(name)
                .ok_or_else(|| source_unavailable(format!("{name} is unset or empty")))?,
            TokenSource::GitHubActions { audience } => self.github(http, audience).await?,
            TokenSource::Gcp { audience } => self.gcp(http, audience).await?,
            TokenSource::Azure {
                audience,
                client_id,
            } => self.azure(http, audience, client_id.as_deref()).await?,
            TokenSource::Aws { audience } => super::aws::fetch(http, audience).await?,
        };
        check_jws(&self.describe(), &raw)
    }

    fn read_file(&self, json_field: Option<&str>) -> Result<String, WifError> {
        let TokenSource::File { path, .. } = self else {
            unreachable!()
        };
        let fail = |cause: String| source_unavailable(format!("{}{cause}", self.describe()));
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        // Non-blocking open, then fstat: a FIFO or device is rejected without ever blocking.
        #[cfg(unix)]
        std::os::unix::fs::OpenOptionsExt::custom_flags(&mut options, libc::O_NONBLOCK);
        let file = options.open(path).map_err(|e| fail(format!(": {e}")))?;
        let meta = file.metadata().map_err(|e| fail(format!(": {e}")))?;
        if !meta.is_file() {
            return Err(fail(" is not a regular file".into()));
        }
        let mut data = Vec::new();
        file.take(MAX_SOURCE_TOKEN_BYTES as u64 + 1)
            .read_to_end(&mut data)
            .map_err(|e| fail(format!(": {e}")))?;
        if data.len() > MAX_SOURCE_TOKEN_BYTES {
            return Err(fail(" is over 16 KB".into()));
        }
        let text = String::from_utf8_lossy(&data).into_owned();
        match json_field {
            None => Ok(text),
            Some(field) => serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| v.get(field)?.as_str().map(str::to_owned))
                .ok_or_else(|| fail(format!(" has no string field {field}"))),
        }
    }

    async fn github(&self, http: &reqwest::Client, audience: &str) -> Result<String, WifError> {
        let url = super::env::var("ACTIONS_ID_TOKEN_REQUEST_URL");
        let token = super::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
        let (url, token) = match (url, token) {
            (Some(u), Some(t)) => (u, t),
            (u, t) => {
                let missing: Vec<&str> = [
                    ("ACTIONS_ID_TOKEN_REQUEST_URL", u.is_none()),
                    ("ACTIONS_ID_TOKEN_REQUEST_TOKEN", t.is_none()),
                ]
                .iter()
                .filter(|(_, m)| *m)
                .map(|(n, _)| *n)
                .collect();
                return Err(source_unavailable(format!(
                    "{} not set",
                    missing.join(" and ")
                )));
            }
        };
        let mut url = Url::parse(&url)
            .map_err(|_| source_unavailable("ACTIONS_ID_TOKEN_REQUEST_URL is not a URL"))?;
        url.query_pairs_mut().append_pair("audience", audience);
        let request = http.get(url).bearer_auth(token);
        let body = get(request, &self.describe()).await?;
        json_string(&body, "value")
            .ok_or_else(|| source_unavailable(format!("{} response has no value", self.describe())))
    }

    async fn gcp(&self, http: &reqwest::Client, audience: &str) -> Result<String, WifError> {
        let host = env_or(GCE_METADATA_HOST_ENV, GCE_METADATA_DEFAULT_HOST);
        let mut url = Url::parse(&format!(
            "http://{host}/computeMetadata/v1/instance/service-accounts/default/identity"
        ))
        .map_err(|_| source_unavailable(format!("{GCE_METADATA_HOST_ENV} is not a host")))?;
        url.query_pairs_mut()
            .append_pair("audience", audience)
            .append_pair("format", "full");
        get(
            http.get(url).header("Metadata-Flavor", "Google"),
            &self.describe(),
        )
        .await
    }

    async fn azure(
        &self,
        http: &reqwest::Client,
        audience: &str,
        client_id: Option<&str>,
    ) -> Result<String, WifError> {
        let base = env_or(AZURE_IMDS_ENV, AZURE_IMDS_DEFAULT);
        let mut url = Url::parse(&format!(
            "{}/metadata/identity/oauth2/token",
            base.trim_end_matches('/')
        ))
        .map_err(|_| source_unavailable(format!("{AZURE_IMDS_ENV} is not a URL")))?;
        url.query_pairs_mut()
            .append_pair("api-version", "2018-02-01")
            .append_pair("resource", audience);
        if let Some(id) = client_id {
            url.query_pairs_mut().append_pair("client_id", id);
        }
        let body = get(http.get(url).header("Metadata", "true"), &self.describe()).await?;
        json_string(&body, "access_token").ok_or_else(|| {
            source_unavailable(format!("{} response has no access_token", self.describe()))
        })
    }
}

fn env_or(name: &str, default: &str) -> String {
    super::env::var(name).unwrap_or_else(|| default.to_string())
}

fn json_string(body: &str, field: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()?
        .get(field)?
        .as_str()
        .map(str::to_owned)
}

/// Sends a source request and returns the 200 body; other statuses are source_token_unavailable.
pub async fn get(request: reqwest::RequestBuilder, at: &str) -> Result<String, WifError> {
    let response = request
        .send()
        .await
        .map_err(|e| source_unavailable(format!("{at}: {}", describe_http_error(&e))))?;
    let status = response.status().as_u16();
    if status != 200 {
        return Err(source_unavailable(format!("{at} answered HTTP {status}")));
    }
    response
        .text()
        .await
        .map_err(|e| source_unavailable(format!("{at}: {}", describe_http_error(&e))))
}

/// Trims and checks the value is one compact JWS under 16 KB, without quoting it on failure.
pub fn check_jws(at: &str, value: &str) -> Result<String, WifError> {
    let token = value.trim();
    if token.len() > MAX_SOURCE_TOKEN_BYTES {
        return Err(source_unavailable(format!(
            "{at} returned a token over 16 KB"
        )));
    }
    let parts: Vec<&str> = token.split('.').collect();
    let base64url = |p: &&str| {
        !p.is_empty()
            && p.bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            && base64::engine::general_purpose::URL_SAFE_NO_PAD
                .decode(p)
                .is_ok()
    };
    if parts.len() != 3 || !parts.iter().all(base64url) {
        return Err(source_unavailable(format!(
            "{at} did not return a single JWS"
        )));
    }
    Ok(token.to_string())
}
