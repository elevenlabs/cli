pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An MCP tool configuration that can be used to call MCP servers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpToolConfigOutput {
    #[serde(default)]
    pub name: String,
    /// Description of when the tool should be used and what it does.
    #[serde(default)]
    pub description: String,
    /// The maximum time in seconds to wait for the MCP tool call to complete. Must be between 5 and 300 seconds (inclusive).
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
    /// The type of MCP tool
    pub integration_type: IntegrationType,
    /// Schema for any parameters the LLM needs to provide to the MCP tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ObjectJsonSchemaPropertyOutput>,
    /// The approval policy for the MCP tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpApprovalPolicy>,
    /// The name of the MCP tool to call
    #[serde(default)]
    pub mcp_tool_name: String,
    /// The description of the MCP tool to call
    #[serde(default)]
    pub mcp_tool_description: String,
    /// The id of the MCP server to call
    #[serde(default)]
    pub mcp_server_id: String,
    /// The name of the MCP server to call
    #[serde(default)]
    pub mcp_server_name: String,
    /// Original inputSchema dict for consistent hashing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_input_schema: Option<HashMap<String, serde_json::Value>>,
    /// Determines when and how the tool executes: 'immediate' executes the tool right away when requested by the LLM, 'post_tool_speech' waits for the agent to finish speaking before executing, 'async' runs the tool in the background without blocking - best for long-running operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    /// Input parameter overrides for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_overrides: Option<HashMap<String, Option<McpToolConfigOutputInputOverridesValue>>>,
}

impl McpToolConfigOutput {
    pub fn builder() -> McpToolConfigOutputBuilder {
        <McpToolConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolConfigOutputBuilder {
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
    integration_type: Option<IntegrationType>,
    parameters: Option<ObjectJsonSchemaPropertyOutput>,
    approval_policy: Option<McpApprovalPolicy>,
    mcp_tool_name: Option<String>,
    mcp_tool_description: Option<String>,
    mcp_server_id: Option<String>,
    mcp_server_name: Option<String>,
    mcp_input_schema: Option<HashMap<String, serde_json::Value>>,
    execution_mode: Option<ToolExecutionMode>,
    input_overrides: Option<HashMap<String, Option<McpToolConfigOutputInputOverridesValue>>>,
}

impl McpToolConfigOutputBuilder {
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

    pub fn integration_type(mut self, value: IntegrationType) -> Self {
        self.integration_type = Some(value);
        self
    }

    pub fn parameters(mut self, value: ObjectJsonSchemaPropertyOutput) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn approval_policy(mut self, value: McpApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    pub fn mcp_tool_name(mut self, value: impl Into<String>) -> Self {
        self.mcp_tool_name = Some(value.into());
        self
    }

    pub fn mcp_tool_description(mut self, value: impl Into<String>) -> Self {
        self.mcp_tool_description = Some(value.into());
        self
    }

    pub fn mcp_server_id(mut self, value: impl Into<String>) -> Self {
        self.mcp_server_id = Some(value.into());
        self
    }

    pub fn mcp_server_name(mut self, value: impl Into<String>) -> Self {
        self.mcp_server_name = Some(value.into());
        self
    }

    pub fn mcp_input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.mcp_input_schema = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn input_overrides(mut self, value: HashMap<String, Option<McpToolConfigOutputInputOverridesValue>>) -> Self {
        self.input_overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](McpToolConfigOutputBuilder::name)
    /// - [`description`](McpToolConfigOutputBuilder::description)
    /// - [`integration_type`](McpToolConfigOutputBuilder::integration_type)
    /// - [`mcp_tool_name`](McpToolConfigOutputBuilder::mcp_tool_name)
    /// - [`mcp_tool_description`](McpToolConfigOutputBuilder::mcp_tool_description)
    /// - [`mcp_server_id`](McpToolConfigOutputBuilder::mcp_server_id)
    /// - [`mcp_server_name`](McpToolConfigOutputBuilder::mcp_server_name)
    pub fn build(self) -> Result<McpToolConfigOutput, BuildError> {
        Ok(McpToolConfigOutput {
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
            integration_type: self.integration_type.ok_or_else(|| BuildError::missing_field("integration_type"))?,
            parameters: self.parameters,
            approval_policy: self.approval_policy,
            mcp_tool_name: self.mcp_tool_name.ok_or_else(|| BuildError::missing_field("mcp_tool_name"))?,
            mcp_tool_description: self.mcp_tool_description.ok_or_else(|| BuildError::missing_field("mcp_tool_description"))?,
            mcp_server_id: self.mcp_server_id.ok_or_else(|| BuildError::missing_field("mcp_server_id"))?,
            mcp_server_name: self.mcp_server_name.ok_or_else(|| BuildError::missing_field("mcp_server_name"))?,
            mcp_input_schema: self.mcp_input_schema,
            execution_mode: self.execution_mode,
            input_overrides: self.input_overrides,
        })
    }
}
