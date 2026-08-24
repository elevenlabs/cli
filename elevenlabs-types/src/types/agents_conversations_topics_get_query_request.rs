pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsTopicsGetQueryRequest {
    /// Number of top-level topic groups to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Topic table column to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<TopicSortBy>,
    /// Direction to sort topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Start of the window to view topics for. When set with to_unix_secs, per-day topics in the range are aggregated together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_unix_secs: Option<i64>,
    /// End of the window to view topics for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_unix_secs: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsConversationsTopicsGetQueryRequest {
    pub fn builder() -> AgentsConversationsTopicsGetQueryRequestBuilder {
        <AgentsConversationsTopicsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsTopicsGetQueryRequestBuilder {
    page_size: Option<i64>,
    sort_by: Option<TopicSortBy>,
    sort_direction: Option<SortDirection>,
    from_unix_secs: Option<i64>,
    to_unix_secs: Option<i64>,
    cursor: Option<String>,
}

impl AgentsConversationsTopicsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort_by(mut self, value: TopicSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn from_unix_secs(mut self, value: i64) -> Self {
        self.from_unix_secs = Some(value);
        self
    }

    pub fn to_unix_secs(mut self, value: i64) -> Self {
        self.to_unix_secs = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsTopicsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsConversationsTopicsGetQueryRequest, BuildError> {
        Ok(AgentsConversationsTopicsGetQueryRequest {
            page_size: self.page_size,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
            from_unix_secs: self.from_unix_secs,
            to_unix_secs: self.to_unix_secs,
            cursor: self.cursor,
        })
    }
}

