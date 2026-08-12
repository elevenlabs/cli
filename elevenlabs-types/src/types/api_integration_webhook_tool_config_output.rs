pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiIntegrationWebhookToolConfigOutput {
    #[serde(default)]
    pub name: String,
    /// Description of when the tool should be used and what it does.
    #[serde(default)]
    pub description: String,
    /// The maximum time in seconds to wait for the tool call to complete. Must be between 5 and 300 seconds (inclusive).
    #[serde(default)]
    pub response_timeout_secs: i64,
    /// DEPRECATED: use `interruption_mode` instead. If true, the user will not be able to interrupt the agent while this tool is running.
    #[serde(default)]
    pub disable_interruptions: bool,
    /// Controls whether the user can interrupt the agent around this tool call. 'allow' (default) lets the user interrupt at any time, 'disable_during_tool' suppresses interruptions only while the tool is running, 'disable_during_tool_and_turn' suppresses interruptions while the tool runs and for the agent response that follows it.
    pub interruption_mode: ToolInterruptionMode,
    /// DEPRECATED: use `pre_tool_speech` instead. If true, the agent will speak before the tool call.
    #[serde(default)]
    pub force_pre_tool_speech: bool,
    /// Controls whether the agent speaks before this tool is called. 'auto' (default) decides based on recent tool latency, 'force' always asks the agent to speak, 'off' fully opts out regardless of latency.
    pub pre_tool_speech: PreToolSpeechMode,
    /// Configuration for extracting values from tool responses and assigning them to dynamic variables
    #[serde(default)]
    pub assignments: Vec<DynamicVariableAssignment>,
    /// Predefined tool call sound type to play during tool execution. If not specified, no tool call sound will be played.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// Determines when the tool call sound should play. 'auto' only plays when there's pre-tool speech, 'always' plays for every tool call.
    pub tool_call_sound_behavior: ToolCallSoundBehavior,
    /// Controls how tool errors are processed before being shared with the agent. 'auto' determines handling based on tool type (summarized for native integrations, hide for others), 'summarized' sends an LLM-generated summary, 'passthrough' sends the raw error, 'hide' does not share the error with the agent.
    pub tool_error_handling_mode: ToolErrorHandlingMode,
    /// Configuration for dynamic variables
    #[serde(default)]
    pub dynamic_variables: DynamicVariablesConfig,
    /// Determines when and how the tool executes: 'immediate' executes the tool right away when requested by the LLM, 'post_tool_speech' waits for the agent to finish speaking before executing, 'async' runs the tool in the background without blocking - best for long-running operations.
    pub execution_mode: ToolExecutionMode,
    /// The version of the API integration tool
    #[serde(default)]
    pub tool_version: String,
    #[serde(default)]
    pub api_integration_id: String,
    #[serde(default)]
    pub api_integration_connection_id: String,
    /// User overrides applied on top of the base api_schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_schema_overrides: Option<ApiIntegrationWebhookOverrides>,
}

impl ApiIntegrationWebhookToolConfigOutput {
    pub fn builder() -> ApiIntegrationWebhookToolConfigOutputBuilder {
        <ApiIntegrationWebhookToolConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiIntegrationWebhookToolConfigOutputBuilder {
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
    dynamic_variables: Option<DynamicVariablesConfig>,
    execution_mode: Option<ToolExecutionMode>,
    tool_version: Option<String>,
    api_integration_id: Option<String>,
    api_integration_connection_id: Option<String>,
    api_schema_overrides: Option<ApiIntegrationWebhookOverrides>,
}

impl ApiIntegrationWebhookToolConfigOutputBuilder {
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

    pub fn dynamic_variables(mut self, value: DynamicVariablesConfig) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn tool_version(mut self, value: impl Into<String>) -> Self {
        self.tool_version = Some(value.into());
        self
    }

    pub fn api_integration_id(mut self, value: impl Into<String>) -> Self {
        self.api_integration_id = Some(value.into());
        self
    }

    pub fn api_integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.api_integration_connection_id = Some(value.into());
        self
    }

    pub fn api_schema_overrides(mut self, value: ApiIntegrationWebhookOverrides) -> Self {
        self.api_schema_overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiIntegrationWebhookToolConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ApiIntegrationWebhookToolConfigOutputBuilder::name)
    /// - [`description`](ApiIntegrationWebhookToolConfigOutputBuilder::description)
    /// - [`response_timeout_secs`](ApiIntegrationWebhookToolConfigOutputBuilder::response_timeout_secs)
    /// - [`disable_interruptions`](ApiIntegrationWebhookToolConfigOutputBuilder::disable_interruptions)
    /// - [`interruption_mode`](ApiIntegrationWebhookToolConfigOutputBuilder::interruption_mode)
    /// - [`force_pre_tool_speech`](ApiIntegrationWebhookToolConfigOutputBuilder::force_pre_tool_speech)
    /// - [`pre_tool_speech`](ApiIntegrationWebhookToolConfigOutputBuilder::pre_tool_speech)
    /// - [`assignments`](ApiIntegrationWebhookToolConfigOutputBuilder::assignments)
    /// - [`tool_call_sound_behavior`](ApiIntegrationWebhookToolConfigOutputBuilder::tool_call_sound_behavior)
    /// - [`tool_error_handling_mode`](ApiIntegrationWebhookToolConfigOutputBuilder::tool_error_handling_mode)
    /// - [`dynamic_variables`](ApiIntegrationWebhookToolConfigOutputBuilder::dynamic_variables)
    /// - [`execution_mode`](ApiIntegrationWebhookToolConfigOutputBuilder::execution_mode)
    /// - [`tool_version`](ApiIntegrationWebhookToolConfigOutputBuilder::tool_version)
    /// - [`api_integration_id`](ApiIntegrationWebhookToolConfigOutputBuilder::api_integration_id)
    /// - [`api_integration_connection_id`](ApiIntegrationWebhookToolConfigOutputBuilder::api_integration_connection_id)
    pub fn build(self) -> Result<ApiIntegrationWebhookToolConfigOutput, BuildError> {
        Ok(ApiIntegrationWebhookToolConfigOutput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            response_timeout_secs: self.response_timeout_secs.ok_or_else(|| BuildError::missing_field("response_timeout_secs"))?,
            disable_interruptions: self.disable_interruptions.ok_or_else(|| BuildError::missing_field("disable_interruptions"))?,
            interruption_mode: self.interruption_mode.ok_or_else(|| BuildError::missing_field("interruption_mode"))?,
            force_pre_tool_speech: self.force_pre_tool_speech.ok_or_else(|| BuildError::missing_field("force_pre_tool_speech"))?,
            pre_tool_speech: self.pre_tool_speech.ok_or_else(|| BuildError::missing_field("pre_tool_speech"))?,
            assignments: self.assignments.ok_or_else(|| BuildError::missing_field("assignments"))?,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior.ok_or_else(|| BuildError::missing_field("tool_call_sound_behavior"))?,
            tool_error_handling_mode: self.tool_error_handling_mode.ok_or_else(|| BuildError::missing_field("tool_error_handling_mode"))?,
            dynamic_variables: self.dynamic_variables.ok_or_else(|| BuildError::missing_field("dynamic_variables"))?,
            execution_mode: self.execution_mode.ok_or_else(|| BuildError::missing_field("execution_mode"))?,
            tool_version: self.tool_version.ok_or_else(|| BuildError::missing_field("tool_version"))?,
            api_integration_id: self.api_integration_id.ok_or_else(|| BuildError::missing_field("api_integration_id"))?,
            api_integration_connection_id: self.api_integration_connection_id.ok_or_else(|| BuildError::missing_field("api_integration_connection_id"))?,
            api_schema_overrides: self.api_schema_overrides,
        })
    }
}
