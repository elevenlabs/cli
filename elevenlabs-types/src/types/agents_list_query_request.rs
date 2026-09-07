pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsListQueryRequest {
    /// How many Agents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search by agents name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter agents by archived status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// If set to true, the endpoint will omit any agents that were shared with you by someone else and include only the ones you own. Deprecated: use created_by_user_id instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_only_owned_agents: Option<bool>,
    /// Filter agents by creator user ID. When set, only agents created by this user are returned. Takes precedence over show_only_owned_agents. Use '@me' to refer to the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    /// Filter agents by tag. Repeat the parameter to match any of several tags.
    #[serde(default)]
    pub tags: Vec<Option<String>>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// The field to sort the results by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AgentSortBy>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsListQueryRequest {
    pub fn builder() -> AgentsListQueryRequestBuilder {
        <AgentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsListQueryRequestBuilder {
    page_size: Option<i64>,
    search: Option<String>,
    archived: Option<bool>,
    show_only_owned_agents: Option<bool>,
    created_by_user_id: Option<String>,
    tags: Option<Vec<Option<String>>>,
    sort_direction: Option<SortDirection>,
    sort_by: Option<AgentSortBy>,
    cursor: Option<String>,
}

impl AgentsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn show_only_owned_agents(mut self, value: bool) -> Self {
        self.show_only_owned_agents = Some(value);
        self
    }

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<Option<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn sort_by(mut self, value: AgentSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](AgentsListQueryRequestBuilder::tags)
    pub fn build(self) -> Result<AgentsListQueryRequest, BuildError> {
        Ok(AgentsListQueryRequest {
            page_size: self.page_size,
            search: self.search,
            archived: self.archived,
            show_only_owned_agents: self.show_only_owned_agents,
            created_by_user_id: self.created_by_user_id,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            sort_direction: self.sort_direction,
            sort_by: self.sort_by,
            cursor: self.cursor,
        })
    }
}

