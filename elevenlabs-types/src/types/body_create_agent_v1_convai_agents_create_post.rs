pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyCreateAgentV1ConvaiAgentsCreatePost {
    /// Conversation configuration for an agent
    #[serde(default)]
    pub conversation_config: ConversationalConfig,
    /// Platform settings for the agent are all settings that aren't related to the conversation orchestration and content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<AgentPlatformSettingsRequestModel>,
    /// Workflow for the agent. This is used to define the flow of the conversation and how the agent interacts with tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<AgentWorkflowRequestModel>,
    /// A name to make the agent easier to find
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tags to help classify and filter the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Deprecated: all agents are versioned. This parameter is ignored.
    #[serde(skip)]
    pub enable_versioning: Option<bool>,
}

impl BodyCreateAgentV1ConvaiAgentsCreatePost {
    pub fn builder() -> BodyCreateAgentV1ConvaiAgentsCreatePostBuilder {
        <BodyCreateAgentV1ConvaiAgentsCreatePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateAgentV1ConvaiAgentsCreatePostBuilder {
    conversation_config: Option<ConversationalConfig>,
    platform_settings: Option<AgentPlatformSettingsRequestModel>,
    workflow: Option<AgentWorkflowRequestModel>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    enable_versioning: Option<bool>,
}

impl BodyCreateAgentV1ConvaiAgentsCreatePostBuilder {
    pub fn conversation_config(mut self, value: ConversationalConfig) -> Self {
        self.conversation_config = Some(value);
        self
    }

    pub fn platform_settings(mut self, value: AgentPlatformSettingsRequestModel) -> Self {
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

    pub fn enable_versioning(mut self, value: bool) -> Self {
        self.enable_versioning = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateAgentV1ConvaiAgentsCreatePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_config`](BodyCreateAgentV1ConvaiAgentsCreatePostBuilder::conversation_config)
    pub fn build(self) -> Result<BodyCreateAgentV1ConvaiAgentsCreatePost, BuildError> {
        Ok(BodyCreateAgentV1ConvaiAgentsCreatePost {
            conversation_config: self.conversation_config.ok_or_else(|| BuildError::missing_field("conversation_config"))?,
            platform_settings: self.platform_settings,
            workflow: self.workflow,
            name: self.name,
            tags: self.tags,
            enable_versioning: self.enable_versioning,
        })
    }
}

