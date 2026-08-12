pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_sip_messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiConversationsGetSipMessagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ConversationalAiConversationsGetSipMessagesQueryRequest {
    pub fn builder() -> ConversationalAiConversationsGetSipMessagesQueryRequestBuilder {
        <ConversationalAiConversationsGetSipMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiConversationsGetSipMessagesQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl ConversationalAiConversationsGetSipMessagesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiConversationsGetSipMessagesQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiConversationsGetSipMessagesQueryRequest, BuildError> {
        Ok(ConversationalAiConversationsGetSipMessagesQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

