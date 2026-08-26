pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AlertingSettingsNotifiersItem {
        #[serde(rename = "integration")]
        #[non_exhaustive]
        Integration {
            #[serde(default)]
            connection_id: String,
        },

        #[serde(rename = "webhook")]
        #[non_exhaustive]
        Webhook {
            #[serde(default)]
            webhook_id: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AlertingSettingsNotifiersItem {
    pub fn integration(connection_id: String) -> Self {
        Self::Integration { connection_id }
    }

    pub fn webhook(webhook_id: String) -> Self {
        Self::Webhook { webhook_id }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
