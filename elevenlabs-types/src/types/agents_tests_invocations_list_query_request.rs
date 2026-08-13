pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsTestsInvocationsListQueryRequest {
    /// Filter by agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
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
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsTestsInvocationsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`AgentsTestsInvocationsListQueryRequest`].
    pub fn build(self) -> Result<AgentsTestsInvocationsListQueryRequest, BuildError> {
        Ok(AgentsTestsInvocationsListQueryRequest {
            agent_id: self.agent_id,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

