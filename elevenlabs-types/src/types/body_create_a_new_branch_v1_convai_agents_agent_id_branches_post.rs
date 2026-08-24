pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost {
    /// ID of the version to branch from
    #[serde(default)]
    pub parent_version_id: String,
    /// Name of the branch. It is unique within the agent.
    #[serde(default)]
    pub name: String,
    /// Description for the branch
    #[serde(default)]
    pub description: String,
    /// Changes to apply to conversation config
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<HashMap<String, serde_json::Value>>,
    /// Changes to apply to platform settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<HashMap<String, serde_json::Value>>,
    /// Updated workflow definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<AgentWorkflowRequestModel>,
}

impl BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost {
    pub fn builder() -> BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder {
        <BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder {
    parent_version_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    conversation_config: Option<HashMap<String, serde_json::Value>>,
    platform_settings: Option<HashMap<String, serde_json::Value>>,
    workflow: Option<AgentWorkflowRequestModel>,
}

impl BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder {
    pub fn parent_version_id(mut self, value: impl Into<String>) -> Self {
        self.parent_version_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn conversation_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.conversation_config = Some(value);
        self
    }

    pub fn platform_settings(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.platform_settings = Some(value);
        self
    }

    pub fn workflow(mut self, value: AgentWorkflowRequestModel) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parent_version_id`](BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder::parent_version_id)
    /// - [`name`](BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder::name)
    /// - [`description`](BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPostBuilder::description)
    pub fn build(self) -> Result<BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost, BuildError> {
        Ok(BodyCreateANewBranchV1ConvaiAgentsAgentIdBranchesPost {
            parent_version_id: self.parent_version_id.ok_or_else(|| BuildError::missing_field("parent_version_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            conversation_config: self.conversation_config,
            platform_settings: self.platform_settings,
            workflow: self.workflow,
        })
    }
}

