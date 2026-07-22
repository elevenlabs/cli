pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServerConfigUpdateRequestModel {
    /// The approval mode to set for the MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpApprovalPolicy>,
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
    /// Predefined tool call sound type to play during tool execution for all tools from this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// Determines when the tool call sound should play for all tools from this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    /// If set, overrides the server's execution_mode setting for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    /// The maximum time in seconds to wait for each MCP tool call to complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout_secs: Option<i64>,
    /// The headers to include in requests to the MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, Option<McpServerConfigUpdateRequestModelRequestHeadersValue>>>,
    /// Whether to disable HTTP compression for this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_compression: Option<bool>,
    /// Optional secret token for authentication with this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<ConvAiSecretLocator>,
    /// Optional auth connection to use for authentication with this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_connection: Option<McpServerConfigUpdateRequestModelAuthConnection>,
}

impl McpServerConfigUpdateRequestModel {
    pub fn builder() -> McpServerConfigUpdateRequestModelBuilder {
        <McpServerConfigUpdateRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerConfigUpdateRequestModelBuilder {
    approval_policy: Option<McpApprovalPolicy>,
    force_pre_tool_speech: Option<bool>,
    pre_tool_speech: Option<PreToolSpeechMode>,
    disable_interruptions: Option<bool>,
    interruption_mode: Option<ToolInterruptionMode>,
    tool_call_sound: Option<ToolCallSoundType>,
    tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    execution_mode: Option<ToolExecutionMode>,
    response_timeout_secs: Option<i64>,
    request_headers: Option<HashMap<String, Option<McpServerConfigUpdateRequestModelRequestHeadersValue>>>,
    disable_compression: Option<bool>,
    secret_token: Option<ConvAiSecretLocator>,
    auth_connection: Option<McpServerConfigUpdateRequestModelAuthConnection>,
}

impl McpServerConfigUpdateRequestModelBuilder {
    pub fn approval_policy(mut self, value: McpApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
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

    pub fn disable_interruptions(mut self, value: bool) -> Self {
        self.disable_interruptions = Some(value);
        self
    }

    pub fn interruption_mode(mut self, value: ToolInterruptionMode) -> Self {
        self.interruption_mode = Some(value);
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

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn response_timeout_secs(mut self, value: i64) -> Self {
        self.response_timeout_secs = Some(value);
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, Option<McpServerConfigUpdateRequestModelRequestHeadersValue>>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn disable_compression(mut self, value: bool) -> Self {
        self.disable_compression = Some(value);
        self
    }

    pub fn secret_token(mut self, value: ConvAiSecretLocator) -> Self {
        self.secret_token = Some(value);
        self
    }

    pub fn auth_connection(mut self, value: McpServerConfigUpdateRequestModelAuthConnection) -> Self {
        self.auth_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerConfigUpdateRequestModel`].
    pub fn build(self) -> Result<McpServerConfigUpdateRequestModel, BuildError> {
        Ok(McpServerConfigUpdateRequestModel {
            approval_policy: self.approval_policy,
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            execution_mode: self.execution_mode,
            response_timeout_secs: self.response_timeout_secs,
            request_headers: self.request_headers,
            disable_compression: self.disable_compression,
            secret_token: self.secret_token,
            auth_connection: self.auth_connection,
        })
    }
}

