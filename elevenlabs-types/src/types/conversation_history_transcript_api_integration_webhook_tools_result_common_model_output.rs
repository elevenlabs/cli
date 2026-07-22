pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub result_value: String,
    #[serde(default)]
    pub is_error: bool,
    #[serde(default)]
    pub is_blocked: bool,
    #[serde(default)]
    pub tool_has_been_called: bool,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub tool_latency_secs: f64,
    #[serde(default)]
    pub error_type: String,
    #[serde(default)]
    pub raw_error_message: String,
    #[serde(default)]
    pub dynamic_variable_updates: Vec<DynamicVariableUpdateCommonModel>,
    pub r#type: String,
    #[serde(default)]
    pub integration_id: String,
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    pub integration_connection_id: String,
}

impl ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput {
    pub fn builder() -> ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder {
        <ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder {
    request_id: Option<String>,
    tool_name: Option<String>,
    result_value: Option<String>,
    is_error: Option<bool>,
    is_blocked: Option<bool>,
    tool_has_been_called: Option<bool>,
    tool_latency_secs: Option<f64>,
    error_type: Option<String>,
    raw_error_message: Option<String>,
    dynamic_variable_updates: Option<Vec<DynamicVariableUpdateCommonModel>>,
    r#type: Option<String>,
    integration_id: Option<String>,
    credential_id: Option<String>,
    integration_connection_id: Option<String>,
}

impl ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn result_value(mut self, value: impl Into<String>) -> Self {
        self.result_value = Some(value.into());
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn is_blocked(mut self, value: bool) -> Self {
        self.is_blocked = Some(value);
        self
    }

    pub fn tool_has_been_called(mut self, value: bool) -> Self {
        self.tool_has_been_called = Some(value);
        self
    }

    pub fn tool_latency_secs(mut self, value: f64) -> Self {
        self.tool_latency_secs = Some(value);
        self
    }

    pub fn error_type(mut self, value: impl Into<String>) -> Self {
        self.error_type = Some(value.into());
        self
    }

    pub fn raw_error_message(mut self, value: impl Into<String>) -> Self {
        self.raw_error_message = Some(value.into());
        self
    }

    pub fn dynamic_variable_updates(mut self, value: Vec<DynamicVariableUpdateCommonModel>) -> Self {
        self.dynamic_variable_updates = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn integration_connection_id(mut self, value: impl Into<String>) -> Self {
        self.integration_connection_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::request_id)
    /// - [`tool_name`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::tool_name)
    /// - [`result_value`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::result_value)
    /// - [`is_error`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::is_error)
    /// - [`is_blocked`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::is_blocked)
    /// - [`tool_has_been_called`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::tool_has_been_called)
    /// - [`tool_latency_secs`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::tool_latency_secs)
    /// - [`error_type`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::error_type)
    /// - [`raw_error_message`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::raw_error_message)
    /// - [`dynamic_variable_updates`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::dynamic_variable_updates)
    /// - [`r#type`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::r#type)
    /// - [`integration_id`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::integration_id)
    /// - [`credential_id`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::credential_id)
    /// - [`integration_connection_id`](ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutputBuilder::integration_connection_id)
    pub fn build(self) -> Result<ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput, BuildError> {
        Ok(ConversationHistoryTranscriptApiIntegrationWebhookToolsResultCommonModelOutput {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            result_value: self.result_value.ok_or_else(|| BuildError::missing_field("result_value"))?,
            is_error: self.is_error.ok_or_else(|| BuildError::missing_field("is_error"))?,
            is_blocked: self.is_blocked.ok_or_else(|| BuildError::missing_field("is_blocked"))?,
            tool_has_been_called: self.tool_has_been_called.ok_or_else(|| BuildError::missing_field("tool_has_been_called"))?,
            tool_latency_secs: self.tool_latency_secs.ok_or_else(|| BuildError::missing_field("tool_latency_secs"))?,
            error_type: self.error_type.ok_or_else(|| BuildError::missing_field("error_type"))?,
            raw_error_message: self.raw_error_message.ok_or_else(|| BuildError::missing_field("raw_error_message"))?,
            dynamic_variable_updates: self.dynamic_variable_updates.ok_or_else(|| BuildError::missing_field("dynamic_variable_updates"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            integration_id: self.integration_id.ok_or_else(|| BuildError::missing_field("integration_id"))?,
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            integration_connection_id: self.integration_connection_id.ok_or_else(|| BuildError::missing_field("integration_connection_id"))?,
        })
    }
}
