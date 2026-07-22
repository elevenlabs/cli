pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationInitiationClientDataConfigOutput {
    /// Overrides for the conversation configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config_override: Option<ConversationConfigClientOverrideConfigOutput>,
    /// Whether to include custom LLM extra body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_llm_extra_body: Option<bool>,
    /// Whether to enable conversation initiation client data from webhooks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_conversation_initiation_client_data_from_webhook: Option<bool>,
    /// Whether clients may pass starting_workflow_node_id in initiation client data; if false, sending it fails conversation start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_starting_workflow_node_id_from_client: Option<bool>,
}

impl ConversationInitiationClientDataConfigOutput {
    pub fn builder() -> ConversationInitiationClientDataConfigOutputBuilder {
        <ConversationInitiationClientDataConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationClientDataConfigOutputBuilder {
    conversation_config_override: Option<ConversationConfigClientOverrideConfigOutput>,
    custom_llm_extra_body: Option<bool>,
    enable_conversation_initiation_client_data_from_webhook: Option<bool>,
    enable_starting_workflow_node_id_from_client: Option<bool>,
}

impl ConversationInitiationClientDataConfigOutputBuilder {
    pub fn conversation_config_override(mut self, value: ConversationConfigClientOverrideConfigOutput) -> Self {
        self.conversation_config_override = Some(value);
        self
    }

    pub fn custom_llm_extra_body(mut self, value: bool) -> Self {
        self.custom_llm_extra_body = Some(value);
        self
    }

    pub fn enable_conversation_initiation_client_data_from_webhook(mut self, value: bool) -> Self {
        self.enable_conversation_initiation_client_data_from_webhook = Some(value);
        self
    }

    pub fn enable_starting_workflow_node_id_from_client(mut self, value: bool) -> Self {
        self.enable_starting_workflow_node_id_from_client = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationClientDataConfigOutput`].
    pub fn build(self) -> Result<ConversationInitiationClientDataConfigOutput, BuildError> {
        Ok(ConversationInitiationClientDataConfigOutput {
            conversation_config_override: self.conversation_config_override,
            custom_llm_extra_body: self.custom_llm_extra_body,
            enable_conversation_initiation_client_data_from_webhook: self.enable_conversation_initiation_client_data_from_webhook,
            enable_starting_workflow_node_id_from_client: self.enable_starting_workflow_node_id_from_client,
        })
    }
}
