pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
    /// The ID of the branch to pull changes from. Defaults to the agent's main branch.
    #[serde(skip)]
    pub source_branch_id: Option<String>,
}

impl BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
    pub fn builder() -> BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
        <BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
    source_branch_id: Option<String>,
}

impl BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost`].
    pub fn build(self) -> Result<BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost, BuildError> {
        Ok(BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
            source_branch_id: self.source_branch_id,
        })
    }
}

