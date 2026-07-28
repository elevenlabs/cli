use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MessagesClient {
    pub http_client: HttpClient,
}

impl MessagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Search through conversation transcript messages by full-text and fuzzy search
    ///
    /// # Arguments
    ///
    /// * `text_query` - The search query text for full-text and fuzzy matching
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `visited_agent_ids` - Filter conversations where any of these agents participated. Can not exceed 50 values.
    /// * `visited_agent_branch_ids` - Filter conversations where any of these agent branches participated. Can not exceed 50 values.
    /// * `call_successful` - The result of the success evaluation
    /// * `call_start_before_unix` - Unix timestamp (in seconds) to filter conversations up to this start date.
    /// * `call_start_after_unix` - Unix timestamp (in seconds) to filter conversations after to this start date.
    /// * `call_duration_min_secs` - Minimum call duration in seconds.
    /// * `call_duration_max_secs` - Maximum call duration in seconds.
    /// * `rating_max` - Maximum overall rating (1-5).
    /// * `rating_min` - Minimum overall rating (1-5).
    /// * `has_feedback_comment` - Filter conversations with user feedback comments.
    /// * `user_id` - Filter conversations by the user ID who initiated them.
    /// * `evaluation_params` - Evaluation filters. Repeat param. Format: criteria_id:result. Example: eval=value_framing:success
    /// * `data_collection_params` - Data collection filters. Repeat param. Format: id:op:value where op is one of eq|neq|gt|gte|lt|lte|in|exists|missing. For in, pipe-delimit values.
    /// * `tool_names` - Filter conversations by tool names used during the call.
    /// * `tool_names_successful` - Filter conversations by tool names that had successful calls.
    /// * `tool_names_errored` - Filter conversations by tool names that had errored calls.
    /// * `main_languages` - Filter conversations by detected main language (language code).
    /// * `exclude_statuses` - Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views.
    /// * `termination_reasons` - Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several.
    /// * `page_size` - Number of results per page. Max 50.
    /// * `summary_mode` - Whether to include transcript summaries in the response.
    /// * `conversation_product_type` - Restrict results to a single conversation product surface.
    /// * `branch_id` - Filter conversations by branch ID.
    /// * `topic_ids` - Filter conversations by topic IDs assigned during topic discovery.
    /// * `sort_by` - Sort order for search results. 'search_score' sorts by search score, 'created_at' sorts by conversation start time.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .conversations
    ///         .messages
    ///         .text_search(
    ///             &TextSearchQueryRequest {
    ///                 text_query: "refund policy".to_string(),
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 visited_agent_ids: vec![Some("visited_agent_ids".to_string())],
    ///                 visited_agent_branch_ids: vec![Some("visited_agent_branch_ids".to_string())],
    ///                 call_successful: Some(EvaluationSuccessResult::Success),
    ///                 call_start_before_unix: Some(1),
    ///                 call_start_after_unix: Some(1),
    ///                 call_duration_min_secs: Some(1),
    ///                 call_duration_max_secs: Some(1),
    ///                 rating_max: Some(1),
    ///                 rating_min: Some(1),
    ///                 has_feedback_comment: Some(true),
    ///                 user_id: Some("user_id".to_string()),
    ///                 evaluation_params: vec![Some("evaluation_params".to_string())],
    ///                 data_collection_params: vec![Some("data_collection_params".to_string())],
    ///                 tool_names: vec![Some("tool_names".to_string())],
    ///                 tool_names_successful: vec![Some("tool_names_successful".to_string())],
    ///                 tool_names_errored: vec![Some("tool_names_errored".to_string())],
    ///                 main_languages: vec![Some("main_languages".to_string())],
    ///                 exclude_statuses: vec![Some(
    ///                     MessagesTextSearchRequestExcludeStatusesItem::Initiated,
    ///                 )],
    ///                 termination_reasons: vec![Some("termination_reasons".to_string())],
    ///                 page_size: Some(1),
    ///                 summary_mode: Some(MessagesTextSearchRequestSummaryMode::Exclude),
    ///                 conversation_initiation_source: Some(ConversationInitiationSource::Unknown),
    ///                 text_only: Some(true),
    ///                 conversation_product_type: Some(ConversationProduct::Agents),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 topic_ids: vec![Some("topic_ids".to_string())],
    ///                 sort_by: Some(MessageSearchSortBy::SearchScore),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn text_search(
        &self,
        request: &TextSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MessagesSearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversations/messages/text-search",
                None,
                QueryBuilder::new()
                    .string("text_query", request.text_query.clone())
                    .string("agent_id", request.agent_id.clone())
                    .string_array("visited_agent_ids", request.visited_agent_ids.clone())
                    .string_array(
                        "visited_agent_branch_ids",
                        request.visited_agent_branch_ids.clone(),
                    )
                    .serialize("call_successful", request.call_successful.clone())
                    .int(
                        "call_start_before_unix",
                        request.call_start_before_unix.clone(),
                    )
                    .int(
                        "call_start_after_unix",
                        request.call_start_after_unix.clone(),
                    )
                    .int(
                        "call_duration_min_secs",
                        request.call_duration_min_secs.clone(),
                    )
                    .int(
                        "call_duration_max_secs",
                        request.call_duration_max_secs.clone(),
                    )
                    .int("rating_max", request.rating_max.clone())
                    .int("rating_min", request.rating_min.clone())
                    .bool("has_feedback_comment", request.has_feedback_comment.clone())
                    .string("user_id", request.user_id.clone())
                    .string_array("evaluation_params", request.evaluation_params.clone())
                    .string_array(
                        "data_collection_params",
                        request.data_collection_params.clone(),
                    )
                    .string_array("tool_names", request.tool_names.clone())
                    .string_array(
                        "tool_names_successful",
                        request.tool_names_successful.clone(),
                    )
                    .string_array("tool_names_errored", request.tool_names_errored.clone())
                    .string_array("main_languages", request.main_languages.clone())
                    .serialize_array("exclude_statuses", request.exclude_statuses.clone())
                    .string_array("termination_reasons", request.termination_reasons.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("summary_mode", request.summary_mode.clone())
                    .serialize(
                        "conversation_initiation_source",
                        request.conversation_initiation_source.clone(),
                    )
                    .bool("text_only", request.text_only.clone())
                    .serialize(
                        "conversation_product_type",
                        request.conversation_product_type.clone(),
                    )
                    .string("branch_id", request.branch_id.clone())
                    .string_array("topic_ids", request.topic_ids.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Search conversation transcripts by semantic similarity to surface relevant messages based on meaning and intent, rather than exact keyword matches
    ///
    /// # Arguments
    ///
    /// * `text_query` - The search query text for semantic similarity matching
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `page_size` - Number of results per page. Max 50.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .agents
    ///         .conversations
    ///         .messages
    ///         .search(
    ///             &AgentsConversationsMessagesSearchQueryRequest {
    ///                 text_query: "Customer asking to cancel and get money back".to_string(),
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &AgentsConversationsMessagesSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MessagesSearchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversations/messages/smart-search",
                None,
                QueryBuilder::new()
                    .string("text_query", request.text_query.clone())
                    .string("agent_id", request.agent_id.clone())
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
