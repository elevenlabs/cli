pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentVersionMetadata {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub branch_id: String,
    #[serde(default)]
    pub version_description: String,
    #[serde(default)]
    pub seq_no_in_branch: i64,
    #[serde(default)]
    pub time_committed_secs: i64,
    #[serde(default)]
    pub parents: AgentVersionParents,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
}

impl AgentVersionMetadata {
    pub fn builder() -> AgentVersionMetadataBuilder {
        <AgentVersionMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentVersionMetadataBuilder {
    id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    version_description: Option<String>,
    seq_no_in_branch: Option<i64>,
    time_committed_secs: Option<i64>,
    parents: Option<AgentVersionParents>,
    access_info: Option<ResourceAccessInfo>,
}

impl AgentVersionMetadataBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn version_description(mut self, value: impl Into<String>) -> Self {
        self.version_description = Some(value.into());
        self
    }

    pub fn seq_no_in_branch(mut self, value: i64) -> Self {
        self.seq_no_in_branch = Some(value);
        self
    }

    pub fn time_committed_secs(mut self, value: i64) -> Self {
        self.time_committed_secs = Some(value);
        self
    }

    pub fn parents(mut self, value: AgentVersionParents) -> Self {
        self.parents = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentVersionMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentVersionMetadataBuilder::id)
    /// - [`agent_id`](AgentVersionMetadataBuilder::agent_id)
    /// - [`branch_id`](AgentVersionMetadataBuilder::branch_id)
    /// - [`version_description`](AgentVersionMetadataBuilder::version_description)
    /// - [`seq_no_in_branch`](AgentVersionMetadataBuilder::seq_no_in_branch)
    /// - [`time_committed_secs`](AgentVersionMetadataBuilder::time_committed_secs)
    /// - [`parents`](AgentVersionMetadataBuilder::parents)
    pub fn build(self) -> Result<AgentVersionMetadata, BuildError> {
        Ok(AgentVersionMetadata {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
            version_description: self.version_description.ok_or_else(|| BuildError::missing_field("version_description"))?,
            seq_no_in_branch: self.seq_no_in_branch.ok_or_else(|| BuildError::missing_field("seq_no_in_branch"))?,
            time_committed_secs: self.time_committed_secs.ok_or_else(|| BuildError::missing_field("time_committed_secs"))?,
            parents: self.parents.ok_or_else(|| BuildError::missing_field("parents"))?,
            access_info: self.access_info,
        })
    }
}
