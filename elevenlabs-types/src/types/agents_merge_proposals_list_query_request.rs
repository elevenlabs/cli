pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsMergeProposalsListQueryRequest {
    /// Only return proposals with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MergeProposalStatus>,
    /// Only return proposals with this source branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch_id: Option<String>,
    /// Only return proposals with this target branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_branch_id: Option<String>,
    /// Case-insensitive substring match over title and description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// How many results at most should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsMergeProposalsListQueryRequest {
    pub fn builder() -> AgentsMergeProposalsListQueryRequestBuilder {
        <AgentsMergeProposalsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsMergeProposalsListQueryRequestBuilder {
    status: Option<MergeProposalStatus>,
    source_branch_id: Option<String>,
    target_branch_id: Option<String>,
    search: Option<String>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsMergeProposalsListQueryRequestBuilder {
    pub fn status(mut self, value: MergeProposalStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
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

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsMergeProposalsListQueryRequest`].
    pub fn build(self) -> Result<AgentsMergeProposalsListQueryRequest, BuildError> {
        Ok(AgentsMergeProposalsListQueryRequest {
            status: self.status,
            source_branch_id: self.source_branch_id,
            target_branch_id: self.target_branch_id,
            search: self.search,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

