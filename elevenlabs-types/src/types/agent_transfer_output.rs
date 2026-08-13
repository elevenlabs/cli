pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentTransferOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default)]
    pub condition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_transferred_agent_first_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_workflow_node_transfer: Option<bool>,
    /// Defines whether TTS client overrides should be carried over to the transferred agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_client_tts_overrides: Option<bool>,
}

impl AgentTransferOutput {
    pub fn builder() -> AgentTransferOutputBuilder {
        <AgentTransferOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentTransferOutputBuilder {
    agent_id: Option<String>,
    node_id: Option<String>,
    condition: Option<String>,
    delay_ms: Option<i64>,
    transfer_message: Option<String>,
    enable_transferred_agent_first_message: Option<bool>,
    is_workflow_node_transfer: Option<bool>,
    preserve_client_tts_overrides: Option<bool>,
}

impl AgentTransferOutputBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn node_id(mut self, value: impl Into<String>) -> Self {
        self.node_id = Some(value.into());
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    pub fn delay_ms(mut self, value: i64) -> Self {
        self.delay_ms = Some(value);
        self
    }

    pub fn transfer_message(mut self, value: impl Into<String>) -> Self {
        self.transfer_message = Some(value.into());
        self
    }

    pub fn enable_transferred_agent_first_message(mut self, value: bool) -> Self {
        self.enable_transferred_agent_first_message = Some(value);
        self
    }

    pub fn is_workflow_node_transfer(mut self, value: bool) -> Self {
        self.is_workflow_node_transfer = Some(value);
        self
    }

    pub fn preserve_client_tts_overrides(mut self, value: bool) -> Self {
        self.preserve_client_tts_overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentTransferOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](AgentTransferOutputBuilder::condition)
    pub fn build(self) -> Result<AgentTransferOutput, BuildError> {
        Ok(AgentTransferOutput {
            agent_id: self.agent_id,
            node_id: self.node_id,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
            delay_ms: self.delay_ms,
            transfer_message: self.transfer_message,
            enable_transferred_agent_first_message: self.enable_transferred_agent_first_message,
            is_workflow_node_transfer: self.is_workflow_node_transfer,
            preserve_client_tts_overrides: self.preserve_client_tts_overrides,
        })
    }
}
