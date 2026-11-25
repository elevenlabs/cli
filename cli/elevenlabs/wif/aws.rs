//! AWS source: STS GetWebIdentityToken signed with SigV4, without the AWS SDK.

use hmac::{Hmac, Mac};
use reqwest::Url;
use sha2::{Digest, Sha256};

use super::error::{describe_http_error, source_unavailable, WifError};
use super::sources::get;
use super::urls::require_https_or_loopback;

const IMDS_DEFAULT: &str = "http://169.254.169.254/";
const ECS_RELATIVE_HOST: &str = "http://169.254.170.2";
const STS_DURATION_SECONDS: &str = "300";

struct Credentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: Option<String>,
}

pub async fn fetch(http: &reqwest::Client, audience: &str) -> Result<String, WifError> {
    let mut imds = Imds::new(http);
    let region = match env("AWS_REGION").or_else(|| env("AWS_DEFAULT_REGION")) {
        Some(r) => r,
        None => imds.get("meta-data/placement/region").await?,
    };
    let credentials = credentials(http, &mut imds).await?;
    let endpoint = env("AWS_ENDPOINT_URL_STS")
        .unwrap_or_else(|| format!("https://sts.{region}.amazonaws.com"));
    require_https_or_loopback(&endpoint, "AWS_ENDPOINT_URL_STS")?;
    let url =
        Url::parse(&endpoint).map_err(|_| source_unavailable("the STS endpoint is not a URL"))?;

    let body = form_urlencoded::Serializer::new(String::new())
        .append_pair("Action", "GetWebIdentityToken")
        .append_pair("Version", "2011-06-15")
        .append_pair("Audience.member.1", audience)
        .append_pair("SigningAlgorithm", "RS256")
        .append_pair("DurationSeconds", STS_DURATION_SECONDS)
        .finish();
    let mut request = http.post(url.clone()).body(body.clone());
    for (name, value) in sign(&url, body.as_bytes(), &credentials, &region) {
        request = request.header(name, value);
    }
    let xml = get(request, "STS").await?;
    let open = "WebIdentityToken>";
    xml.match_indices(open)
        .filter_map(|(i, _)| {
            let start = i + open.len();
            // Only an opening tag counts: `<…WebIdentityToken>` not `</…WebIdentityToken>`.
            let tag_start = xml[..i].rfind('<')?;
            if xml[tag_start..i].starts_with("</") {
                return None;
            }
            let end = xml[start..].find('<')? + start;
            Some(xml[start..end].trim().to_string()).filter(|t| !t.is_empty())
        })
        .next()
        .ok_or_else(|| source_unavailable("STS response has no WebIdentityToken"))
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

/// EC2 instance metadata, IMDSv2 only; the session token is fetched on first use.
struct Imds<'a> {
    http: &'a reqwest::Client,
    base: String,
    token: Option<String>,
}

impl<'a> Imds<'a> {
    fn new(http: &'a reqwest::Client) -> Self {
        let base =
            env("AWS_EC2_METADATA_SERVICE_ENDPOINT").unwrap_or_else(|| IMDS_DEFAULT.to_string());
        Self {
            http,
            base: format!("{}/latest/", base.trim_end_matches('/')),
            token: None,
        }
    }

    async fn get(&mut self, path: &str) -> Result<String, WifError> {
        if self.token.is_none() {
            let request = self
                .http
                .put(format!("{}api/token", self.base))
                .header("X-aws-ec2-metadata-token-ttl-seconds", "300");
            self.token = Some(get(request, "IMDS session token request").await?);
        }
        let request = self.http.get(format!("{}{path}", self.base)).header(
            "X-aws-ec2-metadata-token",
            self.token.as_deref().unwrap_or_default(),
        );
        Ok(get(request, &format!("IMDS {path}"))
            .await?
            .trim()
            .to_string())
    }
}

async fn credentials(http: &reqwest::Client, imds: &mut Imds<'_>) -> Result<Credentials, WifError> {
    if let (Some(id), Some(secret)) = (env("AWS_ACCESS_KEY_ID"), env("AWS_SECRET_ACCESS_KEY")) {
        return Ok(Credentials {
            access_key_id: id,
            secret_access_key: secret,
            session_token: env("AWS_SESSION_TOKEN"),
        });
    }
    let full = env("AWS_CONTAINER_CREDENTIALS_FULL_URI");
    let relative = env("AWS_CONTAINER_CREDENTIALS_RELATIVE_URI");
    if full.is_some() || relative.is_some() {
        let url =
            full.unwrap_or_else(|| format!("{ECS_RELATIVE_HOST}{}", relative.unwrap_or_default()));
        let mut request = http.get(&url);
        if let Some(token) = env("AWS_CONTAINER_AUTHORIZATION_TOKEN") {
            request = request.header("Authorization", token);
        }
        let response = request.send().await.map_err(|e| {
            source_unavailable(format!(
                "ECS credentials endpoint: {}",
                describe_http_error(&e)
            ))
        })?;
        let status = response.status().as_u16();
        if status != 200 {
            return Err(source_unavailable(format!(
                "ECS credentials endpoint answered HTTP {status}"
            )));
        }
        let text = response.text().await.unwrap_or_default();
        return parse_credentials(&text, "ECS credentials");
    }
    let roles = imds.get("meta-data/iam/security-credentials/").await?;
    let role = roles.lines().next().unwrap_or_default().to_string();
    let text = imds
        .get(&format!("meta-data/iam/security-credentials/{role}"))
        .await?;
    parse_credentials(&text, "IMDS credentials")
}

fn parse_credentials(text: &str, at: &str) -> Result<Credentials, WifError> {
    let malformed = || source_unavailable(format!("{at} response is malformed"));
    let v: serde_json::Value = serde_json::from_str(text).map_err(|_| malformed())?;
    let field = |k: &str| v.get(k).and_then(|x| x.as_str()).map(str::to_owned);
    Ok(Credentials {
        access_key_id: field("AccessKeyId").ok_or_else(malformed)?,
        secret_access_key: field("SecretAccessKey").ok_or_else(malformed)?,
        session_token: field("Token"),
    })
}

/// SigV4 headers for a form POST; uses wall-clock time, never the WIF test clock.
fn sign(url: &Url, body: &[u8], c: &Credentials, region: &str) -> Vec<(String, String)> {
    let amz_date = amz_date(std::time::SystemTime::now());
    let date = &amz_date[..8];
    let host = match url.port() {
        Some(port) => format!("{}:{port}", url.host_str().unwrap_or_default()),
        None => url.host_str().unwrap_or_default().to_string(),
    };
    let mut headers: Vec<(String, String)> = vec![
        (
            "content-type".into(),
            "application/x-www-form-urlencoded; charset=utf-8".into(),
        ),
        ("host".into(), host),
        ("x-amz-date".into(), amz_date.clone()),
    ];
    if let Some(token) = &c.session_token {
        headers.push(("x-amz-security-token".into(), token.clone()));
    }
    headers.sort();
    let signed: Vec<&str> = headers.iter().map(|(n, _)| n.as_str()).collect();
    let signed = signed.join(";");
    let canonical_headers: String = headers
        .iter()
        .map(|(n, v)| {
            format!(
                "{n}:{}\n",
                v.split_whitespace().collect::<Vec<_>>().join(" ")
            )
        })
        .collect();
    let path = if url.path().is_empty() {
        "/"
    } else {
        url.path()
    };
    let canonical_request = [
        "POST",
        path,
        "",
        &canonical_headers,
        &signed,
        &hex(&Sha256::digest(body)),
    ]
    .join("\n");
    let scope = format!("{date}/{region}/sts/aws4_request");
    let string_to_sign = [
        "AWS4-HMAC-SHA256",
        &amz_date,
        &scope,
        &hex(&Sha256::digest(canonical_request.as_bytes())),
    ]
    .join("\n");
    let mut key = format!("AWS4{}", c.secret_access_key).into_bytes();
    for part in [date, region, "sts", "aws4_request"] {
        key = hmac(&key, part.as_bytes());
    }
    let signature = hex(&hmac(&key, string_to_sign.as_bytes()));
    headers.retain(|(n, _)| n != "host"); // reqwest sets the same value itself.
    headers.push((
        "authorization".into(),
        format!(
            "AWS4-HMAC-SHA256 Credential={}/{scope}, SignedHeaders={signed}, Signature={signature}",
            c.access_key_id
        ),
    ));
    headers
}

fn hmac(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC takes any key length");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// `YYYYMMDDTHHMMSSZ` in UTC.
fn amz_date(t: std::time::SystemTime) -> String {
    let secs = t
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as i64;
    let (days, rem) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}{month:02}{day:02}T{:02}{:02}{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

#[cfg(test)]
mod tests {
    use super::amz_date;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn amz_date_formats_utc() {
        assert_eq!(amz_date(UNIX_EPOCH), "19700101T000000Z");
        assert_eq!(
            amz_date(UNIX_EPOCH + Duration::from_secs(951_782_400)),
            "20000229T000000Z"
        );
        assert_eq!(
            amz_date(UNIX_EPOCH + Duration::from_secs(1_795_600_000)),
            "20261125T094640Z"
        );
    }
}
