pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_sip_messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsGetSipMessagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsConversationsGetSipMessagesQueryRequest {
    pub fn builder() -> AgentsConversationsGetSipMessagesQueryRequestBuilder {
        <AgentsConversationsGetSipMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsGetSipMessagesQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsConversationsGetSipMessagesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsGetSipMessagesQueryRequest`].
    pub fn build(self) -> Result<AgentsConversationsGetSipMessagesQueryRequest, BuildError> {
        Ok(AgentsConversationsGetSipMessagesQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

