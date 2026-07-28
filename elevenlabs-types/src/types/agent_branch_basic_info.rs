pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentBranchBasicInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl AgentBranchBasicInfo {
    pub fn builder() -> AgentBranchBasicInfoBuilder {
        <AgentBranchBasicInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBranchBasicInfoBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AgentBranchBasicInfoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentBranchBasicInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentBranchBasicInfoBuilder::id)
    /// - [`name`](AgentBranchBasicInfoBuilder::name)
    pub fn build(self) -> Result<AgentBranchBasicInfo, BuildError> {
        Ok(AgentBranchBasicInfo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
