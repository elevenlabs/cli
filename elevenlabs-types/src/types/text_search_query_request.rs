pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for text_search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TextSearchQueryRequest {
    /// The search query text for full-text and fuzzy matching
    #[serde(default)]
    pub text_query: String,
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter conversations where any of these agents participated. Can not exceed 50 values.
    #[serde(default)]
    pub visited_agent_ids: Vec<Option<String>>,
    /// Filter conversations where any of these agent branches participated. Can not exceed 50 values.
    #[serde(default)]
    pub visited_agent_branch_ids: Vec<Option<String>>,
    /// Filter conversations where any of these procedures were triggered. Can not exceed 50 values.
    #[serde(default)]
    pub triggered_procedure_ids: Vec<Option<String>>,
    /// The result of the success evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_successful: Option<EvaluationResultFilter>,
    /// Unix timestamp (in seconds) to filter conversations up to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_before_unix: Option<i64>,
    /// Unix timestamp (in seconds) to filter conversations after to this start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_start_after_unix: Option<i64>,
    /// Minimum call duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_duration_min_secs: Option<i64>,
    /// Maximum call duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_duration_max_secs: Option<i64>,
    /// Maximum overall rating (1-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_max: Option<i64>,
    /// Minimum overall rating (1-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_min: Option<i64>,
    /// Filter conversations with user feedback comments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_feedback_comment: Option<bool>,
    /// Filter conversations by the user ID who initiated them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Evaluation filters. Repeat param. Format: criteria_id:result. Example: eval=value_framing:success
    #[serde(default)]
    pub evaluation_params: Vec<Option<String>>,
    /// Data collection filters. Repeat param. Format: id:op:value where op is one of eq|neq|gt|gte|lt|lte|in|exists|missing. For in, pipe-delimit values.
    #[serde(default)]
    pub data_collection_params: Vec<Option<String>>,
    /// Filter conversations by tool names used during the call.
    #[serde(default)]
    pub tool_names: Vec<Option<String>>,
    /// Filter conversations by tool names that had successful calls.
    #[serde(default)]
    pub tool_names_successful: Vec<Option<String>>,
    /// Filter conversations by tool names that had errored calls.
    #[serde(default)]
    pub tool_names_errored: Vec<Option<String>>,
    /// Also match tool calls that never ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_invalid_tool_calls: Option<bool>,
    /// Filter conversations by detected main language (language code).
    #[serde(default)]
    pub main_languages: Vec<Option<String>>,
    /// Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views.
    #[serde(default)]
    pub exclude_statuses: Vec<Option<MessagesTextSearchRequestExcludeStatusesItem>>,
    /// Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several.
    #[serde(default)]
    pub termination_reasons: Vec<Option<String>>,
    /// Number of results per page. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Whether to include transcript summaries in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_mode: Option<MessagesTextSearchRequestSummaryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
    /// Restrict results to a single conversation product surface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_product_type: Option<ConversationProduct>,
    /// Filter conversations by branch ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Filter conversations by version ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Filter conversations by topic IDs assigned during topic discovery.
    #[serde(default)]
    pub topic_ids: Vec<Option<String>>,
    /// Sort order for search results. 'search_score' sorts by search score, 'created_at' sorts by conversation start time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<MessageSearchSortBy>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl TextSearchQueryRequest {
    pub fn builder() -> TextSearchQueryRequestBuilder {
        <TextSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextSearchQueryRequestBuilder {
    text_query: Option<String>,
    agent_id: Option<String>,
    visited_agent_ids: Option<Vec<Option<String>>>,
    visited_agent_branch_ids: Option<Vec<Option<String>>>,
    triggered_procedure_ids: Option<Vec<Option<String>>>,
    call_successful: Option<EvaluationResultFilter>,
    call_start_before_unix: Option<i64>,
    call_start_after_unix: Option<i64>,
    call_duration_min_secs: Option<i64>,
    call_duration_max_secs: Option<i64>,
    rating_max: Option<i64>,
    rating_min: Option<i64>,
    has_feedback_comment: Option<bool>,
    user_id: Option<String>,
    evaluation_params: Option<Vec<Option<String>>>,
    data_collection_params: Option<Vec<Option<String>>>,
    tool_names: Option<Vec<Option<String>>>,
    tool_names_successful: Option<Vec<Option<String>>>,
    tool_names_errored: Option<Vec<Option<String>>>,
    include_invalid_tool_calls: Option<bool>,
    main_languages: Option<Vec<Option<String>>>,
    exclude_statuses: Option<Vec<Option<MessagesTextSearchRequestExcludeStatusesItem>>>,
    termination_reasons: Option<Vec<Option<String>>>,
    page_size: Option<i64>,
    summary_mode: Option<MessagesTextSearchRequestSummaryMode>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    text_only: Option<bool>,
    conversation_product_type: Option<ConversationProduct>,
    branch_id: Option<String>,
    version_id: Option<String>,
    topic_ids: Option<Vec<Option<String>>>,
    sort_by: Option<MessageSearchSortBy>,
    cursor: Option<String>,
}

impl TextSearchQueryRequestBuilder {
    pub fn text_query(mut self, value: impl Into<String>) -> Self {
        self.text_query = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn visited_agent_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.visited_agent_ids = Some(value);
        self
    }

    pub fn visited_agent_branch_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.visited_agent_branch_ids = Some(value);
        self
    }

    pub fn triggered_procedure_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.triggered_procedure_ids = Some(value);
        self
    }

    pub fn call_successful(mut self, value: EvaluationResultFilter) -> Self {
        self.call_successful = Some(value);
        self
    }

    pub fn call_start_before_unix(mut self, value: i64) -> Self {
        self.call_start_before_unix = Some(value);
        self
    }

    pub fn call_start_after_unix(mut self, value: i64) -> Self {
        self.call_start_after_unix = Some(value);
        self
    }

    pub fn call_duration_min_secs(mut self, value: i64) -> Self {
        self.call_duration_min_secs = Some(value);
        self
    }

    pub fn call_duration_max_secs(mut self, value: i64) -> Self {
        self.call_duration_max_secs = Some(value);
        self
    }

    pub fn rating_max(mut self, value: i64) -> Self {
        self.rating_max = Some(value);
        self
    }

    pub fn rating_min(mut self, value: i64) -> Self {
        self.rating_min = Some(value);
        self
    }

    pub fn has_feedback_comment(mut self, value: bool) -> Self {
        self.has_feedback_comment = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn evaluation_params(mut self, value: Vec<Option<String>>) -> Self {
        self.evaluation_params = Some(value);
        self
    }

    pub fn data_collection_params(mut self, value: Vec<Option<String>>) -> Self {
        self.data_collection_params = Some(value);
        self
    }

    pub fn tool_names(mut self, value: Vec<Option<String>>) -> Self {
        self.tool_names = Some(value);
        self
    }

    pub fn tool_names_successful(mut self, value: Vec<Option<String>>) -> Self {
        self.tool_names_successful = Some(value);
        self
    }

    pub fn tool_names_errored(mut self, value: Vec<Option<String>>) -> Self {
        self.tool_names_errored = Some(value);
        self
    }

    pub fn include_invalid_tool_calls(mut self, value: bool) -> Self {
        self.include_invalid_tool_calls = Some(value);
        self
    }

    pub fn main_languages(mut self, value: Vec<Option<String>>) -> Self {
        self.main_languages = Some(value);
        self
    }

    pub fn exclude_statuses(mut self, value: Vec<Option<MessagesTextSearchRequestExcludeStatusesItem>>) -> Self {
        self.exclude_statuses = Some(value);
        self
    }

    pub fn termination_reasons(mut self, value: Vec<Option<String>>) -> Self {
        self.termination_reasons = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn summary_mode(mut self, value: MessagesTextSearchRequestSummaryMode) -> Self {
        self.summary_mode = Some(value);
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    pub fn conversation_product_type(mut self, value: ConversationProduct) -> Self {
        self.conversation_product_type = Some(value);
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

    pub fn topic_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.topic_ids = Some(value);
        self
    }

    pub fn sort_by(mut self, value: MessageSearchSortBy) -> Self {
        self.sort_by = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_query`](TextSearchQueryRequestBuilder::text_query)
    /// - [`visited_agent_ids`](TextSearchQueryRequestBuilder::visited_agent_ids)
    /// - [`visited_agent_branch_ids`](TextSearchQueryRequestBuilder::visited_agent_branch_ids)
    /// - [`triggered_procedure_ids`](TextSearchQueryRequestBuilder::triggered_procedure_ids)
    /// - [`evaluation_params`](TextSearchQueryRequestBuilder::evaluation_params)
    /// - [`data_collection_params`](TextSearchQueryRequestBuilder::data_collection_params)
    /// - [`tool_names`](TextSearchQueryRequestBuilder::tool_names)
    /// - [`tool_names_successful`](TextSearchQueryRequestBuilder::tool_names_successful)
    /// - [`tool_names_errored`](TextSearchQueryRequestBuilder::tool_names_errored)
    /// - [`main_languages`](TextSearchQueryRequestBuilder::main_languages)
    /// - [`exclude_statuses`](TextSearchQueryRequestBuilder::exclude_statuses)
    /// - [`termination_reasons`](TextSearchQueryRequestBuilder::termination_reasons)
    /// - [`topic_ids`](TextSearchQueryRequestBuilder::topic_ids)
    pub fn build(self) -> Result<TextSearchQueryRequest, BuildError> {
        Ok(TextSearchQueryRequest {
            text_query: self.text_query.ok_or_else(|| BuildError::missing_field("text_query"))?,
            agent_id: self.agent_id,
            visited_agent_ids: self.visited_agent_ids.ok_or_else(|| BuildError::missing_field("visited_agent_ids"))?,
            visited_agent_branch_ids: self.visited_agent_branch_ids.ok_or_else(|| BuildError::missing_field("visited_agent_branch_ids"))?,
            triggered_procedure_ids: self.triggered_procedure_ids.ok_or_else(|| BuildError::missing_field("triggered_procedure_ids"))?,
            call_successful: self.call_successful,
            call_start_before_unix: self.call_start_before_unix,
            call_start_after_unix: self.call_start_after_unix,
            call_duration_min_secs: self.call_duration_min_secs,
            call_duration_max_secs: self.call_duration_max_secs,
            rating_max: self.rating_max,
            rating_min: self.rating_min,
            has_feedback_comment: self.has_feedback_comment,
            user_id: self.user_id,
            evaluation_params: self.evaluation_params.ok_or_else(|| BuildError::missing_field("evaluation_params"))?,
            data_collection_params: self.data_collection_params.ok_or_else(|| BuildError::missing_field("data_collection_params"))?,
            tool_names: self.tool_names.ok_or_else(|| BuildError::missing_field("tool_names"))?,
            tool_names_successful: self.tool_names_successful.ok_or_else(|| BuildError::missing_field("tool_names_successful"))?,
            tool_names_errored: self.tool_names_errored.ok_or_else(|| BuildError::missing_field("tool_names_errored"))?,
            include_invalid_tool_calls: self.include_invalid_tool_calls,
            main_languages: self.main_languages.ok_or_else(|| BuildError::missing_field("main_languages"))?,
            exclude_statuses: self.exclude_statuses.ok_or_else(|| BuildError::missing_field("exclude_statuses"))?,
            termination_reasons: self.termination_reasons.ok_or_else(|| BuildError::missing_field("termination_reasons"))?,
            page_size: self.page_size,
            summary_mode: self.summary_mode,
            conversation_initiation_source: self.conversation_initiation_source,
            text_only: self.text_only,
            conversation_product_type: self.conversation_product_type,
            branch_id: self.branch_id,
            version_id: self.version_id,
            topic_ids: self.topic_ids.ok_or_else(|| BuildError::missing_field("topic_ids"))?,
            sort_by: self.sort_by,
            cursor: self.cursor,
        })
    }
}

