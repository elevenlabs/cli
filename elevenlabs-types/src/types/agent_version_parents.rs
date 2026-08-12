pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVersionParents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_branch_parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_branch_parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_from_branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_from_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebased_from_version_id: Option<String>,
}

impl AgentVersionParents {
    pub fn builder() -> AgentVersionParentsBuilder {
        <AgentVersionParentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionParentsBuilder {
    in_branch_parent_id: Option<String>,
    out_of_branch_parent_id: Option<String>,
    merged_into_branch_id: Option<String>,
    merged_from_branch_id: Option<String>,
    merged_from_version_id: Option<String>,
    rebased_from_version_id: Option<String>,
}

impl AgentVersionParentsBuilder {
    pub fn in_branch_parent_id(mut self, value: impl Into<String>) -> Self {
        self.in_branch_parent_id = Some(value.into());
        self
    }

    pub fn out_of_branch_parent_id(mut self, value: impl Into<String>) -> Self {
        self.out_of_branch_parent_id = Some(value.into());
        self
    }

    pub fn merged_into_branch_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_branch_id = Some(value.into());
        self
    }

    pub fn merged_from_branch_id(mut self, value: impl Into<String>) -> Self {
        self.merged_from_branch_id = Some(value.into());
        self
    }

    pub fn merged_from_version_id(mut self, value: impl Into<String>) -> Self {
        self.merged_from_version_id = Some(value.into());
        self
    }

    pub fn rebased_from_version_id(mut self, value: impl Into<String>) -> Self {
        self.rebased_from_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentVersionParents`].
    pub fn build(self) -> Result<AgentVersionParents, BuildError> {
        Ok(AgentVersionParents {
            in_branch_parent_id: self.in_branch_parent_id,
            out_of_branch_parent_id: self.out_of_branch_parent_id,
            merged_into_branch_id: self.merged_into_branch_id,
            merged_from_branch_id: self.merged_from_branch_id,
            merged_from_version_id: self.merged_from_version_id,
            rebased_from_version_id: self.rebased_from_version_id,
        })
    }
}
