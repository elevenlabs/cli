pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpServerConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpApprovalPolicy>,
    /// List of tool approval hashes for per-tool approval when approval_policy is REQUIRE_APPROVAL_PER_TOOL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_approval_hashes: Option<Vec<McpToolApprovalHash>>,
    /// The transport type used to connect to the MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<McpServerTransport>,
    /// The URL of the MCP server, if this contains a secret please store as a workspace secret, otherwise store as a plain string. Must use https
    pub url: McpServerConfigOutputUrl,
    /// The secret token (Authorization header) stored as a workspace secret or in-place secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<McpServerConfigOutputSecretToken>,
    /// The headers included in the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<HashMap<String, McpServerConfigOutputRequestHeadersValue>>,
    /// Entries sent in the MCP `_meta` field of tools/call requests. Values may be JSON scalars, or references to a workspace secret, dynamic variable, or environment variable resolved per call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_meta: Option<HashMap<String, McpServerConfigOutputRequestMetaValue>>,
    /// Optional auth connection to use for authentication with this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_connection: Option<McpServerConfigOutputAuthConnection>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// DEPRECATED: use `pre_tool_speech` instead. If true, all tools from this MCP server will require pre-tool execution speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_pre_tool_speech: Option<bool>,
    /// Controls whether the agent speaks before this tool is called. 'auto' (default) decides based on recent tool latency, 'force' always asks the agent to speak, 'off' fully opts out regardless of latency. Applies to every tool from this MCP server unless overridden per tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_tool_speech: Option<PreToolSpeechMode>,
    /// DEPRECATED: use `interruption_mode` instead. If true, the user will not be able to interrupt the agent while any tool from this MCP server is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_interruptions: Option<bool>,
    /// Controls whether the user can interrupt the agent around this tool call. 'allow' (default) lets the user interrupt at any time, 'disable_during_tool' suppresses interruptions only while the tool is running, 'disable_during_tool_and_turn' suppresses interruptions while the tool runs and for the agent response that follows it. Applies to every tool from this MCP server unless overridden per tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_mode: Option<ToolInterruptionMode>,
    /// Predefined tool call sound type to play during tool execution for all tools from this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound: Option<ToolCallSoundType>,
    /// Determines when the tool call sound should play for all tools from this MCP server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    /// Determines when and how all tools from this MCP server execute: 'immediate' executes the tool right away when requested by the LLM, 'post_tool_speech' waits for the agent to finish speaking before executing, 'async' runs the tool in the background without blocking - best for long-running operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    /// The maximum time in seconds to wait for each MCP tool call to complete. Must be between 5 and 300 seconds (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout_secs: Option<i64>,
    /// List of per-tool configuration overrides that override the server-level defaults for specific tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config_overrides: Option<Vec<McpToolConfigOverrideOutput>>,
    /// Whether to disable HTTP compression for this MCP server. Enable this if the server does not support compressed responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_compression: Option<bool>,
}

impl McpServerConfigOutput {
    pub fn builder() -> McpServerConfigOutputBuilder {
        <McpServerConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerConfigOutputBuilder {
    approval_policy: Option<McpApprovalPolicy>,
    tool_approval_hashes: Option<Vec<McpToolApprovalHash>>,
    transport: Option<McpServerTransport>,
    url: Option<McpServerConfigOutputUrl>,
    secret_token: Option<McpServerConfigOutputSecretToken>,
    request_headers: Option<HashMap<String, McpServerConfigOutputRequestHeadersValue>>,
    request_meta: Option<HashMap<String, McpServerConfigOutputRequestMetaValue>>,
    auth_connection: Option<McpServerConfigOutputAuthConnection>,
    name: Option<String>,
    description: Option<String>,
    force_pre_tool_speech: Option<bool>,
    pre_tool_speech: Option<PreToolSpeechMode>,
    disable_interruptions: Option<bool>,
    interruption_mode: Option<ToolInterruptionMode>,
    tool_call_sound: Option<ToolCallSoundType>,
    tool_call_sound_behavior: Option<ToolCallSoundBehavior>,
    execution_mode: Option<ToolExecutionMode>,
    response_timeout_secs: Option<i64>,
    tool_config_overrides: Option<Vec<McpToolConfigOverrideOutput>>,
    disable_compression: Option<bool>,
}

impl McpServerConfigOutputBuilder {
    pub fn approval_policy(mut self, value: McpApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    pub fn tool_approval_hashes(mut self, value: Vec<McpToolApprovalHash>) -> Self {
        self.tool_approval_hashes = Some(value);
        self
    }

    pub fn transport(mut self, value: McpServerTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn url(mut self, value: McpServerConfigOutputUrl) -> Self {
        self.url = Some(value);
        self
    }

    pub fn secret_token(mut self, value: McpServerConfigOutputSecretToken) -> Self {
        self.secret_token = Some(value);
        self
    }

    pub fn request_headers(mut self, value: HashMap<String, McpServerConfigOutputRequestHeadersValue>) -> Self {
        self.request_headers = Some(value);
        self
    }

    pub fn request_meta(mut self, value: HashMap<String, McpServerConfigOutputRequestMetaValue>) -> Self {
        self.request_meta = Some(value);
        self
    }

    pub fn auth_connection(mut self, value: McpServerConfigOutputAuthConnection) -> Self {
        self.auth_connection = Some(value);
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

    pub fn tool_config_overrides(mut self, value: Vec<McpToolConfigOverrideOutput>) -> Self {
        self.tool_config_overrides = Some(value);
        self
    }

    pub fn disable_compression(mut self, value: bool) -> Self {
        self.disable_compression = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerConfigOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](McpServerConfigOutputBuilder::url)
    /// - [`name`](McpServerConfigOutputBuilder::name)
    pub fn build(self) -> Result<McpServerConfigOutput, BuildError> {
        Ok(McpServerConfigOutput {
            approval_policy: self.approval_policy,
            tool_approval_hashes: self.tool_approval_hashes,
            transport: self.transport,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            secret_token: self.secret_token,
            request_headers: self.request_headers,
            request_meta: self.request_meta,
            auth_connection: self.auth_connection,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            force_pre_tool_speech: self.force_pre_tool_speech,
            pre_tool_speech: self.pre_tool_speech,
            disable_interruptions: self.disable_interruptions,
            interruption_mode: self.interruption_mode,
            tool_call_sound: self.tool_call_sound,
            tool_call_sound_behavior: self.tool_call_sound_behavior,
            execution_mode: self.execution_mode,
            response_timeout_secs: self.response_timeout_secs,
            tool_config_overrides: self.tool_config_overrides,
            disable_compression: self.disable_compression,
        })
    }
}
