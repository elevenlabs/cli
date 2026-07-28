pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DependentBranchInfo {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub agent_name: String,
    #[serde(default)]
    pub branch_id: String,
    #[serde(default)]
    pub branch_name: String,
    #[serde(default)]
    pub is_main: bool,
}

impl DependentBranchInfo {
    pub fn builder() -> DependentBranchInfoBuilder {
        <DependentBranchInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DependentBranchInfoBuilder {
    agent_id: Option<String>,
    agent_name: Option<String>,
    branch_id: Option<String>,
    branch_name: Option<String>,
    is_main: Option<bool>,
}

impl DependentBranchInfoBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn branch_name(mut self, value: impl Into<String>) -> Self {
        self.branch_name = Some(value.into());
        self
    }

    pub fn is_main(mut self, value: bool) -> Self {
        self.is_main = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DependentBranchInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](DependentBranchInfoBuilder::agent_id)
    /// - [`agent_name`](DependentBranchInfoBuilder::agent_name)
    /// - [`branch_id`](DependentBranchInfoBuilder::branch_id)
    /// - [`branch_name`](DependentBranchInfoBuilder::branch_name)
    /// - [`is_main`](DependentBranchInfoBuilder::is_main)
    pub fn build(self) -> Result<DependentBranchInfo, BuildError> {
        Ok(DependentBranchInfo {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self.agent_name.ok_or_else(|| BuildError::missing_field("agent_name"))?,
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
            branch_name: self.branch_name.ok_or_else(|| BuildError::missing_field("branch_name"))?,
            is_main: self.is_main.ok_or_else(|| BuildError::missing_field("is_main"))?,
        })
    }
}
