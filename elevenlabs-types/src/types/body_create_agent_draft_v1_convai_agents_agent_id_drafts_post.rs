pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost {
    /// Conversation config for the draft
    #[serde(default)]
    pub conversation_config: HashMap<String, serde_json::Value>,
    /// Platform settings for the draft
    #[serde(default)]
    pub platform_settings: HashMap<String, serde_json::Value>,
    /// Workflow for the draft
    #[serde(default)]
    pub workflow: AgentWorkflowRequestModel,
    /// Name for the draft
    #[serde(default)]
    pub name: String,
    /// Tags to help classify and filter the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The ID of the agent branch to use
    #[serde(skip)]
    #[serde(default)]
    pub branch_id: String,
}

impl BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost {
    pub fn builder() -> BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder {
        <BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder {
    conversation_config: Option<HashMap<String, serde_json::Value>>,
    platform_settings: Option<HashMap<String, serde_json::Value>>,
    workflow: Option<AgentWorkflowRequestModel>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    branch_id: Option<String>,
}

impl BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_config`](BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder::conversation_config)
    /// - [`platform_settings`](BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder::platform_settings)
    /// - [`workflow`](BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder::workflow)
    /// - [`name`](BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder::name)
    /// - [`branch_id`](BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPostBuilder::branch_id)
    pub fn build(self) -> Result<BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost, BuildError> {
        Ok(BodyCreateAgentDraftV1ConvaiAgentsAgentIdDraftsPost {
            conversation_config: self.conversation_config.ok_or_else(|| BuildError::missing_field("conversation_config"))?,
            platform_settings: self.platform_settings.ok_or_else(|| BuildError::missing_field("platform_settings"))?,
            workflow: self.workflow.ok_or_else(|| BuildError::missing_field("workflow"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tags: self.tags,
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}

