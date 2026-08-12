pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsTagsListQueryRequest {
    /// How many conversation tags to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsConversationsTagsListQueryRequest {
    pub fn builder() -> AgentsConversationsTagsListQueryRequestBuilder {
        <AgentsConversationsTagsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsTagsListQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsConversationsTagsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsTagsListQueryRequest`].
    pub fn build(self) -> Result<AgentsConversationsTagsListQueryRequest, BuildError> {
        Ok(AgentsConversationsTagsListQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

