pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationHistoryTranscriptToolCallCommonModelOutputToolDetails {
        #[serde(rename = "api_integration_webhook")]
        #[non_exhaustive]
        ApiIntegrationWebhook {
            #[serde(flatten)]
            data: ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput,
        },

        #[serde(rename = "client")]
        #[non_exhaustive]
        Client {
            #[serde(flatten)]
            data: ConversationHistoryTranscriptToolCallClientDetails,
        },

        #[serde(rename = "mcp")]
        #[non_exhaustive]
        Mcp {
            #[serde(flatten)]
            data: ConversationHistoryTranscriptToolCallMcpDetails,
        },

        #[serde(rename = "webhook")]
        #[non_exhaustive]
        Webhook {
            #[serde(flatten)]
            data: ConversationHistoryTranscriptToolCallWebhookDetails,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ConversationHistoryTranscriptToolCallCommonModelOutputToolDetails {
    pub fn api_integration_webhook(data: ConversationHistoryTranscriptToolCallApiIntegrationWebhookDetailsOutput) -> Self {
        Self::ApiIntegrationWebhook { data }
    }

    pub fn client(data: ConversationHistoryTranscriptToolCallClientDetails) -> Self {
        Self::Client { data }
    }

    pub fn mcp(data: ConversationHistoryTranscriptToolCallMcpDetails) -> Self {
        Self::Mcp { data }
    }

    pub fn webhook(data: ConversationHistoryTranscriptToolCallWebhookDetails) -> Self {
        Self::Webhook { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
