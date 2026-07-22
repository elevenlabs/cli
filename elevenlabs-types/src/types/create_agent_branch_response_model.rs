pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentBranchResponseModel {
    /// ID of the created branch
    #[serde(default)]
    pub created_branch_id: String,
    /// ID of the first version on the created branch
    #[serde(default)]
    pub created_version_id: String,
}

impl CreateAgentBranchResponseModel {
    pub fn builder() -> CreateAgentBranchResponseModelBuilder {
        <CreateAgentBranchResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentBranchResponseModelBuilder {
    created_branch_id: Option<String>,
    created_version_id: Option<String>,
}

impl CreateAgentBranchResponseModelBuilder {
    pub fn created_branch_id(mut self, value: impl Into<String>) -> Self {
        self.created_branch_id = Some(value.into());
        self
    }

    pub fn created_version_id(mut self, value: impl Into<String>) -> Self {
        self.created_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentBranchResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_branch_id`](CreateAgentBranchResponseModelBuilder::created_branch_id)
    /// - [`created_version_id`](CreateAgentBranchResponseModelBuilder::created_version_id)
    pub fn build(self) -> Result<CreateAgentBranchResponseModel, BuildError> {
        Ok(CreateAgentBranchResponseModel {
            created_branch_id: self.created_branch_id.ok_or_else(|| BuildError::missing_field("created_branch_id"))?,
            created_version_id: self.created_version_id.ok_or_else(|| BuildError::missing_field("created_version_id"))?,
        })
    }
}
