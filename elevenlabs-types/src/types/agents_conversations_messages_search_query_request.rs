pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsMessagesSearchQueryRequest {
    /// The search query text for semantic similarity matching
    #[serde(default)]
    pub text_query: String,
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Number of results per page. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsConversationsMessagesSearchQueryRequest {
    pub fn builder() -> AgentsConversationsMessagesSearchQueryRequestBuilder {
        <AgentsConversationsMessagesSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsMessagesSearchQueryRequestBuilder {
    text_query: Option<String>,
    agent_id: Option<String>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsConversationsMessagesSearchQueryRequestBuilder {
    pub fn text_query(mut self, value: impl Into<String>) -> Self {
        self.text_query = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsMessagesSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_query`](AgentsConversationsMessagesSearchQueryRequestBuilder::text_query)
    pub fn build(self) -> Result<AgentsConversationsMessagesSearchQueryRequest, BuildError> {
        Ok(AgentsConversationsMessagesSearchQueryRequest {
            text_query: self.text_query.ok_or_else(|| BuildError::missing_field("text_query"))?,
            agent_id: self.agent_id,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

