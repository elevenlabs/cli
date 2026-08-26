pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum UnitTestRunResponseModelTestInfo {
        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dynamic_variables: Option<HashMap<String, serde_json::Value>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            conversation_initiation_source: Option<ConversationInitiationSource>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            success_condition: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            success_examples: Option<Vec<AgentSuccessfulResponseExample>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_examples: Option<Vec<AgentFailureResponseExample>>,
        },

        #[serde(rename = "simulation")]
        #[non_exhaustive]
        Simulation {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dynamic_variables: Option<HashMap<String, serde_json::Value>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            conversation_initiation_source: Option<ConversationInitiationSource>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            success_condition: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            success_conditions: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            simulation_scenario: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            simulation_max_turns: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            simulation_environment: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tool_mock_config: Option<SimulationToolMockBehaviorConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            evaluation_model: Option<Llm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            simulated_user_model: Option<Llm>,
        },

        #[serde(rename = "tool")]
        #[non_exhaustive]
        Tool {
            #[serde(skip_serializing_if = "Option::is_none")]
            from_conversation_metadata: Option<TestFromConversationMetadataOutput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            dynamic_variables: Option<HashMap<String, serde_json::Value>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            conversation_initiation_source: Option<ConversationInitiationSource>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>,
            #[serde(skip_serializing_if = "Option::is_none")]
            check_any_tool_matches: Option<bool>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl UnitTestRunResponseModelTestInfo {
    pub fn llm() -> Self {
        Self::Llm { from_conversation_metadata: None, dynamic_variables: None, chat_history: None, conversation_initiation_source: None, environment: None, success_condition: None, success_examples: None, failure_examples: None }
    }

    pub fn simulation() -> Self {
        Self::Simulation { from_conversation_metadata: None, dynamic_variables: None, chat_history: None, conversation_initiation_source: None, environment: None, success_condition: None, success_conditions: None, simulation_scenario: None, simulation_max_turns: None, simulation_environment: None, tool_mock_config: None, tool_mock_overrides: None, evaluation_model: None, simulated_user_model: None }
    }

    pub fn tool() -> Self {
        Self::Tool { from_conversation_metadata: None, dynamic_variables: None, chat_history: None, conversation_initiation_source: None, environment: None, tool_call_parameters: None, check_any_tool_matches: None }
    }

    pub fn llm_with_from_conversation_metadata(from_conversation_metadata: TestFromConversationMetadataOutput, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata: Some(from_conversation_metadata), dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_examples, failure_examples }
    }

    pub fn llm_with_dynamic_variables(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: HashMap<String, serde_json::Value>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables: Some(dynamic_variables), chat_history, conversation_initiation_source, environment, success_condition, success_examples, failure_examples }
    }

    pub fn llm_with_chat_history(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Vec<ConversationHistoryTranscriptCommonModelOutput>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history: Some(chat_history), conversation_initiation_source, environment, success_condition, success_examples, failure_examples }
    }

    pub fn llm_with_conversation_initiation_source(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: ConversationInitiationSource, environment: Option<String>, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source: Some(conversation_initiation_source), environment, success_condition, success_examples, failure_examples }
    }

    pub fn llm_with_environment(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: String, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment: Some(environment), success_condition, success_examples, failure_examples }
    }

    pub fn llm_with_success_condition(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: String, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition: Some(success_condition), success_examples, failure_examples }
    }

    pub fn llm_with_success_examples(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_examples: Vec<AgentSuccessfulResponseExample>, failure_examples: Option<Vec<AgentFailureResponseExample>>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_examples: Some(success_examples), failure_examples }
    }

    pub fn llm_with_failure_examples(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_examples: Option<Vec<AgentSuccessfulResponseExample>>, failure_examples: Vec<AgentFailureResponseExample>) -> Self {
        Self::Llm { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_examples, failure_examples: Some(failure_examples) }
    }

    pub fn simulation_with_from_conversation_metadata(from_conversation_metadata: TestFromConversationMetadataOutput, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata: Some(from_conversation_metadata), dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_dynamic_variables(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: HashMap<String, serde_json::Value>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables: Some(dynamic_variables), chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_chat_history(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Vec<ConversationHistoryTranscriptCommonModelOutput>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history: Some(chat_history), conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_conversation_initiation_source(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: ConversationInitiationSource, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source: Some(conversation_initiation_source), environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_environment(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: String, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment: Some(environment), success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_success_condition(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: String, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition: Some(success_condition), success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_success_conditions(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Vec<String>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions: Some(success_conditions), simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_simulation_scenario(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: String, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario: Some(simulation_scenario), simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_simulation_max_turns(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: i64, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns: Some(simulation_max_turns), simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_simulation_environment(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: String, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment: Some(simulation_environment), tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_tool_mock_config(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: SimulationToolMockBehaviorConfig, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config: Some(tool_mock_config), tool_mock_overrides, evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_tool_mock_overrides(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: HashMap<String, Vec<ToolResponseMockConfigOutput>>, evaluation_model: Option<Llm>, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides: Some(tool_mock_overrides), evaluation_model, simulated_user_model }
    }

    pub fn simulation_with_evaluation_model(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Llm, simulated_user_model: Option<Llm>) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model: Some(evaluation_model), simulated_user_model }
    }

    pub fn simulation_with_simulated_user_model(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, success_condition: Option<String>, success_conditions: Option<Vec<String>>, simulation_scenario: Option<String>, simulation_max_turns: Option<i64>, simulation_environment: Option<String>, tool_mock_config: Option<SimulationToolMockBehaviorConfig>, tool_mock_overrides: Option<HashMap<String, Vec<ToolResponseMockConfigOutput>>>, evaluation_model: Option<Llm>, simulated_user_model: Llm) -> Self {
        Self::Simulation { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, success_condition, success_conditions, simulation_scenario, simulation_max_turns, simulation_environment, tool_mock_config, tool_mock_overrides, evaluation_model, simulated_user_model: Some(simulated_user_model) }
    }

    pub fn tool_with_from_conversation_metadata(from_conversation_metadata: TestFromConversationMetadataOutput, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata: Some(from_conversation_metadata), dynamic_variables, chat_history, conversation_initiation_source, environment, tool_call_parameters, check_any_tool_matches }
    }

    pub fn tool_with_dynamic_variables(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: HashMap<String, serde_json::Value>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables: Some(dynamic_variables), chat_history, conversation_initiation_source, environment, tool_call_parameters, check_any_tool_matches }
    }

    pub fn tool_with_chat_history(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Vec<ConversationHistoryTranscriptCommonModelOutput>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables, chat_history: Some(chat_history), conversation_initiation_source, environment, tool_call_parameters, check_any_tool_matches }
    }

    pub fn tool_with_conversation_initiation_source(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: ConversationInitiationSource, environment: Option<String>, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source: Some(conversation_initiation_source), environment, tool_call_parameters, check_any_tool_matches }
    }

    pub fn tool_with_environment(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: String, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment: Some(environment), tool_call_parameters, check_any_tool_matches }
    }

    pub fn tool_with_tool_call_parameters(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, tool_call_parameters: UnitTestToolCallEvaluationModelOutput, check_any_tool_matches: Option<bool>) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, tool_call_parameters: Some(tool_call_parameters), check_any_tool_matches }
    }

    pub fn tool_with_check_any_tool_matches(from_conversation_metadata: Option<TestFromConversationMetadataOutput>, dynamic_variables: Option<HashMap<String, serde_json::Value>>, chat_history: Option<Vec<ConversationHistoryTranscriptCommonModelOutput>>, conversation_initiation_source: Option<ConversationInitiationSource>, environment: Option<String>, tool_call_parameters: Option<UnitTestToolCallEvaluationModelOutput>, check_any_tool_matches: bool) -> Self {
        Self::Tool { from_conversation_metadata, dynamic_variables, chat_history, conversation_initiation_source, environment, tool_call_parameters, check_any_tool_matches: Some(check_any_tool_matches) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
