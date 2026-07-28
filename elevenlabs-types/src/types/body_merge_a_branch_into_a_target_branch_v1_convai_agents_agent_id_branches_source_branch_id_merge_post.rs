pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
    /// Whether to archive the source branch after merging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_source_branch: Option<bool>,
    /// Force source branch changes onto the target, overriding timestamp-based conflict resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// The ID of the target branch to merge into.
    #[serde(skip)]
    #[serde(default)]
    pub target_branch_id: String,
}

impl BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
    pub fn builder() -> BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder {
        <BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder {
    archive_source_branch: Option<bool>,
    force: Option<bool>,
    target_branch_id: Option<String>,
}

impl BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder {
    pub fn archive_source_branch(mut self, value: bool) -> Self {
        self.archive_source_branch = Some(value);
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    pub fn target_branch_id(mut self, value: impl Into<String>) -> Self {
        self.target_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_branch_id`](BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePostBuilder::target_branch_id)
    pub fn build(self) -> Result<BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost, BuildError> {
        Ok(BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
            archive_source_branch: self.archive_source_branch,
            force: self.force,
            target_branch_id: self.target_branch_id.ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
        })
    }
}

