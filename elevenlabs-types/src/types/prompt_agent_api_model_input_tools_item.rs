pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PromptAgentApiModelInputToolsItem {
        #[serde(rename = "api_integration_webhook")]
        #[non_exhaustive]
        ApiIntegrationWebhook {
            #[serde(flatten)]
            data: ApiIntegrationWebhookToolConfigInput,
        },

        #[serde(rename = "client")]
        #[non_exhaustive]
        Client {
            #[serde(flatten)]
            data: ClientToolConfigInput,
        },

        #[serde(rename = "mcp")]
        #[non_exhaustive]
        Mcp {
            value: serde_json::Value,
        },

        #[serde(rename = "smb")]
        #[non_exhaustive]
        Smb {
            value: serde_json::Value,
        },

        #[serde(rename = "system")]
        #[non_exhaustive]
        System {
            #[serde(flatten)]
            data: SystemToolConfigInput,
        },

        #[serde(rename = "webhook")]
        #[non_exhaustive]
        Webhook {
            #[serde(flatten)]
            data: WebhookToolConfigInput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl PromptAgentApiModelInputToolsItem {
    pub fn api_integration_webhook(data: ApiIntegrationWebhookToolConfigInput) -> Self {
        Self::ApiIntegrationWebhook { data }
    }

    pub fn client(data: ClientToolConfigInput) -> Self {
        Self::Client { data }
    }

    pub fn mcp(value: serde_json::Value) -> Self {
        Self::Mcp { value }
    }

    pub fn smb(value: serde_json::Value) -> Self {
        Self::Smb { value }
    }

    pub fn system(data: SystemToolConfigInput) -> Self {
        Self::System { data }
    }

    pub fn webhook(data: WebhookToolConfigInput) -> Self {
        Self::Webhook { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
