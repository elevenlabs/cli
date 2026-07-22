pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentMetadata {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl AgentMetadata {
    pub fn builder() -> AgentMetadataBuilder {
        <AgentMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentMetadataBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
    workflow_node_id: Option<String>,
    version_id: Option<String>,
}

impl AgentMetadataBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AgentMetadataBuilder::agent_id)
    pub fn build(self) -> Result<AgentMetadata, BuildError> {
        Ok(AgentMetadata {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            workflow_node_id: self.workflow_node_id,
            version_id: self.version_id,
        })
    }
}
