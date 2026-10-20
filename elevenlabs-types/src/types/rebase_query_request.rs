pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for rebase
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RebaseQueryRequest {
    /// The ID of the branch to pull changes from. Defaults to the agent's main branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch_id: Option<String>,
}

impl RebaseQueryRequest {
    pub fn builder() -> RebaseQueryRequestBuilder {
        <RebaseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RebaseQueryRequestBuilder {
    source_branch_id: Option<String>,
}

impl RebaseQueryRequestBuilder {
    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RebaseQueryRequest`].
    pub fn build(self) -> Result<RebaseQueryRequest, BuildError> {
        Ok(RebaseQueryRequest {
            source_branch_id: self.source_branch_id,
        })
    }
}

