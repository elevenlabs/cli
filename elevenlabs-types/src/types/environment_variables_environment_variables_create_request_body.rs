pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum EnvironmentVariablesCreateRequestBody {
        #[serde(rename = "string")]
        #[non_exhaustive]
        r#String {
            #[serde(default)]
            label: String,
            #[serde(default)]
            values: HashMap<String, String>,
        },

        #[serde(rename = "secret")]
        #[non_exhaustive]
        Secret {
            #[serde(default)]
            label: String,
            #[serde(default)]
            values: HashMap<String, EnvironmentVariableSecretValueRequest>,
        },

        #[serde(rename = "auth_connection")]
        #[non_exhaustive]
        AuthConnection {
            #[serde(default)]
            label: String,
            #[serde(default)]
            values: HashMap<String, EnvironmentVariableAuthConnectionValueRequest>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl EnvironmentVariablesCreateRequestBody {
    pub fn string(label: String, values: HashMap<String, String>) -> Self {
        Self::r#String { label, values }
    }

    pub fn secret(label: String, values: HashMap<String, EnvironmentVariableSecretValueRequest>) -> Self {
        Self::Secret { label, values }
    }

    pub fn auth_connection(label: String, values: HashMap<String, EnvironmentVariableAuthConnectionValueRequest>) -> Self {
        Self::AuthConnection { label, values }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
