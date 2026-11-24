pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyMergeABranchIntoATargetBranchV1ConvaiAgentsAgentIdBranchesSourceBranchIdMergePost {
    /// Whether to archive the source branch after merging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_source_branch: Option<bool>,
    /// Keep the source branch's value for every conflict instead of sending `conflict_resolutions`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Side to keep, or custom text where the conflict accepts it, for each conflict the preview reported. A merge or rebase with conflicts is rejected unless there is one for every conflict, sent with both tip ids. On a merge it can't be combined with `force`, which lets the source win every conflict instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolutions: Option<Vec<ConflictResolution>>,
    /// Source branch tip version the conflicts were resolved against (`source_tip_version_id` of the preview). The request fails with 409 if the branch moved since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tip_id: Option<String>,
    /// Target branch tip version the conflicts were resolved against (`target_tip_version_id` of the preview). The request fails with 409 if the branch moved since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tip_id: Option<String>,
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
    conflict_resolutions: Option<Vec<ConflictResolution>>,
    source_tip_id: Option<String>,
    target_tip_id: Option<String>,
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

    pub fn conflict_resolutions(mut self, value: Vec<ConflictResolution>) -> Self {
        self.conflict_resolutions = Some(value);
        self
    }

    pub fn source_tip_id(mut self, value: impl Into<String>) -> Self {
        self.source_tip_id = Some(value.into());
        self
    }

    pub fn target_tip_id(mut self, value: impl Into<String>) -> Self {
        self.target_tip_id = Some(value.into());
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
            conflict_resolutions: self.conflict_resolutions,
            source_tip_id: self.source_tip_id,
            target_tip_id: self.target_tip_id,
            target_branch_id: self.target_branch_id.ok_or_else(|| BuildError::missing_field("target_branch_id"))?,
        })
    }
}

