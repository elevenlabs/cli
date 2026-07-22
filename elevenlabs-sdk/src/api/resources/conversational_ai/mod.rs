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
pub mod agents;
pub use agents::AgentsClient;
pub mod tests;
pub use tests::TestsClient;
pub mod users;
pub use users::UsersClient;
pub mod phone_numbers;
pub use phone_numbers::PhoneNumbersClient;
pub mod llm_usage;
pub use llm_usage::LlmUsageClient;
pub mod llm;
pub use llm::LlmClient;
pub mod knowledge_base;
pub use knowledge_base::KnowledgeBaseClient;
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
pub mod analytics;
pub use analytics::AnalyticsClient;
pub mod dashboard;
pub use dashboard::DashboardClient;
pub struct ConversationalAiClient {
    pub http_client: HttpClient,
    pub conversations: ConversationsClient,
    pub twilio: TwilioClient,
    pub exotel: ExotelClient,
    pub whatsapp: WhatsappClient,
    pub agents: AgentsClient,
    pub tests: TestsClient,
    pub users: UsersClient,
    pub phone_numbers: PhoneNumbersClient,
    pub llm_usage: LlmUsageClient,
    pub llm: LlmClient,
    pub knowledge_base: KnowledgeBaseClient,
    pub tools: ToolsClient,
    pub settings: SettingsClient,
    pub secrets: SecretsClient,
    pub batch_calls: BatchCallsClient,
    pub sip_trunk: SipTrunkClient,
    pub mcp_servers: McpServersClient,
    pub whatsapp_accounts: WhatsappAccountsClient,
    pub analytics: AnalyticsClient,
    pub dashboard: DashboardClient,
}

impl ConversationalAiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            twilio: TwilioClient::new(config.clone())?,
            exotel: ExotelClient::new(config.clone())?,
            whatsapp: WhatsappClient::new(config.clone())?,
            agents: AgentsClient::new(config.clone())?,
            tests: TestsClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            phone_numbers: PhoneNumbersClient::new(config.clone())?,
            llm_usage: LlmUsageClient::new(config.clone())?,
            llm: LlmClient::new(config.clone())?,
            knowledge_base: KnowledgeBaseClient::new(config.clone())?,
            tools: ToolsClient::new(config.clone())?,
            settings: SettingsClient::new(config.clone())?,
            secrets: SecretsClient::new(config.clone())?,
            batch_calls: BatchCallsClient::new(config.clone())?,
            sip_trunk: SipTrunkClient::new(config.clone())?,
            mcp_servers: McpServersClient::new(config.clone())?,
            whatsapp_accounts: WhatsappAccountsClient::new(config.clone())?,
            analytics: AnalyticsClient::new(config.clone())?,
            dashboard: DashboardClient::new(config.clone())?,
        })
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
