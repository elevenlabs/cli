pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetConversationSummaryResponseModel {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub agent_id: String,
    pub status: GetConversationSummaryResponseModelStatus,
    /// Short generated title for the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_summary_title: Option<String>,
    /// Generated natural-language summary of the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_successful: Option<EvaluationSuccessResult>,
    /// Number of plain chat message turns in the conversation.
    #[serde(default)]
    pub message_count: i64,
    /// The plain chat messages (role and text only). Included only when message_count does not exceed the requested max_messages; otherwise null and messages_omitted is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ConversationSummaryMessageModel>>,
    /// True when the chat messages were omitted because the conversation was too long. Fetch the full transcript for the messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_omitted: Option<bool>,
    /// Guidance telling the agent how to get the full transcript.
    #[serde(default)]
    pub note: String,
}

impl GetConversationSummaryResponseModel {
    pub fn builder() -> GetConversationSummaryResponseModelBuilder {
        <GetConversationSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationSummaryResponseModelBuilder {
    conversation_id: Option<String>,
    agent_id: Option<String>,
    status: Option<GetConversationSummaryResponseModelStatus>,
    call_summary_title: Option<String>,
    transcript_summary: Option<String>,
    call_successful: Option<EvaluationSuccessResult>,
    message_count: Option<i64>,
    messages: Option<Vec<ConversationSummaryMessageModel>>,
    messages_omitted: Option<bool>,
    note: Option<String>,
}

impl GetConversationSummaryResponseModelBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetConversationSummaryResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn call_summary_title(mut self, value: impl Into<String>) -> Self {
        self.call_summary_title = Some(value.into());
        self
    }

    pub fn transcript_summary(mut self, value: impl Into<String>) -> Self {
        self.transcript_summary = Some(value.into());
        self
    }

    pub fn call_successful(mut self, value: EvaluationSuccessResult) -> Self {
        self.call_successful = Some(value);
        self
    }

    pub fn message_count(mut self, value: i64) -> Self {
        self.message_count = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ConversationSummaryMessageModel>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn messages_omitted(mut self, value: bool) -> Self {
        self.messages_omitted = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetConversationSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](GetConversationSummaryResponseModelBuilder::conversation_id)
    /// - [`agent_id`](GetConversationSummaryResponseModelBuilder::agent_id)
    /// - [`status`](GetConversationSummaryResponseModelBuilder::status)
    /// - [`message_count`](GetConversationSummaryResponseModelBuilder::message_count)
    /// - [`note`](GetConversationSummaryResponseModelBuilder::note)
    pub fn build(self) -> Result<GetConversationSummaryResponseModel, BuildError> {
        Ok(GetConversationSummaryResponseModel {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            call_summary_title: self.call_summary_title,
            transcript_summary: self.transcript_summary,
            call_successful: self.call_successful,
            message_count: self.message_count.ok_or_else(|| BuildError::missing_field("message_count"))?,
            messages: self.messages,
            messages_omitted: self.messages_omitted,
            note: self.note.ok_or_else(|| BuildError::missing_field("note"))?,
        })
    }
}
