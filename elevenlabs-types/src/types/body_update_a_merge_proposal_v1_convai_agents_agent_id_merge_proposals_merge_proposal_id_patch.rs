pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch {
    /// New title for the merge_proposal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New description for the merge_proposal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Replacement list of user IDs to request a review from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_reviewer_user_ids: Option<Vec<String>>,
    /// When true, close the merge_proposal without merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,
}

impl BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch {
    pub fn builder() -> BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatchBuilder {
        <BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatchBuilder {
    title: Option<String>,
    description: Option<String>,
    requested_reviewer_user_ids: Option<Vec<String>>,
    close: Option<bool>,
}

impl BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatchBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn requested_reviewer_user_ids(mut self, value: Vec<String>) -> Self {
        self.requested_reviewer_user_ids = Some(value);
        self
    }

    pub fn close(mut self, value: bool) -> Self {
        self.close = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch`].
    pub fn build(self) -> Result<BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch, BuildError> {
        Ok(BodyUpdateAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdPatch {
            title: self.title,
            description: self.description,
            requested_reviewer_user_ids: self.requested_reviewer_user_ids,
            close: self.close,
        })
    }
}

