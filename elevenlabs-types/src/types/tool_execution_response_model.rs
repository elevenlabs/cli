pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolExecutionResponseModel {
    /// The ID of the tool that was executed
    #[serde(default)]
    pub tool_id: String,
    /// The request/call ID associated with this tool execution
    #[serde(default)]
    pub tool_request_id: String,
    /// The ID of the conversation where the tool was executed
    #[serde(default)]
    pub conversation_id: String,
    /// The ID of the agent that ran the tool
    #[serde(default)]
    pub agent_id: String,
    /// The branch ID if the agent has branches
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Unix timestamp when the tool was executed
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub timestamp: f64,
    /// How long the tool execution took
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latency_secs: f64,
    /// Whether the tool execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// LLM-extracted parameters sent to the tool (JSON string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payload: Option<String>,
    /// Response returned by the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_payload: Option<String>,
    /// Error message if the tool execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Error category (internal, customer_config, customer_auth, external_server, external_client, client_timeout, unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_details: Option<ToolExecutionResponseModelToolCallDetails>,
}

impl ToolExecutionResponseModel {
    pub fn builder() -> ToolExecutionResponseModelBuilder {
        <ToolExecutionResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionResponseModelBuilder {
    tool_id: Option<String>,
    tool_request_id: Option<String>,
    conversation_id: Option<String>,
    agent_id: Option<String>,
    branch_id: Option<String>,
    timestamp: Option<f64>,
    latency_secs: Option<f64>,
    is_error: Option<bool>,
    request_payload: Option<String>,
    response_payload: Option<String>,
    error_message: Option<String>,
    error_type: Option<String>,
    id: Option<String>,
    tool_call_details: Option<ToolExecutionResponseModelToolCallDetails>,
}

impl ToolExecutionResponseModelBuilder {
    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    pub fn tool_request_id(mut self, value: impl Into<String>) -> Self {
        self.tool_request_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: f64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn latency_secs(mut self, value: f64) -> Self {
        self.latency_secs = Some(value);
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn request_payload(mut self, value: impl Into<String>) -> Self {
        self.request_payload = Some(value.into());
        self
    }

    pub fn response_payload(mut self, value: impl Into<String>) -> Self {
        self.response_payload = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tool_call_details(mut self, value: ToolExecutionResponseModelToolCallDetails) -> Self {
        self.tool_call_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolExecutionResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_id`](ToolExecutionResponseModelBuilder::tool_id)
    /// - [`tool_request_id`](ToolExecutionResponseModelBuilder::tool_request_id)
    /// - [`conversation_id`](ToolExecutionResponseModelBuilder::conversation_id)
    /// - [`agent_id`](ToolExecutionResponseModelBuilder::agent_id)
    /// - [`timestamp`](ToolExecutionResponseModelBuilder::timestamp)
    /// - [`latency_secs`](ToolExecutionResponseModelBuilder::latency_secs)
    /// - [`id`](ToolExecutionResponseModelBuilder::id)
    pub fn build(self) -> Result<ToolExecutionResponseModel, BuildError> {
        Ok(ToolExecutionResponseModel {
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
            tool_request_id: self.tool_request_id.ok_or_else(|| BuildError::missing_field("tool_request_id"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            latency_secs: self.latency_secs.ok_or_else(|| BuildError::missing_field("latency_secs"))?,
            is_error: self.is_error,
            request_payload: self.request_payload,
            response_payload: self.response_payload,
            error_message: self.error_message,
            error_type: self.error_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tool_call_details: self.tool_call_details,
        })
    }
}
