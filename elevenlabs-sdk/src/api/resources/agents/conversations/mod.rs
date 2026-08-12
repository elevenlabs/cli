use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod audio;
pub use audio::AudioClient;
pub mod feedback;
pub use feedback::FeedbackClient;
pub mod messages;
pub use messages::MessagesClient;
pub mod tags;
pub use tags::TagsClient;
pub mod files;
pub use files::FilesClient;
pub mod topics;
pub use topics::TopicsClient;
pub mod analysis;
pub use analysis::AnalysisClient;
pub struct ConversationsClient {
    pub http_client: HttpClient,
    pub audio: AudioClient,
    pub feedback: FeedbackClient,
    pub messages: MessagesClient,
    pub tags: TagsClient,
    pub files: FilesClient,
    pub topics: TopicsClient,
    pub analysis: AnalysisClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            audio: AudioClient::new(config.clone())?,
            feedback: FeedbackClient::new(config.clone())?,
            messages: MessagesClient::new(config.clone())?,
            tags: TagsClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            topics: TopicsClient::new(config.clone())?,
            analysis: AnalysisClient::new(config.clone())?,
        })
    }

    /// Get a signed url to start a conversation with an agent with an agent that requires authorization
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `include_conversation_id` - Whether to include a conversation_id with the response. If included, the conversation_signature cannot be used again.
    /// * `branch_id` - The ID of the branch to use
    /// * `environment` - The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'.
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
    ///         .get_signed_url(
    ///             &GetSignedURLQueryRequest {
    ///                 agent_id: "agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///                 include_conversation_id: Some(true),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 environment: Some("environment".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_signed_url(
        &self,
        request: &GetSignedUrlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationSignedUrlResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversation/get-signed-url",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .bool(
                        "include_conversation_id",
                        request.include_conversation_id.clone(),
                    )
                    .string("branch_id", request.branch_id.clone())
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get a WebRTC session token for real-time communication.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `participant_name` - Optional custom participant name. If not provided, user ID will be used
    /// * `branch_id` - The ID of the branch to use
    /// * `environment` - The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'.
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
    ///         .get_webrtc_token(
    ///             &GetWebrtcTokenQueryRequest {
    ///                 agent_id: "agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///                 participant_name: Some("participant_name".to_string()),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 environment: Some("environment".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_webrtc_token(
        &self,
        request: &GetWebrtcTokenQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TokenResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversation/token",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .string("participant_name", request.participant_name.clone())
                    .string("branch_id", request.branch_id.clone())
                    .string("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get all conversations of agents that user owns. With option to restrict to a specific agent.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
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
    /// * `data_collection_ids` - Data collection field IDs to include in each conversation summary. Repeat param. When omitted, data_collection_results is not returned.
    /// * `evaluation_criteria_ids` - Evaluation criteria IDs to include in each conversation summary. Repeat param. When omitted, evaluation_criteria_results is not returned.
    /// * `tool_names` - Filter conversations by tool names used during the call.
    /// * `tool_names_successful` - Filter conversations by tool names that had successful calls.
    /// * `tool_names_errored` - Filter conversations by tool names that had errored calls.
    /// * `main_languages` - Filter conversations by detected main language (language code).
    /// * `page_size` - How many conversations to return at maximum. Can not exceed 100, defaults to 30.
    /// * `summary_mode` - Whether to include transcript summaries in the response.
    /// * `search` - Full-text or fuzzy search over transcript messages
    /// * `conversation_product_type` - Restrict results to a single conversation product surface.
    /// * `branch_id` - Filter conversations by branch ID.
    /// * `version_id` - Filter conversations by version ID.
    /// * `parent_conversation_id` - Filter conversations by parent conversation ID for subagent conversations.
    /// * `topic_ids` - Filter conversations by topic IDs assigned during topic discovery.
    /// * `exclude_statuses` - Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views.
    /// * `tag_ids` - Filter conversations by conversation tag IDs assigned via the conversation-tags endpoints.
    /// * `workflow_node_entered_id` - Filter conversations to only those that entered the given node.
    /// * `termination_reasons` - Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several.
    /// * `guardrail_types` - Filter to conversations where a guardrail of any of these types triggered (metadata.triggered_guardrails.guardrail_type). Repeat param to match any of several.
    /// * `custom_guardrail_names` - Filter to conversations where a custom guardrail with any of these names triggered (metadata.triggered_guardrails.guardrail_name). Only custom guardrails carry a name. Repeat param to match any of several.
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
    ///         .list(
    ///             &AgentsConversationsListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
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
    ///                 data_collection_ids: vec![Some("data_collection_ids".to_string())],
    ///                 evaluation_criteria_ids: vec![Some("evaluation_criteria_ids".to_string())],
    ///                 tool_names: vec![Some("tool_names".to_string())],
    ///                 tool_names_successful: vec![Some("tool_names_successful".to_string())],
    ///                 tool_names_errored: vec![Some("tool_names_errored".to_string())],
    ///                 main_languages: vec![Some("main_languages".to_string())],
    ///                 page_size: Some(1),
    ///                 summary_mode: Some(ConversationsListRequestSummaryMode::Exclude),
    ///                 search: Some("search".to_string()),
    ///                 conversation_initiation_source: Some(ConversationInitiationSource::Unknown),
    ///                 text_only: Some(true),
    ///                 conversation_product_type: Some(ConversationProduct::Agents),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 version_id: Some("version_id".to_string()),
    ///                 parent_conversation_id: Some("parent_conversation_id".to_string()),
    ///                 topic_ids: vec![Some("topic_ids".to_string())],
    ///                 exclude_statuses: vec![Some(
    ///                     ConversationsListRequestExcludeStatusesItem::Initiated,
    ///                 )],
    ///                 tag_ids: vec![Some("tag_ids".to_string())],
    ///                 workflow_node_entered_id: Some("workflow_node_entered_id".to_string()),
    ///                 termination_reasons: vec![Some("termination_reasons".to_string())],
    ///                 guardrail_types: vec![Some(GuardrailType::Custom)],
    ///                 custom_guardrail_names: vec![Some("custom_guardrail_names".to_string())],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversations",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
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
                    .string_array("data_collection_ids", request.data_collection_ids.clone())
                    .string_array(
                        "evaluation_criteria_ids",
                        request.evaluation_criteria_ids.clone(),
                    )
                    .string_array("tool_names", request.tool_names.clone())
                    .string_array(
                        "tool_names_successful",
                        request.tool_names_successful.clone(),
                    )
                    .string_array("tool_names_errored", request.tool_names_errored.clone())
                    .string_array("main_languages", request.main_languages.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("summary_mode", request.summary_mode.clone())
                    .string("search", request.search.clone())
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
                    .string("version_id", request.version_id.clone())
                    .string(
                        "parent_conversation_id",
                        request.parent_conversation_id.clone(),
                    )
                    .string_array("topic_ids", request.topic_ids.clone())
                    .serialize_array("exclude_statuses", request.exclude_statuses.clone())
                    .string_array("tag_ids", request.tag_ids.clone())
                    .string(
                        "workflow_node_entered_id",
                        request.workflow_node_entered_id.clone(),
                    )
                    .string_array("termination_reasons", request.termination_reasons.clone())
                    .serialize_array("guardrail_types", request.guardrail_types.clone())
                    .string_array(
                        "custom_guardrail_names",
                        request.custom_guardrail_names.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Resolve a conversation URL (a Slack message URL or a Zendesk ticket URL) to the deterministic conversation ID for the given agent, then confirm the conversation exists.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    /// * `reference` - A Slack message URL or a Zendesk ticket URL.
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
    ///         .resolve(
    ///             &ResolveQueryRequest {
    ///                 agent_id: "agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///                 reference: "https://your-domain.zendesk.com/agent/tickets/12345".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resolve(
        &self,
        request: &ResolveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/conversations/resolve",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .string("reference", request.reference.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get the details of a particular conversation
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
    /// * `format` - Response format. Defaults to 'json'. Set to 'opentelemetry' for an OTLP-compatible trace payload using the same structure as the post-call webhook.
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
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &AgentsConversationsGetQueryRequest {
    ///                 format: Some(ConversationsGetRequestFormat::JSON),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        conversation_id: &str,
        request: &AgentsConversationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/conversations/{}", conversation_id),
                None,
                QueryBuilder::new()
                    .serialize("format", request.format.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a particular conversation
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
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
    ///         .delete(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        conversation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/conversations/{}", conversation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get SIP messages associated with a conversation's phone call
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - The id of the conversation you're taking the action on.
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
    ///         .get_sip_messages(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &AgentsConversationsGetSipMessagesQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_sip_messages(
        &self,
        conversation_id: &str,
        request: &AgentsConversationsGetSipMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSipLogMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/conversations/{}/sip-messages", conversation_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
