pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
    /// New name for the branch. Must be unique within the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the branch should be archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// The protection level for the branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<BranchProtectionStatus>,
}

impl BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
    pub fn builder() -> BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
        <BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
    name: Option<String>,
    is_archived: Option<bool>,
    protection_status: Option<BranchProtectionStatus>,
}

impl BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatchBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn protection_status(mut self, value: BranchProtectionStatus) -> Self {
        self.protection_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch`].
    pub fn build(self) -> Result<BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch, BuildError> {
        Ok(BodyUpdateAgentBranchV1ConvaiAgentsAgentIdBranchesBranchIdPatch {
            name: self.name,
            is_archived: self.is_archived,
            protection_status: self.protection_status,
        })
    }
}

