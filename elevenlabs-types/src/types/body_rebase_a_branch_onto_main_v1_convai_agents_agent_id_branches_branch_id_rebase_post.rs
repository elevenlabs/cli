pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
    /// Side to keep, or custom text where the conflict accepts it, for each conflict the preview reported. A merge or rebase with conflicts is rejected unless there is one for every conflict, sent with both tip ids. On a merge it can't be combined with `force`, which lets the source win every conflict instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_resolutions: Option<Vec<ConflictResolution>>,
    /// Tip version of the branch being rebased (`source_tip_version_id` of the rebase preview) the conflicts were resolved against. The request fails with 409 if it moved since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_tip_id: Option<String>,
    /// Tip version of the branch the changes are pulled from (`source_branch_id`, the main branch by default; `target_tip_version_id` of the rebase preview) the conflicts were resolved against. The request fails with 409 if it moved since.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_tip_id: Option<String>,
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
    conflict_resolutions: Option<Vec<ConflictResolution>>,
    branch_tip_id: Option<String>,
    source_tip_id: Option<String>,
    source_branch_id: Option<String>,
}

impl BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePostBuilder {
    pub fn conflict_resolutions(mut self, value: Vec<ConflictResolution>) -> Self {
        self.conflict_resolutions = Some(value);
        self
    }

    pub fn branch_tip_id(mut self, value: impl Into<String>) -> Self {
        self.branch_tip_id = Some(value.into());
        self
    }

    pub fn source_tip_id(mut self, value: impl Into<String>) -> Self {
        self.source_tip_id = Some(value.into());
        self
    }

    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost`].
    pub fn build(self) -> Result<BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost, BuildError> {
        Ok(BodyRebaseABranchOntoMainV1ConvaiAgentsAgentIdBranchesBranchIdRebasePost {
            conflict_resolutions: self.conflict_resolutions,
            branch_tip_id: self.branch_tip_id,
            source_tip_id: self.source_tip_id,
            source_branch_id: self.source_branch_id,
        })
    }
}

