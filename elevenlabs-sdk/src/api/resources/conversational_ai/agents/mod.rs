use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod summaries;
pub use summaries::SummariesClient;
pub mod widget;
pub use widget::WidgetClient;
pub mod link;
pub use link::LinkClient;
pub mod knowledge_base;
pub use knowledge_base::KnowledgeBaseClient2;
pub mod llm_usage;
pub use llm_usage::LlmUsageClient2;
pub mod branches;
pub use branches::BranchesClient;
pub mod versions;
pub use versions::VersionsClient;
pub mod deployments;
pub use deployments::DeploymentsClient;
pub mod drafts;
pub use drafts::DraftsClient;
pub struct AgentsClient {
    pub http_client: HttpClient,
    pub summaries: SummariesClient,
    pub widget: WidgetClient,
    pub link: LinkClient,
    pub knowledge_base: KnowledgeBaseClient2,
    pub llm_usage: LlmUsageClient2,
    pub branches: BranchesClient,
    pub versions: VersionsClient,
    pub deployments: DeploymentsClient,
    pub drafts: DraftsClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            summaries: SummariesClient::new(config.clone())?,
            widget: WidgetClient::new(config.clone())?,
            link: LinkClient::new(config.clone())?,
            knowledge_base: KnowledgeBaseClient2::new(config.clone())?,
            llm_usage: LlmUsageClient2::new(config.clone())?,
            branches: BranchesClient::new(config.clone())?,
            versions: VersionsClient::new(config.clone())?,
            deployments: DeploymentsClient::new(config.clone())?,
            drafts: DraftsClient::new(config.clone())?,
        })
    }

    /// Create an agent from a config object
    ///
    /// # Arguments
    ///
    /// * `enable_versioning` - Deprecated: all agents are versioned. This parameter is ignored.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &BodyCreateAgentV1ConvaiAgentsCreatePost,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/agents/create",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("enable_versioning", request.enable_versioning.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve config for an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `version_id` - The ID of the agent version to use
    /// * `branch_id` - The ID of the branch to use
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        agent_id: &str,
        request: &ConversationalAiAgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}", agent_id),
                None,
                QueryBuilder::new()
                    .string("version_id", request.version_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete an agent
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/agents/{}", agent_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches an Agent settings
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `enable_versioning_if_not_enabled` - Deprecated: all agents are versioned. This parameter is ignored.
    /// * `branch_id` - The ID of the branch to use
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        agent_id: &str,
        request: &UpdateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/convai/agents/{}", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool(
                        "enable_versioning_if_not_enabled",
                        request.enable_versioning_if_not_enabled.clone(),
                    )
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a list of your agents and their metadata.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many Agents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `search` - Search by agents name.
    /// * `archived` - Filter agents by archived status
    /// * `show_only_owned_agents` - If set to true, the endpoint will omit any agents that were shared with you by someone else and include only the ones you own. Deprecated: use created_by_user_id instead.
    /// * `created_by_user_id` - Filter agents by creator user ID. When set, only agents created by this user are returned. Takes precedence over show_only_owned_agents. Use '@me' to refer to the authenticated user.
    /// * `sort_direction` - The direction to sort the results
    /// * `sort_by` - The field to sort the results by
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiAgentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/agents",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .bool("archived", request.archived.clone())
                    .bool(
                        "show_only_owned_agents",
                        request.show_only_owned_agents.clone(),
                    )
                    .string("created_by_user_id", request.created_by_user_id.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new agent by duplicating an existing one
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn duplicate(
        &self,
        agent_id: &str,
        request: &BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost,
        options: Option<RequestOptions>,
    ) -> Result<CreateAgentResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/duplicate", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deprecated. Use the `/v1/convai/agent-testing/create` and `/v1/convai/agents/:agent_id/run-tests` endpoints to create and run simulations. Run a conversation between the agent and a simulated user.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn simulate_conversation(
        &self,
        agent_id: &str,
        request: &BodySimulatesAConversationV1ConvaiAgentsAgentIdSimulateConversationPost,
        options: Option<RequestOptions>,
    ) -> Result<AgentSimulatedChatTestResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/simulate-conversation", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deprecated. Use the `/v1/convai/agent-testing/create` and `/v1/convai/agents/:agent_id/run-tests` endpoints to create and run simulations. Run a conversation between the agent and a simulated user and stream back the response. Response is streamed back as partial lists of messages that should be concatenated and once the conversation has complete a single final message with the conversation analysis will be sent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn simulate_conversation_stream(
        &self,
        agent_id: &str,
        request: &BodySimulatesAConversationStreamV1ConvaiAgentsAgentIdSimulateConversationStreamPost,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/simulate-conversation/stream", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Run selected tests on the agent with provided configuration. If the agent configuration is provided, it will be used to override default agent configuration.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent. This is returned on agent creation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn run_tests(
        &self,
        agent_id: &str,
        request: &RunAgentTestsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<GetTestSuiteInvocationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/agents/{}/run-tests", agent_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
