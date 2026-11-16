pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SendConversationMessageStreamRequestInputItem {
        #[serde(rename = "client_tool_result")]
        #[non_exhaustive]
        ClientToolResult {
            #[serde(flatten)]
            data: ClientToolResultItem,
        },

        #[serde(rename = "contextual_update")]
        #[non_exhaustive]
        ContextualUpdate {
            #[serde(flatten)]
            data: ContextualUpdateInputItem,
        },

        #[serde(rename = "user_message")]
        #[non_exhaustive]
        UserMessage {
            #[serde(flatten)]
            data: UserMessageInputItem,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SendConversationMessageStreamRequestInputItem {
    pub fn client_tool_result(data: ClientToolResultItem) -> Self {
        Self::ClientToolResult { data }
    }

    pub fn contextual_update(data: ContextualUpdateInputItem) -> Self {
        Self::ContextualUpdate { data }
    }

    pub fn user_message(data: UserMessageInputItem) -> Self {
        Self::UserMessage { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
