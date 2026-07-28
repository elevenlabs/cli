pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationSummaryResponseModel {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub start_time_unix_secs: i64,
    #[serde(default)]
    pub call_duration_secs: i64,
    #[serde(default)]
    pub message_count: i64,
    pub status: ConversationSummaryResponseModelStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
    pub call_successful: EvaluationSuccessResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub call_success_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_summary_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TelephonyDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub rating: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis: Option<ConversationSentimentAnalysis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_results: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria_results: Option<HashMap<String, Option<EvaluationCriteriaSummaryResult>>>,
    /// Conversation tag ids assigned to this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl ConversationSummaryResponseModel {
    pub fn builder() -> ConversationSummaryResponseModelBuilder {
        <ConversationSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSummaryResponseModelBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
    version_id: Option<String>,
    agent_name: Option<String>,
    conversation_id: Option<String>,
    start_time_unix_secs: Option<i64>,
    call_duration_secs: Option<i64>,
    message_count: Option<i64>,
    status: Option<ConversationSummaryResponseModelStatus>,
    termination_reason: Option<String>,
    call_successful: Option<EvaluationSuccessResult>,
    call_success_score: Option<f64>,
    transcript_summary: Option<String>,
    call_summary_title: Option<String>,
    main_language: Option<String>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    tool_names: Option<Vec<String>>,
    direction: Option<TelephonyDirection>,
    rating: Option<f64>,
    sentiment_analysis: Option<ConversationSentimentAnalysis>,
    data_collection_results: Option<HashMap<String, serde_json::Value>>,
    evaluation_criteria_results: Option<HashMap<String, Option<EvaluationCriteriaSummaryResult>>>,
    tag_ids: Option<Vec<String>>,
}

impl ConversationSummaryResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn start_time_unix_secs(mut self, value: i64) -> Self {
        self.start_time_unix_secs = Some(value);
        self
    }

    pub fn call_duration_secs(mut self, value: i64) -> Self {
        self.call_duration_secs = Some(value);
        self
    }

    pub fn message_count(mut self, value: i64) -> Self {
        self.message_count = Some(value);
        self
    }

    pub fn status(mut self, value: ConversationSummaryResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn termination_reason(mut self, value: impl Into<String>) -> Self {
        self.termination_reason = Some(value.into());
        self
    }

    pub fn call_successful(mut self, value: EvaluationSuccessResult) -> Self {
        self.call_successful = Some(value);
        self
    }

    pub fn call_success_score(mut self, value: f64) -> Self {
        self.call_success_score = Some(value);
        self
    }

    pub fn transcript_summary(mut self, value: impl Into<String>) -> Self {
        self.transcript_summary = Some(value.into());
        self
    }

    pub fn call_summary_title(mut self, value: impl Into<String>) -> Self {
        self.call_summary_title = Some(value.into());
        self
    }

    pub fn main_language(mut self, value: impl Into<String>) -> Self {
        self.main_language = Some(value.into());
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn tool_names(mut self, value: Vec<String>) -> Self {
        self.tool_names = Some(value);
        self
    }

    pub fn direction(mut self, value: TelephonyDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn rating(mut self, value: f64) -> Self {
        self.rating = Some(value);
        self
    }

    pub fn sentiment_analysis(mut self, value: ConversationSentimentAnalysis) -> Self {
        self.sentiment_analysis = Some(value);
        self
    }

    pub fn data_collection_results(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data_collection_results = Some(value);
        self
    }

    pub fn evaluation_criteria_results(mut self, value: HashMap<String, Option<EvaluationCriteriaSummaryResult>>) -> Self {
        self.evaluation_criteria_results = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](ConversationSummaryResponseModelBuilder::agent_id)
    /// - [`conversation_id`](ConversationSummaryResponseModelBuilder::conversation_id)
    /// - [`start_time_unix_secs`](ConversationSummaryResponseModelBuilder::start_time_unix_secs)
    /// - [`call_duration_secs`](ConversationSummaryResponseModelBuilder::call_duration_secs)
    /// - [`message_count`](ConversationSummaryResponseModelBuilder::message_count)
    /// - [`status`](ConversationSummaryResponseModelBuilder::status)
    /// - [`call_successful`](ConversationSummaryResponseModelBuilder::call_successful)
    pub fn build(self) -> Result<ConversationSummaryResponseModel, BuildError> {
        Ok(ConversationSummaryResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id,
            version_id: self.version_id,
            agent_name: self.agent_name,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            start_time_unix_secs: self.start_time_unix_secs.ok_or_else(|| BuildError::missing_field("start_time_unix_secs"))?,
            call_duration_secs: self.call_duration_secs.ok_or_else(|| BuildError::missing_field("call_duration_secs"))?,
            message_count: self.message_count.ok_or_else(|| BuildError::missing_field("message_count"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            termination_reason: self.termination_reason,
            call_successful: self.call_successful.ok_or_else(|| BuildError::missing_field("call_successful"))?,
            call_success_score: self.call_success_score,
            transcript_summary: self.transcript_summary,
            call_summary_title: self.call_summary_title,
            main_language: self.main_language,
            conversation_initiation_source: self.conversation_initiation_source,
            tool_names: self.tool_names,
            direction: self.direction,
            rating: self.rating,
            sentiment_analysis: self.sentiment_analysis,
            data_collection_results: self.data_collection_results,
            evaluation_criteria_results: self.evaluation_criteria_results,
            tag_ids: self.tag_ids,
        })
    }
}
