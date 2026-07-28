pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A client tool is one that sends an event to the user's client to trigger something client side
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClientToolConfigInput {
    #[serde(default)]
    pub name: String,
    /// Description of when the tool should be used and what it does.
    #[serde(default)]
    pub description: String,
    /// The maximum time in seconds to wait for the tool call to complete. Must be between 1 and 120 seconds (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout_secs: Option<i64>,
    /// DEPRECATED: use `interruption_mode` instead. If true, the user will not be able to interrupt the agent while this tool is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_interruptions: Option<bool>,
    /// Controls whether the user can interrupt the agent around this tool call. 'allow' (default) lets the user interrupt at any time, 'disable_during_tool' suppresses interruptions only while the tool is running, 'disable_during_tool_and_turn' suppresses interruptions while the tool runs and for the agent response that follows it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_mode: Option<ToolInterruptionMode>,
    /// DEPRECATED: use `pre_tool_speech` instead. If true, the agent will speak before the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_pre_tool_speech: Option<bool>,
    /// Controls whether the agent speaks before this tool is called. 'auto' (default) decides based on recent tool latency, 'force' always asks the agent to speak, 'off' fully opts out regardless of latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_tool_speech: Option<PreToolSpeechMode>,
    /// Configuration for extracting values from tool responses and assigning them to dynamic variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<DynamicVariableAssignment>>,
    /// Predefined tool call sound type to play during tool execution. If not specified, no tool call sound will be played.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// Determines when the tool call sound should play. 'auto' only plays when there's pre-tool speech, 'always' plays for every tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    /// Controls how tool errors are processed before being shared with the agent. 'auto' determines handling based on tool type (summarized for native integrations, hide for others), 'summarized' sends an LLM-generated summary, 'passthrough' sends the raw error, 'hide' does not share the error with the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_error_handling_mode: Option<ToolErrorHandlingMode>,
    /// Schema for any parameters to pass to the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ObjectJsonSchemaPropertyInput>,
    /// If true, calling this tool should block the conversation until the client responds with some response which is passed to the llm. If false then we will continue the conversation without waiting for the client to respond, this is useful to show content to a user but not block the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expects_response: Option<bool>,
    /// Configuration for dynamic variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<DynamicVariablesConfig>,
    /// Determines when and how the tool executes: 'immediate' executes the tool right away when requested by the LLM, 'post_tool_speech' waits for the agent to finish speaking before executing, 'async' runs the tool in the background without blocking - best for long-running operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
}

impl ClientToolConfigInput {
    pub fn builder() -> ClientToolConfigInputBuilder {
        <ClientToolConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClientToolConfigInputBuilder {
    name: Option<String>,
    description: Option<String>,
    response_timeout_secs: Option<i64>,
    disable_interruptions: Option<bool>,
    interruption_mode: Option<ToolInterruptionMode>,
    force_pre_tool_speech: Option<bool>,
    pre_tool_speech: Option<PreToolSpeechMode>,
    assignments: Option<Vec<DynamicVariableAssignment>>,
    tool_call_sound: Option<ToolCallSoundType>,
    tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    tool_error_handling_mode: Option<ToolErrorHandlingMode>,
    parameters: Option<ObjectJsonSchemaPropertyInput>,
    expects_response: Option<bool>,
    dynamic_variables: Option<DynamicVariablesConfig>,
    execution_mode: Option<ToolExecutionMode>,
}

impl ClientToolConfigInputBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn response_timeout_secs(mut self, value: i64) -> Self {
        self.response_timeout_secs = Some(value);
        self
    }

    pub fn disable_interruptions(mut self, value: bool) -> Self {
        self.disable_interruptions = Some(value);
        self
    }

    pub fn interruption_mode(mut self, value: ToolInterruptionMode) -> Self {
        self.interruption_mode = Some(value);
        self
    }

    pub fn force_pre_tool_speech(mut self, value: bool) -> Self {
        self.force_pre_tool_speech = Some(value);
        self
    }

    pub fn pre_tool_speech(mut self, value: PreToolSpeechMode) -> Self {
        self.pre_tool_speech = Some(value);
        self
    }

    pub fn assignments(mut self, value: Vec<DynamicVariableAssignment>) -> Self {
        self.assignments = Some(value);
        self
    }

    pub fn tool_call_sound(mut self, value: ToolCallSoundType) -> Self {
        self.tool_call_sound = Some(value);
        self
    }

    pub fn tool_call_sound_behavior(mut self, value: ToolCallSoundBehavior) -> Self {
        self.tool_call_sound_behavior = Some(value);
        self
    }

    pub fn tool_error_handling_mode(mut self, value: ToolErrorHandlingMode) -> Self {
        self.tool_error_handling_mode = Some(value);
        self
    }

    pub fn parameters(mut self, value: ObjectJsonSchemaPropertyInput) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn expects_response(mut self, value: bool) -> Self {
        self.expects_response = Some(value);
        self
    }

    pub fn dynamic_variables(mut self, value: DynamicVariablesConfig) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClientToolConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ClientToolConfigInputBuilder::name)
    /// - [`description`](ClientToolConfigInputBuilder::description)
    pub fn build(self) -> Result<ClientToolConfigInput, BuildError> {
        Ok(ClientToolConfigInput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            response_timeout_secs: self.response_timeout_secs,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            assignments: self.assignments,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            tool_error_handling_mode: self.tool_error_handling_mode,
            parameters: self.parameters,
            expects_response: self.expects_response,
            dynamic_variables: self.dynamic_variables,
            execution_mode: self.execution_mode,
        })
    }
}
