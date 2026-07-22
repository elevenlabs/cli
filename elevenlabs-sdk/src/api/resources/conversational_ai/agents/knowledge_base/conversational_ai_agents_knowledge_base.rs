use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct KnowledgeBaseClient2 {
    pub http_client: HttpClient,
}

impl KnowledgeBaseClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the number of pages in the agent's knowledge base.
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
    ///         .conversational_ai
    ///         .agents
    ///         .knowledge_base
    ///         .size(&"agent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn size(
        &self,
        agent_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentKnowledgebaseSizeResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agent/{}/knowledge-base/size", agent_id),
                None,
                None,
                options,
            )
            .await
    }
}
