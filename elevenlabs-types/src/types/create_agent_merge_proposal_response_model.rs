pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentMergeProposalResponseModel {
    /// ID of the created merge_proposal
    #[serde(default)]
    pub created_merge_proposal_id: String,
}

impl CreateAgentMergeProposalResponseModel {
    pub fn builder() -> CreateAgentMergeProposalResponseModelBuilder {
        <CreateAgentMergeProposalResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentMergeProposalResponseModelBuilder {
    created_merge_proposal_id: Option<String>,
}

impl CreateAgentMergeProposalResponseModelBuilder {
    pub fn created_merge_proposal_id(mut self, value: impl Into<String>) -> Self {
        self.created_merge_proposal_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentMergeProposalResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_merge_proposal_id`](CreateAgentMergeProposalResponseModelBuilder::created_merge_proposal_id)
    pub fn build(self) -> Result<CreateAgentMergeProposalResponseModel, BuildError> {
        Ok(CreateAgentMergeProposalResponseModel {
            created_merge_proposal_id: self.created_merge_proposal_id.ok_or_else(|| BuildError::missing_field("created_merge_proposal_id"))?,
        })
    }
}
