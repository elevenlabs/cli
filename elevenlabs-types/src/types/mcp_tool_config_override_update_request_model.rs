pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpToolConfigOverrideUpdateRequestModel {
    /// DEPRECATED: use `pre_tool_speech` instead. If set, overrides the server's force_pre_tool_speech setting for this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_pre_tool_speech: Option<bool>,
    /// If set, overrides the server's pre_tool_speech setting for this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_tool_speech: Option<PreToolSpeechMode>,
    /// DEPRECATED: use `interruption_mode` instead. If set, overrides the server's disable_interruptions setting for this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_interruptions: Option<bool>,
    /// If set, overrides the server's interruption_mode setting for this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_mode: Option<ToolInterruptionMode>,
    /// Overrides the server's tool_call_sound setting for this tool. A sound name plays that sound; 'off' overrides to no sound (silence); null means do not override (inherit the server default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<McpToolConfigOverrideUpdateRequestModelToolCallSound>,
    /// If set, overrides the server's tool_call_sound_behavior setting for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    /// If set, overrides the server's execution_mode setting for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    /// If set, overrides the server's response timeout for this MCP tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout_secs: Option<i64>,
    /// Dynamic variable assignments for this MCP tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignments: Option<Vec<DynamicVariableAssignment>>,
    /// Mapping of json path to input override configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_overrides: Option<HashMap<String, Option<McpToolConfigOverrideUpdateRequestModelInputOverridesValue>>>,
    /// Mock responses with optional parameter conditions. Evaluated top-to-bottom; first match wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mocks: Option<Vec<ToolResponseMockConfigInput>>,
    /// Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production.
    #[serde(skip)]
    pub environment: Option<String>,
}

impl McpToolConfigOverrideUpdateRequestModel {
    pub fn builder() -> McpToolConfigOverrideUpdateRequestModelBuilder {
        <McpToolConfigOverrideUpdateRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolConfigOverrideUpdateRequestModelBuilder {
    force_pre_tool_speech: Option<bool>,
    pre_tool_speech: Option<PreToolSpeechMode>,
    disable_interruptions: Option<bool>,
    interruption_mode: Option<ToolInterruptionMode>,
    tool_call_sound: Option<McpToolConfigOverrideUpdateRequestModelToolCallSound>,
    tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    execution_mode: Option<ToolExecutionMode>,
    response_timeout_secs: Option<i64>,
    assignments: Option<Vec<DynamicVariableAssignment>>,
    input_overrides: Option<HashMap<String, Option<McpToolConfigOverrideUpdateRequestModelInputOverridesValue>>>,
    response_mocks: Option<Vec<ToolResponseMockConfigInput>>,
    environment: Option<String>,
}

impl McpToolConfigOverrideUpdateRequestModelBuilder {
    pub fn force_pre_tool_speech(mut self, value: bool) -> Self {
        self.force_pre_tool_speech = Some(value);
        self
    }

    pub fn pre_tool_speech(mut self, value: PreToolSpeechMode) -> Self {
        self.pre_tool_speech = Some(value);
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

    pub fn tool_call_sound(mut self, value: McpToolConfigOverrideUpdateRequestModelToolCallSound) -> Self {
        self.tool_call_sound = Some(value);
        self
    }

    pub fn tool_call_sound_behavior(mut self, value: ToolCallSoundBehavior) -> Self {
        self.tool_call_sound_behavior = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn response_timeout_secs(mut self, value: i64) -> Self {
        self.response_timeout_secs = Some(value);
        self
    }

    pub fn assignments(mut self, value: Vec<DynamicVariableAssignment>) -> Self {
        self.assignments = Some(value);
        self
    }

    pub fn input_overrides(mut self, value: HashMap<String, Option<McpToolConfigOverrideUpdateRequestModelInputOverridesValue>>) -> Self {
        self.input_overrides = Some(value);
        self
    }

    pub fn response_mocks(mut self, value: Vec<ToolResponseMockConfigInput>) -> Self {
        self.response_mocks = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpToolConfigOverrideUpdateRequestModel`].
    pub fn build(self) -> Result<McpToolConfigOverrideUpdateRequestModel, BuildError> {
        Ok(McpToolConfigOverrideUpdateRequestModel {
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            execution_mode: self.execution_mode,
            response_timeout_secs: self.response_timeout_secs,
            assignments: self.assignments,
            input_overrides: self.input_overrides,
            response_mocks: self.response_mocks,
            environment: self.environment,
        })
    }
}

