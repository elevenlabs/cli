pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsBranchesListQueryRequest {
    /// Whether archived branches should be included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// How many results at most should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Whether to compute how far each branch has diverged from main (commits_ahead/commits_behind). This walks the version DAG of every branch, so it is slow on agents with long histories and is off by default, leaving those fields null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_commit_status: Option<bool>,
}

impl AgentsBranchesListQueryRequest {
    pub fn builder() -> AgentsBranchesListQueryRequestBuilder {
        <AgentsBranchesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsBranchesListQueryRequestBuilder {
    include_archived: Option<bool>,
    limit: Option<i64>,
    include_commit_status: Option<bool>,
}

impl AgentsBranchesListQueryRequestBuilder {
    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn include_commit_status(mut self, value: bool) -> Self {
        self.include_commit_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsBranchesListQueryRequest`].
    pub fn build(self) -> Result<AgentsBranchesListQueryRequest, BuildError> {
        Ok(AgentsBranchesListQueryRequest {
            include_archived: self.include_archived,
            limit: self.limit,
            include_commit_status: self.include_commit_status,
        })
    }
}

