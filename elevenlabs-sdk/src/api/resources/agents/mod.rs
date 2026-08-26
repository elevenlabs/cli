use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod conversations;
pub use conversations::ConversationsClient;
pub mod twilio;
pub use twilio::TwilioClient;
pub mod exotel;
pub use exotel::ExotelClient;
pub mod whatsapp;
pub use whatsapp::WhatsappClient;
pub mod summaries;
pub use summaries::SummariesClient;
pub mod widget;
pub use widget::WidgetClient;
pub mod link;
pub use link::LinkClient;
pub mod knowledge_base;
pub use knowledge_base::KnowledgeBaseClient;
pub mod tests;
pub use tests::TestsClient;
pub mod users;
pub use users::UsersClient;
pub mod triage_tickets;
pub use triage_tickets::TriageTicketsClient;
pub mod phone_numbers;
pub use phone_numbers::PhoneNumbersClient;
pub mod llm_usage;
pub use llm_usage::LlmUsageClient;
pub mod llm;
pub use llm::LlmClient;
pub mod tools;
pub use tools::ToolsClient;
pub mod settings;
pub use settings::SettingsClient;
pub mod secrets;
pub use secrets::SecretsClient;
pub mod batch_calls;
pub use batch_calls::BatchCallsClient;
pub mod sip_trunk;
pub use sip_trunk::SipTrunkClient;
pub mod mcp_servers;
pub use mcp_servers::McpServersClient;
pub mod whatsapp_accounts;
pub use whatsapp_accounts::WhatsappAccountsClient;
pub mod branches;
pub use branches::BranchesClient;
pub mod versions;
pub use versions::VersionsClient;
pub mod deployments;
pub use deployments::DeploymentsClient;
pub mod drafts;
pub use drafts::DraftsClient;
pub mod procedures;
pub use procedures::ProceduresClient;
pub mod agents;
pub use agents::AgentsClient2;
pub mod analytics;
pub use analytics::AnalyticsClient;
pub mod dashboard;
pub use dashboard::DashboardClient;
pub struct AgentsClient {
    pub http_client: HttpClient,
    pub conversations: ConversationsClient,
    pub twilio: TwilioClient,
    pub exotel: ExotelClient,
    pub whatsapp: WhatsappClient,
    pub summaries: SummariesClient,
    pub widget: WidgetClient,
    pub link: LinkClient,
    pub knowledge_base: KnowledgeBaseClient,
    pub tests: TestsClient,
    pub users: UsersClient,
    pub triage_tickets: TriageTicketsClient,
    pub phone_numbers: PhoneNumbersClient,
    pub llm_usage: LlmUsageClient,
    pub llm: LlmClient,
    pub tools: ToolsClient,
    pub settings: SettingsClient,
    pub secrets: SecretsClient,
    pub batch_calls: BatchCallsClient,
    pub sip_trunk: SipTrunkClient,
    pub mcp_servers: McpServersClient,
    pub whatsapp_accounts: WhatsappAccountsClient,
    pub branches: BranchesClient,
    pub versions: VersionsClient,
    pub deployments: DeploymentsClient,
    pub drafts: DraftsClient,
    pub procedures: ProceduresClient,
    pub agents: AgentsClient2,
    pub analytics: AnalyticsClient,
    pub dashboard: DashboardClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            twilio: TwilioClient::new(config.clone())?,
            exotel: ExotelClient::new(config.clone())?,
            whatsapp: WhatsappClient::new(config.clone())?,
            summaries: SummariesClient::new(config.clone())?,
            widget: WidgetClient::new(config.clone())?,
            link: LinkClient::new(config.clone())?,
            knowledge_base: KnowledgeBaseClient::new(config.clone())?,
            tests: TestsClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            triage_tickets: TriageTicketsClient::new(config.clone())?,
            phone_numbers: PhoneNumbersClient::new(config.clone())?,
            llm_usage: LlmUsageClient::new(config.clone())?,
            llm: LlmClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
            settings: SettingsClient::new(config.clone())?,
            secrets: SecretsClient::new(config.clone())?,
            batch_calls: BatchCallsClient::new(config.clone())?,
            sip_trunk: SipTrunkClient::new(config.clone())?,
            mcp_servers: McpServersClient::new(config.clone())?,
            whatsapp_accounts: WhatsappAccountsClient::new(config.clone())?,
            branches: BranchesClient::new(config.clone())?,
            versions: VersionsClient::new(config.clone())?,
            deployments: DeploymentsClient::new(config.clone())?,
            drafts: DraftsClient::new(config.clone())?,
            procedures: ProceduresClient::new(config.clone())?,
            agents: AgentsClient2::new(config.clone())?,
            analytics: AnalyticsClient::new(config.clone())?,
            dashboard: DashboardClient::new(config.clone())?,
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
    ///         .create(
    ///             &BodyCreateAgentV1ConvaiAgentsCreatePost {
    ///                 enable_versioning: Some(true),
    ///                 conversation_config: ConversationalConfig {
    ///                     ..Default::default()
    ///                 },
    ///                 platform_settings: None,
    ///                 workflow: None,
    ///                 name: None,
    ///                 tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .get(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &AgentsGetQueryRequest {
    ///                 version_id: Some("version_id".to_string()),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        agent_id: &str,
        request: &AgentsGetQueryRequest,
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
    ///         .delete(&"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(), None)
    ///         .await;
    /// }
    /// ```
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
    ///         .update(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &UpdateAgentRequest {
    ///                 enable_versioning_if_not_enabled: Some(true),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .list(
    ///             &AgentsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 archived: Some(true),
    ///                 show_only_owned_agents: Some(true),
    ///                 created_by_user_id: Some("created_by_user_id".to_string()),
    ///                 sort_direction: Some(SortDirection::Asc),
    ///                 sort_by: Some(AgentSortBy::Name),
    ///                 cursor: Some("cursor".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AgentsListQueryRequest,
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
    ///         .duplicate(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodyDuplicateAgentV1ConvaiAgentsAgentIDDuplicatePost {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .simulate_conversation(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodySimulatesAConversationV1ConvaiAgentsAgentIDSimulateConversationPost {
    ///                 simulation_specification: ConversationSimulationSpecification {
    ///                     simulated_user_config: AgentConfig {
    ///                         first_message: Some("Hello, how can I help you today?".to_string()),
    ///                         language: Some("en".to_string()),
    ///                         disable_first_message_interruptions: Some(false),
    ///                         ..Default::default()
    ///                     },
    ///                     ..Default::default()
    ///                 },
    ///                 extra_evaluation_criteria: None,
    ///                 new_turns_limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .simulate_conversation_stream(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &BodySimulatesAConversationStreamV1ConvaiAgentsAgentIDSimulateConversationStreamPost {
    ///                 simulation_specification: ConversationSimulationSpecification {
    ///                     simulated_user_config: AgentConfig {
    ///                         first_message: Some("Hello, how can I help you today?".to_string()),
    ///                         language: Some("en".to_string()),
    ///                         disable_first_message_interruptions: Some(false),
    ///                         ..Default::default()
    ///                     },
    ///                     ..Default::default()
    ///                 },
    ///                 extra_evaluation_criteria: None,
    ///                 new_turns_limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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
    ///         .run_tests(
    ///             &"agent_3701k3ttaq12ewp8b7qv5rfyszkz".to_string(),
    ///             &RunAgentTestsRequestModel {
    ///                 tests: vec![SingleTestRunRequestModel {
    ///                     test_id: "test_id".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 agent_config_override: None,
    ///                 branch_id: None,
    ///                 repeat_count: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
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

    /// Upload a file or webpage URL to create a knowledge base document. <br> <Note> After creating the document, update the agent's knowledge base by calling [Update agent](/docs/api-reference/agents/update). </Note>
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
    ///         .add_to_knowledge_base(
    ///             &AddToKnowledgeBaseRequest {
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 file: b"test file content".to_vec(),
    ///                 name: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_to_knowledge_base(
        &self,
        request: &AddToKnowledgeBaseRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddKnowledgeBaseResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/convai/knowledge-base",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Provides total size and other information of RAG indexes used by knowledgebase documents
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
    ///     client.agents.rag_index_overview(None).await;
    /// }
    /// ```
    pub async fn rag_index_overview(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<RagIndexOverviewResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/knowledge-base/rag-index",
                None,
                None,
                options,
            )
            .await
    }

    /// Provides information about all RAG indexes of the specified knowledgebase document.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
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
    ///         .get_document_rag_indexes(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_document_rag_indexes(
        &self,
        documentation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RagDocumentIndexesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/knowledge-base/{}/rag-index", documentation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete RAG index for the knowledgebase document.
    ///
    /// # Arguments
    ///
    /// * `documentation_id` - The id of a document from the knowledge base. This is returned on document addition.
    /// * `rag_index_id` - The id of RAG index of document from the knowledge base.
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
    ///         .delete_document_rag_index(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_document_rag_index(
        &self,
        documentation_id: &str,
        rag_index_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RagDocumentIndexResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/convai/knowledge-base/{}/rag-index/{}",
                    documentation_id, rag_index_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
