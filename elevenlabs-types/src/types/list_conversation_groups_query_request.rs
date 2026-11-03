pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_conversation_groups
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListConversationGroupsQueryRequest {
    /// How many conversation groups to return. Can not exceed 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If specified, only conversation groups whose title or description contains this string (case-insensitive) are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListConversationGroupsQueryRequest {
    pub fn builder() -> ListConversationGroupsQueryRequestBuilder {
        <ListConversationGroupsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConversationGroupsQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    cursor: Option<String>,
}

impl ListConversationGroupsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListConversationGroupsQueryRequest`].
    pub fn build(self) -> Result<ListConversationGroupsQueryRequest, BuildError> {
        Ok(ListConversationGroupsQueryRequest {
            page_size: self.page_size,
            search: self.search,
            cursor: self.cursor,
        })
    }
}

