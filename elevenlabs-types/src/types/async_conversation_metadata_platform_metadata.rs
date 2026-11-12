pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "conversation_initiation_source")]
#[non_exhaustive]
pub enum AsyncConversationMetadataPlatformMetadata {
        #[serde(rename = "plain_integration")]
        #[non_exhaustive]
        PlainIntegration {
            #[serde(default)]
            thread_id: String,
            #[serde(default)]
            workspace_id: String,
        },

        #[serde(rename = "zendesk_integration")]
        #[non_exhaustive]
        ZendeskIntegration {
            #[serde(default)]
            ticket_id: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AsyncConversationMetadataPlatformMetadata {
    pub fn plain_integration(thread_id: String, workspace_id: String) -> Self {
        Self::PlainIntegration { thread_id, workspace_id }
    }

    pub fn zendesk_integration(ticket_id: String) -> Self {
        Self::ZendeskIntegration { ticket_id }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
