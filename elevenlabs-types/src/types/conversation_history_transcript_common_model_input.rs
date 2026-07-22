pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationHistoryTranscriptCommonModelInput {
    pub role: ConversationHistoryTranscriptCommonModelInputRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_metadata: Option<AgentMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multivoice_message: Option<ConversationHistoryMultivoiceMessageModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ConversationHistoryTranscriptToolCallCommonModelInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_results: Option<Vec<ConversationHistoryTranscriptCommonModelInputToolResultsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<UserFeedback>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_override: Option<String>,
    #[serde(default)]
    pub time_in_call_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_turn_metrics: Option<ConversationTurnMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_retrieval_info: Option<RagRetrievalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_usage: Option<LlmUsageInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_as_backchannel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Vec<ConversationReasoningModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_medium: Option<ChatSourceMedium>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_event_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_static_kb_document_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identifier: Option<String>,
}

impl ConversationHistoryTranscriptCommonModelInput {
    pub fn builder() -> ConversationHistoryTranscriptCommonModelInputBuilder {
        <ConversationHistoryTranscriptCommonModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptCommonModelInputBuilder {
    role: Option<ConversationHistoryTranscriptCommonModelInputRole>,
    agent_metadata: Option<AgentMetadata>,
    message: Option<String>,
    multivoice_message: Option<ConversationHistoryMultivoiceMessageModel>,
    tool_calls: Option<Vec<ConversationHistoryTranscriptToolCallCommonModelInput>>,
    tool_results: Option<Vec<ConversationHistoryTranscriptCommonModelInputToolResultsItem>>,
    feedback: Option<UserFeedback>,
    llm_override: Option<String>,
    time_in_call_secs: Option<i64>,
    conversation_turn_metrics: Option<ConversationTurnMetrics>,
    rag_retrieval_info: Option<RagRetrievalInfo>,
    llm_usage: Option<LlmUsageInput>,
    interrupted: Option<bool>,
    ignored_as_backchannel: Option<bool>,
    original_message: Option<String>,
    reasoning: Option<Vec<ConversationReasoningModel>>,
    source_medium: Option<ChatSourceMedium>,
    source_event_id: Option<i64>,
    used_static_kb_document_ids: Option<Vec<String>>,
    user_identifier: Option<String>,
}

impl ConversationHistoryTranscriptCommonModelInputBuilder {
    pub fn role(mut self, value: ConversationHistoryTranscriptCommonModelInputRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn agent_metadata(mut self, value: AgentMetadata) -> Self {
        self.agent_metadata = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn multivoice_message(mut self, value: ConversationHistoryMultivoiceMessageModel) -> Self {
        self.multivoice_message = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<ConversationHistoryTranscriptToolCallCommonModelInput>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn tool_results(mut self, value: Vec<ConversationHistoryTranscriptCommonModelInputToolResultsItem>) -> Self {
        self.tool_results = Some(value);
        self
    }

    pub fn feedback(mut self, value: UserFeedback) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn llm_override(mut self, value: impl Into<String>) -> Self {
        self.llm_override = Some(value.into());
        self
    }

    pub fn time_in_call_secs(mut self, value: i64) -> Self {
        self.time_in_call_secs = Some(value);
        self
    }

    pub fn conversation_turn_metrics(mut self, value: ConversationTurnMetrics) -> Self {
        self.conversation_turn_metrics = Some(value);
        self
    }

    pub fn rag_retrieval_info(mut self, value: RagRetrievalInfo) -> Self {
        self.rag_retrieval_info = Some(value);
        self
    }

    pub fn llm_usage(mut self, value: LlmUsageInput) -> Self {
        self.llm_usage = Some(value);
        self
    }

    pub fn interrupted(mut self, value: bool) -> Self {
        self.interrupted = Some(value);
        self
    }

    pub fn ignored_as_backchannel(mut self, value: bool) -> Self {
        self.ignored_as_backchannel = Some(value);
        self
    }

    pub fn original_message(mut self, value: impl Into<String>) -> Self {
        self.original_message = Some(value.into());
        self
    }

    pub fn reasoning(mut self, value: Vec<ConversationReasoningModel>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn source_medium(mut self, value: ChatSourceMedium) -> Self {
        self.source_medium = Some(value);
        self
    }

    pub fn source_event_id(mut self, value: i64) -> Self {
        self.source_event_id = Some(value);
        self
    }

    pub fn used_static_kb_document_ids(mut self, value: Vec<String>) -> Self {
        self.used_static_kb_document_ids = Some(value);
        self
    }

    pub fn user_identifier(mut self, value: impl Into<String>) -> Self {
        self.user_identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptCommonModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](ConversationHistoryTranscriptCommonModelInputBuilder::role)
    /// - [`time_in_call_secs`](ConversationHistoryTranscriptCommonModelInputBuilder::time_in_call_secs)
    pub fn build(self) -> Result<ConversationHistoryTranscriptCommonModelInput, BuildError> {
        Ok(ConversationHistoryTranscriptCommonModelInput {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            agent_metadata: self.agent_metadata,
            message: self.message,
            multivoice_message: self.multivoice_message,
            tool_calls: self.tool_calls,
            tool_results: self.tool_results,
            feedback: self.feedback,
            llm_override: self.llm_override,
            time_in_call_secs: self.time_in_call_secs.ok_or_else(|| BuildError::missing_field("time_in_call_secs"))?,
            conversation_turn_metrics: self.conversation_turn_metrics,
            rag_retrieval_info: self.rag_retrieval_info,
            llm_usage: self.llm_usage,
            interrupted: self.interrupted,
            ignored_as_backchannel: self.ignored_as_backchannel,
            original_message: self.original_message,
            reasoning: self.reasoning,
            source_medium: self.source_medium,
            source_event_id: self.source_event_id,
            used_static_kb_document_ids: self.used_static_kb_document_ids,
            user_identifier: self.user_identifier,
        })
    }
}
