use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TriageTicketsClient {
    pub http_client: HttpClient,
}

impl TriageTicketsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List an agent's conversation triage tickets, ordered by most recently created first unless sorted by priority. These are tickets about the agent's own performance on a conversation (for triage with Architect), not tickets an agent opens for end users.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many agent conversation tickets to return. Can not exceed 100.
    /// * `conversation_id` - Filter tickets by conversation id.
    /// * `status` - Filter tickets by status.
    /// * `sources` - Filter tickets by how they were raised (qa, agent, manual). Repeat the parameter to filter by multiple sources.
    /// * `priorities` - Filter tickets by priority. Repeat the parameter to filter by multiple priorities.
    /// * `sort_by` - Order by most recently created, or by priority (most urgent first, then most recently created).
    /// * `owner_user_id` - Filter tickets by creator. Use 'agent' for agent-raised tickets.
    /// * `assignee_user_id` - Filter tickets by assignee. Use 'unassigned' for tickets with no assignee.
    /// * `issue_type` - Filter clusters by issue type.
    /// * `label` - Filter tickets by an exact label.
    /// * `merged_into_ticket_id` - Filter tickets merged into this ticket.
    /// * `search` - Case-insensitive free-text search across the ticket's title, description, comments, and turn comments.
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
    ///         .triage_tickets
    ///         .list(
    ///             &"agent_id".to_string(),
    ///             &AgentsTriageTicketsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 conversation_id: Some("conversation_id".to_string()),
    ///                 status: Some(AgentConversationTicketStatus::Open),
    ///                 sources: vec![Some(AgentConversationTicketSource::Qa)],
    ///                 priorities: vec![Some(AgentConversationTicketPriority::Low)],
    ///                 sort_by: Some(AgentConversationTicketSortBy::CreatedAt),
    ///                 owner_user_id: Some("owner_user_id".to_string()),
    ///                 assignee_user_id: Some("assignee_user_id".to_string()),
    ///                 issue_type: Some(AgentConversationTicketIssueType::KnowledgeGap),
    ///                 label: Some("label".to_string()),
    ///                 merged_into_ticket_id: Some("merged_into_ticket_id".to_string()),
    ///                 search: Some("search".to_string()),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        agent_id: &str,
        request: &AgentsTriageTicketsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentConversationTicketsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/triage-tickets", agent_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("conversation_id", request.conversation_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize_array("sources", request.sources.clone())
                    .serialize_array("priorities", request.priorities.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("owner_user_id", request.owner_user_id.clone())
                    .string("assignee_user_id", request.assignee_user_id.clone())
                    .serialize("issue_type", request.issue_type.clone())
                    .string("label", request.label.clone())
                    .string(
                        "merged_into_ticket_id",
                        request.merged_into_ticket_id.clone(),
                    )
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Manually raise a follow-up ticket against an agent, not tied to any conversation (for example a task like 'add the KB about X'). Without a title, one is derived from the comment. Requires viewer access to the agent.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .create_manual(
    ///             &"agent_id".to_string(),
    ///             &CreateManualTicketRequestModel {
    ///                 qa_comment: "qa_comment".to_string(),
    ///                 title: None,
    ///                 priority: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_manual(
        &self,
        agent_id: &str,
        request: &CreateManualTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/triage-tickets", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List conversation triage tickets across every agent in the workspace, ordered by most recently created first. Use this to build a workspace-wide view (for example, tickets assigned to the caller). Supports the same filters, sorting and search as the per-agent endpoint. Tickets for agents the caller cannot access are omitted.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many agent conversation tickets to return. Can not exceed 100.
    /// * `agent_id` - Only tickets for this agent.
    /// * `status` - Filter tickets by status.
    /// * `sources` - Filter tickets by how they were raised (qa, agent, manual). Repeat the parameter to filter by multiple sources.
    /// * `priorities` - Filter tickets by priority. Repeat the parameter to filter by multiple priorities.
    /// * `sort_by` - Order by most recently created, or by priority (most urgent first, then most recently created).
    /// * `owner_user_id` - Filter tickets by creator. Use 'agent' for agent-raised tickets.
    /// * `assignee_user_id` - Filter tickets by assignee. Use 'unassigned' for tickets with no assignee.
    /// * `issue_type` - Filter clusters by issue type.
    /// * `label` - Filter tickets by an exact label.
    /// * `merged_into_ticket_id` - Filter tickets merged into this ticket.
    /// * `search` - Case-insensitive free-text search across the ticket's title, description, comments, and turn comments.
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
    ///         .triage_tickets
    ///         .list_for_workspace(
    ///             &ListForWorkspaceQueryRequest {
    ///                 page_size: Some(1),
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 status: Some(AgentConversationTicketStatus::Open),
    ///                 sources: vec![Some(AgentConversationTicketSource::Qa)],
    ///                 priorities: vec![Some(AgentConversationTicketPriority::Low)],
    ///                 sort_by: Some(AgentConversationTicketSortBy::CreatedAt),
    ///                 owner_user_id: Some("owner_user_id".to_string()),
    ///                 assignee_user_id: Some("assignee_user_id".to_string()),
    ///                 issue_type: Some(AgentConversationTicketIssueType::KnowledgeGap),
    ///                 label: Some("label".to_string()),
    ///                 merged_into_ticket_id: Some("merged_into_ticket_id".to_string()),
    ///                 search: Some("search".to_string()),
    ///                 cursor: Some("cursor".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_for_workspace(
        &self,
        request: &ListForWorkspaceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentConversationTicketsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/triage-tickets",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("agent_id", request.agent_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize_array("sources", request.sources.clone())
                    .serialize_array("priorities", request.priorities.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("owner_user_id", request.owner_user_id.clone())
                    .string("assignee_user_id", request.assignee_user_id.clone())
                    .serialize("issue_type", request.issue_type.clone())
                    .string("label", request.label.clone())
                    .string(
                        "merged_into_ticket_id",
                        request.merged_into_ticket_id.clone(),
                    )
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Raise a ticket about an agent's performance on a conversation, for triage with Architect. Provide an overall comment and/or turn-level comments describing what went wrong.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .create(
    ///             &CreateAgentConversationTicketRequestModel {
    ///                 conversation_id: "conversation_id".to_string(),
    ///                 title: None,
    ///                 qa_comment: None,
    ///                 turn_comments: None,
    ///                 priority: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAgentConversationTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/triage-tickets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// All non-service-account workspace members, each flagged with whether they currently have at least viewer access to the agent. Members without access are included (not filtered out) so the UI can offer them as an assignee and prompt to grant access first.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .list_assignable_users(&"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_assignable_users(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AssignableUserResponseModel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/convai/agents/{}/triage-tickets/assignable-users",
                    agent_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get an agent conversation ticket by ID.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .get(&"agentqa_ticket_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        agentqa_ticket_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/triage-tickets/{}", agentqa_ticket_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete an agent conversation ticket. Restricted to the ticket creator or a workspace admin.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .triage_tickets
    ///         .delete(&"agentqa_ticket_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        agentqa_ticket_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/triage-tickets/{}", agentqa_ticket_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a ticket's title, comment, status, priority, and/or assignee. Requires editor access to the ticket's agent.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .update(
    ///             &"agentqa_ticket_id".to_string(),
    ///             &PatchAgentConversationTicketRequestModel {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        agentqa_ticket_id: &str,
        request: &PatchAgentConversationTicketRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/triage-tickets/{}", agentqa_ticket_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Append a comment discussing how to resolve the ticket. Requires viewer access to the ticket's agent.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .add_comment(
    ///             &"agentqa_ticket_id".to_string(),
    ///             &AddTicketCommentRequestModel {
    ///                 comment: "comment".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_comment(
        &self,
        agentqa_ticket_id: &str,
        request: &AddTicketCommentRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/triage-tickets/{}/comments", agentqa_ticket_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Merge other open tickets of the same agent into this pending, in-progress, or resolved one. Their conversations move onto this ticket, its priority is raised to the highest among them, and they are kept with status 'merged'. Requires viewer access to the ticket's agent.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .merge(
    ///             &"agentqa_ticket_id".to_string(),
    ///             &MergeAgentConversationTicketsRequestModel {
    ///                 source_ticket_ids: vec!["source_ticket_ids".to_string()],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn merge(
        &self,
        agentqa_ticket_id: &str,
        request: &MergeAgentConversationTicketsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/triage-tickets/{}/merge", agentqa_ticket_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Append a turn-level comment to a ticket. Requires viewer access to the ticket's agent.
    ///
    /// # Arguments
    ///
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
    ///         .triage_tickets
    ///         .add_turn_comment(
    ///             &"agentqa_ticket_id".to_string(),
    ///             &AddTurnCommentRequestModel {
    ///                 turn_index: 1,
    ///                 comment: "comment".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_turn_comment(
        &self,
        agentqa_ticket_id: &str,
        request: &AddTurnCommentRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<AgentConversationTicketResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/convai/triage-tickets/{}/turn-comments",
                    agentqa_ticket_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
