pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AlertingSettingsResponseNotifiersItem {
        #[serde(rename = "webhook")]
        #[non_exhaustive]
        Webhook {
            #[serde(default)]
            webhook_id: String,
        },

        #[serde(rename = "integration")]
        #[non_exhaustive]
        Integration {
            #[serde(skip_serializing_if = "Option::is_none")]
            integration_type: Option<AlertingIntegrationNotifierResponseIntegrationType>,
            #[serde(default)]
            connection_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            channel_id: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AlertingSettingsResponseNotifiersItem {
    pub fn webhook(webhook_id: String) -> Self {
        Self::Webhook { webhook_id }
    }

    pub fn integration(connection_id: String) -> Self {
        Self::Integration { integration_type: None, connection_id, channel_id: None }
    }

    pub fn integration_with_integration_type(integration_type: AlertingIntegrationNotifierResponseIntegrationType, connection_id: String, channel_id: Option<String>) -> Self {
        Self::Integration { integration_type: Some(integration_type), connection_id, channel_id }
    }

    pub fn integration_with_channel_id(integration_type: Option<AlertingIntegrationNotifierResponseIntegrationType>, connection_id: String, channel_id: String) -> Self {
        Self::Integration { integration_type, connection_id, channel_id: Some(channel_id) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
