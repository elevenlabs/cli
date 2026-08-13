pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ToolResponseModelToolConfig {
        #[serde(rename = "client")]
        #[non_exhaustive]
        Client {
            #[serde(flatten)]
            data: ClientToolConfigOutput,
        },

        #[serde(rename = "mcp")]
        #[non_exhaustive]
        Mcp {
            value: serde_json::Value,
        },

        #[serde(rename = "system")]
        #[non_exhaustive]
        System {
            #[serde(flatten)]
            data: SystemToolConfigOutput,
        },

        #[serde(rename = "webhook")]
        #[non_exhaustive]
        Webhook {
            #[serde(flatten)]
            data: WebhookToolConfigOutput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ToolResponseModelToolConfig {
    pub fn client(data: ClientToolConfigOutput) -> Self {
        Self::Client { data }
    }

    pub fn mcp(value: serde_json::Value) -> Self {
        Self::Mcp { value }
    }

    pub fn system(data: SystemToolConfigOutput) -> Self {
        Self::System { data }
    }

    pub fn webhook(data: WebhookToolConfigOutput) -> Self {
        Self::Webhook { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
