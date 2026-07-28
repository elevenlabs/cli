pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResponseAgentBranchSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListResponseMeta>,
    #[serde(default)]
    pub results: Vec<AgentBranchSummary>,
}

impl ListResponseAgentBranchSummary {
    pub fn builder() -> ListResponseAgentBranchSummaryBuilder {
        <ListResponseAgentBranchSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseAgentBranchSummaryBuilder {
    meta: Option<ListResponseMeta>,
    results: Option<Vec<AgentBranchSummary>>,
}

impl ListResponseAgentBranchSummaryBuilder {
    pub fn meta(mut self, value: ListResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<AgentBranchSummary>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseAgentBranchSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ListResponseAgentBranchSummaryBuilder::results)
    pub fn build(self) -> Result<ListResponseAgentBranchSummary, BuildError> {
        Ok(ListResponseAgentBranchSummary {
            meta: self.meta,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
