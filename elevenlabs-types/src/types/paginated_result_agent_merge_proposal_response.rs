pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaginatedResultAgentMergeProposalResponse {
    #[serde(default)]
    pub results: Vec<AgentMergeProposalResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl PaginatedResultAgentMergeProposalResponse {
    pub fn builder() -> PaginatedResultAgentMergeProposalResponseBuilder {
        <PaginatedResultAgentMergeProposalResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginatedResultAgentMergeProposalResponseBuilder {
    results: Option<Vec<AgentMergeProposalResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl PaginatedResultAgentMergeProposalResponseBuilder {
    pub fn results(mut self, value: Vec<AgentMergeProposalResponse>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginatedResultAgentMergeProposalResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](PaginatedResultAgentMergeProposalResponseBuilder::results)
    pub fn build(self) -> Result<PaginatedResultAgentMergeProposalResponse, BuildError> {
        Ok(PaginatedResultAgentMergeProposalResponse {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
