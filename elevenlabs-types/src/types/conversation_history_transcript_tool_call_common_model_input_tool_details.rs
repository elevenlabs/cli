pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
        #[serde(rename = "api_integration_webhook")]
        #[non_exhaustive]
        ApiIntegrationWebhook {
            #[serde(skip_serializing_if = "Option::is_none")]
            integration_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            credential_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            integration_connection_id: Option<String>,
            #[serde(default)]
            webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails,
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

impl ConversationHistoryTranscriptToolCallCommonModelInputToolDetails {
    pub fn api_integration_webhook(webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails) -> Self {
        Self::ApiIntegrationWebhook { integration_id: None, credential_id: None, integration_connection_id: None, webhook_details }
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

    pub fn api_integration_webhook_with_integration_id(integration_id: String, credential_id: Option<String>, integration_connection_id: Option<String>, webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails) -> Self {
        Self::ApiIntegrationWebhook { integration_id: Some(integration_id), credential_id, integration_connection_id, webhook_details }
    }

    pub fn api_integration_webhook_with_credential_id(integration_id: Option<String>, credential_id: String, integration_connection_id: Option<String>, webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails) -> Self {
        Self::ApiIntegrationWebhook { integration_id, credential_id: Some(credential_id), integration_connection_id, webhook_details }
    }

    pub fn api_integration_webhook_with_integration_connection_id(integration_id: Option<String>, credential_id: Option<String>, integration_connection_id: String, webhook_details: ConversationHistoryTranscriptToolCallWebhookDetails) -> Self {
        Self::ApiIntegrationWebhook { integration_id, credential_id, integration_connection_id: Some(integration_connection_id), webhook_details }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
