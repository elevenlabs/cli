pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsListQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Filter conversations where any of these agents participated. Can not exceed 50 values.
    #[serde(default)]
    pub visited_agent_ids: Vec<Option<String>>,
    /// Filter conversations where any of these agent branches participated. Can not exceed 50 values.
    #[serde(default)]
    pub visited_agent_branch_ids: Vec<Option<String>>,
    /// The result of the success evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_successful: Option<EvaluationSuccessResult>,
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
    /// Data collection field IDs to include in each conversation summary. Repeat param. When omitted, data_collection_results is not returned.
    #[serde(default)]
    pub data_collection_ids: Vec<Option<String>>,
    /// Evaluation criteria IDs to include in each conversation summary. Repeat param. When omitted, evaluation_criteria_results is not returned.
    #[serde(default)]
    pub evaluation_criteria_ids: Vec<Option<String>>,
    /// Filter conversations by tool names used during the call.
    #[serde(default)]
    pub tool_names: Vec<Option<String>>,
    /// Filter conversations by tool names that had successful calls.
    #[serde(default)]
    pub tool_names_successful: Vec<Option<String>>,
    /// Filter conversations by tool names that had errored calls.
    #[serde(default)]
    pub tool_names_errored: Vec<Option<String>>,
    /// Filter conversations by detected main language (language code).
    #[serde(default)]
    pub main_languages: Vec<Option<String>>,
    /// How many conversations to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Whether to include transcript summaries in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_mode: Option<ConversationsListRequestSummaryMode>,
    /// Full-text or fuzzy search over transcript messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
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
    /// Filter conversations by topic IDs assigned during topic discovery.
    #[serde(default)]
    pub topic_ids: Vec<Option<String>>,
    /// Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views.
    #[serde(default)]
    pub exclude_statuses: Vec<Option<ConversationsListRequestExcludeStatusesItem>>,
    /// Filter conversations by conversation tag IDs assigned via the conversation-tags endpoints.
    #[serde(default)]
    pub tag_ids: Vec<Option<String>>,
    /// Filter conversations to only those that entered the given node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_node_entered_id: Option<String>,
    /// Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several.
    #[serde(default)]
    pub termination_reasons: Vec<Option<String>>,
}

impl AgentsConversationsListQueryRequest {
    pub fn builder() -> AgentsConversationsListQueryRequestBuilder {
        <AgentsConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsListQueryRequestBuilder {
    cursor: Option<String>,
    agent_id: Option<String>,
    visited_agent_ids: Option<Vec<Option<String>>>,
    visited_agent_branch_ids: Option<Vec<Option<String>>>,
    call_successful: Option<EvaluationSuccessResult>,
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
    data_collection_ids: Option<Vec<Option<String>>>,
    evaluation_criteria_ids: Option<Vec<Option<String>>>,
    tool_names: Option<Vec<Option<String>>>,
    tool_names_successful: Option<Vec<Option<String>>>,
    tool_names_errored: Option<Vec<Option<String>>>,
    main_languages: Option<Vec<Option<String>>>,
    page_size: Option<i64>,
    summary_mode: Option<ConversationsListRequestSummaryMode>,
    search: Option<String>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    text_only: Option<bool>,
    conversation_product_type: Option<ConversationProduct>,
    branch_id: Option<String>,
    topic_ids: Option<Vec<Option<String>>>,
    exclude_statuses: Option<Vec<Option<ConversationsListRequestExcludeStatusesItem>>>,
    tag_ids: Option<Vec<Option<String>>>,
    workflow_node_entered_id: Option<String>,
    termination_reasons: Option<Vec<Option<String>>>,
}

impl AgentsConversationsListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
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

    pub fn call_successful(mut self, value: EvaluationSuccessResult) -> Self {
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

    pub fn data_collection_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.data_collection_ids = Some(value);
        self
    }

    pub fn evaluation_criteria_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.evaluation_criteria_ids = Some(value);
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

    pub fn main_languages(mut self, value: Vec<Option<String>>) -> Self {
        self.main_languages = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn summary_mode(mut self, value: ConversationsListRequestSummaryMode) -> Self {
        self.summary_mode = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
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

    pub fn topic_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.topic_ids = Some(value);
        self
    }

    pub fn exclude_statuses(mut self, value: Vec<Option<ConversationsListRequestExcludeStatusesItem>>) -> Self {
        self.exclude_statuses = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn workflow_node_entered_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_node_entered_id = Some(value.into());
        self
    }

    pub fn termination_reasons(mut self, value: Vec<Option<String>>) -> Self {
        self.termination_reasons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`visited_agent_ids`](AgentsConversationsListQueryRequestBuilder::visited_agent_ids)
    /// - [`visited_agent_branch_ids`](AgentsConversationsListQueryRequestBuilder::visited_agent_branch_ids)
    /// - [`evaluation_params`](AgentsConversationsListQueryRequestBuilder::evaluation_params)
    /// - [`data_collection_params`](AgentsConversationsListQueryRequestBuilder::data_collection_params)
    /// - [`data_collection_ids`](AgentsConversationsListQueryRequestBuilder::data_collection_ids)
    /// - [`evaluation_criteria_ids`](AgentsConversationsListQueryRequestBuilder::evaluation_criteria_ids)
    /// - [`tool_names`](AgentsConversationsListQueryRequestBuilder::tool_names)
    /// - [`tool_names_successful`](AgentsConversationsListQueryRequestBuilder::tool_names_successful)
    /// - [`tool_names_errored`](AgentsConversationsListQueryRequestBuilder::tool_names_errored)
    /// - [`main_languages`](AgentsConversationsListQueryRequestBuilder::main_languages)
    /// - [`topic_ids`](AgentsConversationsListQueryRequestBuilder::topic_ids)
    /// - [`exclude_statuses`](AgentsConversationsListQueryRequestBuilder::exclude_statuses)
    /// - [`tag_ids`](AgentsConversationsListQueryRequestBuilder::tag_ids)
    /// - [`termination_reasons`](AgentsConversationsListQueryRequestBuilder::termination_reasons)
    pub fn build(self) -> Result<AgentsConversationsListQueryRequest, BuildError> {
        Ok(AgentsConversationsListQueryRequest {
            cursor: self.cursor,
            agent_id: self.agent_id,
            visited_agent_ids: self.visited_agent_ids.ok_or_else(|| BuildError::missing_field("visited_agent_ids"))?,
            visited_agent_branch_ids: self.visited_agent_branch_ids.ok_or_else(|| BuildError::missing_field("visited_agent_branch_ids"))?,
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
            data_collection_ids: self.data_collection_ids.ok_or_else(|| BuildError::missing_field("data_collection_ids"))?,
            evaluation_criteria_ids: self.evaluation_criteria_ids.ok_or_else(|| BuildError::missing_field("evaluation_criteria_ids"))?,
            tool_names: self.tool_names.ok_or_else(|| BuildError::missing_field("tool_names"))?,
            tool_names_successful: self.tool_names_successful.ok_or_else(|| BuildError::missing_field("tool_names_successful"))?,
            tool_names_errored: self.tool_names_errored.ok_or_else(|| BuildError::missing_field("tool_names_errored"))?,
            main_languages: self.main_languages.ok_or_else(|| BuildError::missing_field("main_languages"))?,
            page_size: self.page_size,
            summary_mode: self.summary_mode,
            search: self.search,
            conversation_initiation_source: self.conversation_initiation_source,
            text_only: self.text_only,
            conversation_product_type: self.conversation_product_type,
            branch_id: self.branch_id,
            topic_ids: self.topic_ids.ok_or_else(|| BuildError::missing_field("topic_ids"))?,
            exclude_statuses: self.exclude_statuses.ok_or_else(|| BuildError::missing_field("exclude_statuses"))?,
            tag_ids: self.tag_ids.ok_or_else(|| BuildError::missing_field("tag_ids"))?,
            workflow_node_entered_id: self.workflow_node_entered_id,
            termination_reasons: self.termination_reasons.ok_or_else(|| BuildError::missing_field("termination_reasons"))?,
        })
    }
}

