pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentWorkspaceOverridesInput {
    /// The webhook to send conversation initiation client data to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<ConvAiWebhooks>,
}

impl AgentWorkspaceOverridesInput {
    pub fn builder() -> AgentWorkspaceOverridesInputBuilder {
        <AgentWorkspaceOverridesInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentWorkspaceOverridesInputBuilder {
    conversation_initiation_client_data_webhook: Option<ConversationInitiationClientDataWebhook>,
    webhooks: Option<ConvAiWebhooks>,
}

impl AgentWorkspaceOverridesInputBuilder {
    pub fn conversation_initiation_client_data_webhook(mut self, value: ConversationInitiationClientDataWebhook) -> Self {
        self.conversation_initiation_client_data_webhook = Some(value);
        self
    }

    pub fn webhooks(mut self, value: ConvAiWebhooks) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentWorkspaceOverridesInput`].
    pub fn build(self) -> Result<AgentWorkspaceOverridesInput, BuildError> {
        Ok(AgentWorkspaceOverridesInput {
            conversation_initiation_client_data_webhook: self.conversation_initiation_client_data_webhook,
            webhooks: self.webhooks,
        })
    }
}
