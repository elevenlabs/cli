pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdhocAgentConfigOverrideForTestRequestModel {
    #[serde(default)]
    pub conversation_config: ConversationalConfig,
    #[serde(default)]
    pub platform_settings: AgentPlatformSettingsRequestModel,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_config`](AdhocAgentConfigOverrideForTestRequestModelBuilder::conversation_config)
    /// - [`platform_settings`](AdhocAgentConfigOverrideForTestRequestModelBuilder::platform_settings)
    pub fn build(self) -> Result<AdhocAgentConfigOverrideForTestRequestModel, BuildError> {
        Ok(AdhocAgentConfigOverrideForTestRequestModel {
            conversation_config: self.conversation_config.ok_or_else(|| BuildError::missing_field("conversation_config"))?,
            platform_settings: self.platform_settings.ok_or_else(|| BuildError::missing_field("platform_settings"))?,
            workflow: self.workflow,
        })
    }
}
