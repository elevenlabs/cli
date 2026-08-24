pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A specification that will be used to simulate a conversation between an agent and an AI user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationSimulationSpecification {
    #[serde(default)]
    pub simulated_user_config: AgentConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_mock_config: Option<HashMap<String, ToolMockConfig>>,
    /// A partial conversation history to start the simulation from. If empty, simulation starts fresh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_conversation_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl ConversationSimulationSpecification {
    pub fn builder() -> ConversationSimulationSpecificationBuilder {
        <ConversationSimulationSpecificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSimulationSpecificationBuilder {
    simulated_user_config: Option<AgentConfig>,
    tool_mock_config: Option<HashMap<String, ToolMockConfig>>,
    partial_conversation_history: Option<Vec<ConversationHistoryTranscriptCommonModelInput>>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl ConversationSimulationSpecificationBuilder {
    pub fn simulated_user_config(mut self, value: AgentConfig) -> Self {
        self.simulated_user_config = Some(value);
        self
    }

    pub fn tool_mock_config(mut self, value: HashMap<String, ToolMockConfig>) -> Self {
        self.tool_mock_config = Some(value);
        self
    }

    pub fn partial_conversation_history(mut self, value: Vec<ConversationHistoryTranscriptCommonModelInput>) -> Self {
        self.partial_conversation_history = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationSimulationSpecification`].
    /// This method will fail if any of the following fields are not set:
    /// - [`simulated_user_config`](ConversationSimulationSpecificationBuilder::simulated_user_config)
    pub fn build(self) -> Result<ConversationSimulationSpecification, BuildError> {
        Ok(ConversationSimulationSpecification {
            simulated_user_config: self.simulated_user_config.ok_or_else(|| BuildError::missing_field("simulated_user_config"))?,
            tool_mock_config: self.tool_mock_config,
            partial_conversation_history: self.partial_conversation_history,
            dynamic_variables: self.dynamic_variables,
        })
    }
}
