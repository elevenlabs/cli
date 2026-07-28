pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryTranscriptOtherToolsResultCommonModel {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub result_value: String,
    #[serde(default)]
    pub is_error: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
    #[serde(default)]
    pub tool_has_been_called: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tool_latency_secs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variable_updates: Option<Vec<DynamicVariableUpdateCommonModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ConversationHistoryTranscriptOtherToolsResultCommonModelType>,
}

impl ConversationHistoryTranscriptOtherToolsResultCommonModel {
    pub fn builder() -> ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder {
        <ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder {
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
    r#type: Option<ConversationHistoryTranscriptOtherToolsResultCommonModelType>,
}

impl ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder {
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

    pub fn r#type(mut self, value: ConversationHistoryTranscriptOtherToolsResultCommonModelType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptOtherToolsResultCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder::request_id)
    /// - [`tool_name`](ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder::tool_name)
    /// - [`result_value`](ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder::result_value)
    /// - [`is_error`](ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder::is_error)
    /// - [`tool_has_been_called`](ConversationHistoryTranscriptOtherToolsResultCommonModelBuilder::tool_has_been_called)
    pub fn build(self) -> Result<ConversationHistoryTranscriptOtherToolsResultCommonModel, BuildError> {
        Ok(ConversationHistoryTranscriptOtherToolsResultCommonModel {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            result_value: self.result_value.ok_or_else(|| BuildError::missing_field("result_value"))?,
            is_error: self.is_error.ok_or_else(|| BuildError::missing_field("is_error"))?,
            is_blocked: self.is_blocked,
            tool_has_been_called: self.tool_has_been_called.ok_or_else(|| BuildError::missing_field("tool_has_been_called"))?,
            tool_latency_secs: self.tool_latency_secs,
            error_type: self.error_type,
            raw_error_message: self.raw_error_message,
            dynamic_variable_updates: self.dynamic_variable_updates,
            r#type: self.r#type,
        })
    }
}
