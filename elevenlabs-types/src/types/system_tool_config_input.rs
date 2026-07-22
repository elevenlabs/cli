pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A system tool is a tool that is used to call a system method in the server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemToolConfigInput {
    /// The type of tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Description of when the tool should be used and what it does. Leave empty to use the default description that's optimized for the specific tool type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The maximum time in seconds to wait for the tool call to complete.
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
    pub params: SystemToolConfigInputParams,
}

impl SystemToolConfigInput {
    pub fn builder() -> SystemToolConfigInputBuilder {
        <SystemToolConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SystemToolConfigInputBuilder {
    r#type: Option<String>,
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
    params: Option<SystemToolConfigInputParams>,
}

impl SystemToolConfigInputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

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

    pub fn params(mut self, value: SystemToolConfigInputParams) -> Self {
        self.params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SystemToolConfigInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SystemToolConfigInputBuilder::name)
    /// - [`params`](SystemToolConfigInputBuilder::params)
    pub fn build(self) -> Result<SystemToolConfigInput, BuildError> {
        Ok(SystemToolConfigInput {
            r#type: self.r#type,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            response_timeout_secs: self.response_timeout_secs,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            assignments: self.assignments,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            tool_error_handling_mode: self.tool_error_handling_mode,
            params: self.params.ok_or_else(|| BuildError::missing_field("params"))?,
        })
    }
}
