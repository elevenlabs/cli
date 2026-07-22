pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallCommonModelInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolType>,
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub tool_name: String,
    #[serde(default)]
    pub params_as_json: String,
    #[serde(default)]
    pub tool_has_been_called: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_details: Option<ConversationHistoryTranscriptToolCallCommonModelInputToolDetails>,
}

impl ConversationHistoryTranscriptToolCallCommonModelInput {
    pub fn builder() -> ConversationHistoryTranscriptToolCallCommonModelInputBuilder {
        <ConversationHistoryTranscriptToolCallCommonModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallCommonModelInputBuilder {
    r#type: Option<ToolType>,
    request_id: Option<String>,
    tool_name: Option<String>,
    params_as_json: Option<String>,
    tool_has_been_called: Option<bool>,
    tool_details: Option<ConversationHistoryTranscriptToolCallCommonModelInputToolDetails>,
}

impl ConversationHistoryTranscriptToolCallCommonModelInputBuilder {
    pub fn r#type(mut self, value: ToolType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn params_as_json(mut self, value: impl Into<String>) -> Self {
        self.params_as_json = Some(value.into());
        self
    }

    pub fn tool_has_been_called(mut self, value: bool) -> Self {
        self.tool_has_been_called = Some(value);
        self
    }

    pub fn tool_details(mut self, value: ConversationHistoryTranscriptToolCallCommonModelInputToolDetails) -> Self {
        self.tool_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallCommonModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](ConversationHistoryTranscriptToolCallCommonModelInputBuilder::request_id)
    /// - [`tool_name`](ConversationHistoryTranscriptToolCallCommonModelInputBuilder::tool_name)
    /// - [`params_as_json`](ConversationHistoryTranscriptToolCallCommonModelInputBuilder::params_as_json)
    /// - [`tool_has_been_called`](ConversationHistoryTranscriptToolCallCommonModelInputBuilder::tool_has_been_called)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallCommonModelInput, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallCommonModelInput {
            r#type: self.r#type,
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            params_as_json: self.params_as_json.ok_or_else(|| BuildError::missing_field("params_as_json"))?,
            tool_has_been_called: self.tool_has_been_called.ok_or_else(|| BuildError::missing_field("tool_has_been_called"))?,
            tool_details: self.tool_details,
        })
    }
}
