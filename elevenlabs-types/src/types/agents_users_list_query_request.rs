pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsUsersListQueryRequest {
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter conversations by branch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Unix timestamp (in seconds) to filter conversations up to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_before_unix: Option<i64>,
    /// Unix timestamp (in seconds) to filter conversations after to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_after_unix: Option<i64>,
    /// Search/filter by user ID (exact match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// How many users to return at maximum. Defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// The field to sort the results by. Defaults to last_contact_unix_secs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<UsersSortBy>,
    /// The direction to sort the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsUsersListQueryRequest {
    pub fn builder() -> AgentsUsersListQueryRequestBuilder {
        <AgentsUsersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUsersListQueryRequestBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
    call_start_before_unix: Option<i64>,
    call_start_after_unix: Option<i64>,
    search: Option<String>,
    page_size: Option<i64>,
    sort_by: Option<UsersSortBy>,
    sort_direction: Option<SortDirection>,
    cursor: Option<String>,
}

impl AgentsUsersListQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn call_start_before_unix(mut self, value: i64) -> Self {
        self.call_start_before_unix = Some(value);
        self
    }

    pub fn call_start_after_unix(mut self, value: i64) -> Self {
        self.call_start_after_unix = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort_by(mut self, value: UsersSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn sort_direction(mut self, value: SortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsUsersListQueryRequest`].
    pub fn build(self) -> Result<AgentsUsersListQueryRequest, BuildError> {
        Ok(AgentsUsersListQueryRequest {
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            call_start_before_unix: self.call_start_before_unix,
            call_start_after_unix: self.call_start_after_unix,
            search: self.search,
            page_size: self.page_size,
            sort_by: self.sort_by,
            sort_direction: self.sort_direction,
            cursor: self.cursor,
        })
    }
}

