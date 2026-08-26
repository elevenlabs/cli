pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsTopicsGetQueryRequest {
    /// Number of top-level topic groups to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Column to rank topics by. Use conversations for volume, sentiment with sort_direction=asc for the most negative topics, and frustration with sort_direction=desc for the most frustrated ones. Topics with no score are always ranked last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<TopicSortBy>,
    /// Direction to sort topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Start of the window to view topics for. When set with to_unix_secs, the completed daily topic-discovery runs in the range are aggregated together, so the window scopes the metrics as well as the topic set. Floored to the start of its UTC day because runs cover whole UTC days; aggregated_run_count reports how many runs were summed. Omit both bounds to get the single latest run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_unix_secs: Option<i64>,
    /// End of the window to view topics for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_unix_secs: Option<i64>,
    /// Include the per-criteria evaluation breakdown on each topic's metrics. Pass false to drop it: it dominates the payload and the weighted success_rate is returned either way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_evaluation_criteria: Option<bool>,
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
    include_evaluation_criteria: Option<bool>,
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

    pub fn include_evaluation_criteria(mut self, value: bool) -> Self {
        self.include_evaluation_criteria = Some(value);
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
            include_evaluation_criteria: self.include_evaluation_criteria,
            cursor: self.cursor,
        })
    }
}

