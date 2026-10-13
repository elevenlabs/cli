pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost {
    /// Whether to archive the source branch after merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_source_branch: Option<bool>,
    /// Force source branch changes onto the target, overriding timestamp-based conflict resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost {
    pub fn builder() -> BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePostBuilder {
        <BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePostBuilder {
    archive_source_branch: Option<bool>,
    force: Option<bool>,
}

impl BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePostBuilder {
    pub fn archive_source_branch(mut self, value: bool) -> Self {
        self.archive_source_branch = Some(value);
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost`].
    pub fn build(self) -> Result<BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost, BuildError> {
        Ok(BodyMergeAMergeProposalV1ConvaiAgentsAgentIdMergeProposalsMergeProposalIdMergePost {
            archive_source_branch: self.archive_source_branch,
            force: self.force,
        })
    }
}

