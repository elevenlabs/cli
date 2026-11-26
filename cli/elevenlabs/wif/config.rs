//! The WIF config file: identifiers only, unknown fields rejected.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::{Map, Value};

use super::error::{config_invalid, WifError};
use super::sources::TokenSource;

pub const CONFIG_ENV: &str = "ELEVENLABS_WIF_CONFIG";
const CONFIG_TYPE: &str = "elevenlabs_workload_identity_federation";
const POOL_URN_PREFIX: &str = "urn:elevenlabs-xi:workload_identity_federation:";
const RESOURCE_URN_PREFIX: &str = "urn:elevenlabs-xi:api_key:";

const TOP_LEVEL_FIELDS: &[&str] = &[
    "type",
    "version",
    "pool",
    "resource",
    "scope",
    "token_source",
];

#[derive(Debug)]
pub struct WifConfig {
    pub pool: String,
    pub resource: String,
    pub scope: Option<String>,
    pub source: Arc<TokenSource>,
}

pub fn load(path: &Path) -> Result<WifConfig, WifError> {
    let shown = path.display();
    let text = std::fs::read_to_string(path)
        .map_err(|e| config_invalid(format!("cannot read {shown}: {e}")))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|_| config_invalid(format!("{shown} is not valid JSON")))?;
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| config_invalid(format!("cannot resolve {shown}: {e}")))?
            .join(path)
    };
    parse(&value, absolute.parent().unwrap_or(Path::new("/")))
}

pub fn parse(value: &Value, base_dir: &Path) -> Result<WifConfig, WifError> {
    let obj = value
        .as_object()
        .ok_or_else(|| config_invalid("the config must be a JSON object"))?;
    reject_unknown(obj, TOP_LEVEL_FIELDS, "config")?;
    if obj.get("type").and_then(Value::as_str) != Some(CONFIG_TYPE) {
        return Err(config_invalid(format!("type must be {CONFIG_TYPE}")));
    }
    if obj.get("version").and_then(Value::as_u64) != Some(1) {
        return Err(config_invalid("version must be 1"));
    }
    let pool = required(obj, "pool", "")?;
    let resource = required(obj, "resource", "")?;
    require_prefix(&pool, "pool", POOL_URN_PREFIX)?;
    require_prefix(&resource, "resource", RESOURCE_URN_PREFIX)?;
    let scope = optional(obj, "scope", "")?;
    let source = obj
        .get("token_source")
        .and_then(Value::as_object)
        .ok_or_else(|| config_invalid("token_source is required"))?;
    Ok(WifConfig {
        source: Arc::new(token_source(source, &pool, base_dir)?),
        pool,
        resource,
        scope,
    })
}

fn token_source(
    src: &Map<String, Value>,
    pool: &str,
    base_dir: &Path,
) -> Result<TokenSource, WifError> {
    const P: &str = "token_source.";
    let kind = src.get("type").and_then(Value::as_str).unwrap_or_default();
    let allowed: &[&str] = match kind {
        "file" => &["type", "path", "format", "json_field"],
        "env" => &["type", "name"],
        "github_actions" | "gcp" | "aws" => &["type", "audience"],
        "azure" => &["type", "audience", "client_id"],
        _ => {
            return Err(config_invalid(
                "token_source.type must be one of aws, azure, env, file, gcp, github_actions",
            ))
        }
    };
    reject_unknown(src, allowed, "token_source")?;
    let audience = optional(src, "audience", P)?.unwrap_or_else(|| pool.to_string());
    Ok(match kind {
        "file" => {
            let path = PathBuf::from(required(src, "path", P)?);
            let path = if path.is_absolute() {
                path
            } else {
                base_dir.join(path)
            };
            let json_field = match optional(src, "format", P)?.as_deref() {
                None | Some("text") => None,
                Some("json") => Some(required(src, "json_field", P)?),
                Some(_) => return Err(config_invalid("token_source.format must be text or json")),
            };
            TokenSource::File { path, json_field }
        }
        "env" => TokenSource::Env {
            name: required(src, "name", P)?,
        },
        "github_actions" => TokenSource::GitHubActions { audience },
        "gcp" => TokenSource::Gcp { audience },
        "azure" => TokenSource::Azure {
            audience,
            client_id: optional(src, "client_id", P)?,
        },
        _ => TokenSource::Aws { audience },
    })
}

fn reject_unknown(obj: &Map<String, Value>, allowed: &[&str], at: &str) -> Result<(), WifError> {
    let mut unknown: Vec<&String> = obj
        .keys()
        .filter(|k| !allowed.contains(&k.as_str()))
        .collect();
    unknown.sort();
    match unknown.first() {
        Some(k) => Err(config_invalid(format!("unknown field {at}.{k}"))),
        None => Ok(()),
    }
}

fn require_prefix(value: &str, field: &str, prefix: &str) -> Result<(), WifError> {
    if value.len() > prefix.len() && value.starts_with(prefix) {
        Ok(())
    } else {
        Err(config_invalid(format!("{field} must start with {prefix}")))
    }
}

fn required(obj: &Map<String, Value>, key: &str, prefix: &str) -> Result<String, WifError> {
    optional(obj, key, prefix)?.ok_or_else(|| config_invalid(format!("{prefix}{key} is required")))
}

fn optional(obj: &Map<String, Value>, key: &str, prefix: &str) -> Result<Option<String>, WifError> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if !s.is_empty() => Ok(Some(s.clone())),
        Some(_) => Err(config_invalid(format!(
            "{prefix}{key} must be a non-empty string"
        ))),
    }
}
