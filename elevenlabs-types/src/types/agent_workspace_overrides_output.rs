pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentWorkspaceOverridesOutput {
    /// The webhook to send conversation initiation client data to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<ConvAiWebhooks>,
}

impl AgentWorkspaceOverridesOutput {
    pub fn builder() -> AgentWorkspaceOverridesOutputBuilder {
        <AgentWorkspaceOverridesOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentWorkspaceOverridesOutputBuilder {
    conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    webhooks: Option<ConvAiWebhooks>,
}

impl AgentWorkspaceOverridesOutputBuilder {
    pub fn conversation_initiation_client_data_webhook(mut self, value: ConversationInitiationClientDataWebhook) -> Self {
        self.conversation_initiation_client_data_webhook = Some(value);
        self
    }

    pub fn webhooks(mut self, value: ConvAiWebhooks) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentWorkspaceOverridesOutput`].
    pub fn build(self) -> Result<AgentWorkspaceOverridesOutput, BuildError> {
        Ok(AgentWorkspaceOverridesOutput {
            conversation_initiation_client_data_webhook: self.conversation_initiation_client_data_webhook,
            webhooks: self.webhooks,
        })
    }
}
