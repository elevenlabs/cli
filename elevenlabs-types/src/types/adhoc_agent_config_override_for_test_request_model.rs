pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdhocAgentConfigOverrideForTestRequestModel {
    /// Conversation config fields to change for this run. Fields left out keep the agent's values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config: Option<ConversationalConfig>,
    /// Platform settings fields to change for this run. Fields left out keep the agent's values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<AgentPlatformSettingsRequestModel>,
    /// A workflow to run instead of the agent's. It replaces the agent's workflow whole.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<AgentWorkflowRequestModel>,
}

impl AdhocAgentConfigOverrideForTestRequestModel {
    pub fn builder() -> AdhocAgentConfigOverrideForTestRequestModelBuilder {
        <AdhocAgentConfigOverrideForTestRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdhocAgentConfigOverrideForTestRequestModelBuilder {
    conversation_config: Option<ConversationalConfig>,
    platform_settings: Option<AgentPlatformSettingsRequestModel>,
    workflow: Option<AgentWorkflowRequestModel>,
}

impl AdhocAgentConfigOverrideForTestRequestModelBuilder {
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

    /// Consumes the builder and constructs a [`AdhocAgentConfigOverrideForTestRequestModel`].
    pub fn build(self) -> Result<AdhocAgentConfigOverrideForTestRequestModel, BuildError> {
        Ok(AdhocAgentConfigOverrideForTestRequestModel {
            conversation_config: self.conversation_config,
            platform_settings: self.platform_settings,
            workflow: self.workflow,
        })
    }
}
