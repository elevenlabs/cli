pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsTestsInvocationsListQueryRequest {
    /// Filter by agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter by branch ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Filter to invocations that include a run of this test id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    /// Inclusive lower bound on created_at, in unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Inclusive upper bound on created_at, in unix seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Filter by the agent version the tests ran against
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search query to filter tests and folders by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsTestsInvocationsListQueryRequest {
    pub fn builder() -> AgentsTestsInvocationsListQueryRequestBuilder {
        <AgentsTestsInvocationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsTestsInvocationsListQueryRequestBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
    test_id: Option<String>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    version_id: Option<String>,
    page_size: Option<i64>,
    search: Option<String>,
    cursor: Option<String>,
}

impl AgentsTestsInvocationsListQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn test_id(mut self, value: impl Into<String>) -> Self {
        self.test_id = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`AgentsTestsInvocationsListQueryRequest`].
    pub fn build(self) -> Result<AgentsTestsInvocationsListQueryRequest, BuildError> {
        Ok(AgentsTestsInvocationsListQueryRequest {
            agent_id: self.agent_id,
            branch_id: self.branch_id,
            test_id: self.test_id,
            start_time: self.start_time,
            end_time: self.end_time,
            version_id: self.version_id,
            page_size: self.page_size,
            search: self.search,
            cursor: self.cursor,
        })
    }
}

