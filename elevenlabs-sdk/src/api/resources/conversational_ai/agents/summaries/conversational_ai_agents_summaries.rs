use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct SummariesClient {
    pub http_client: HttpClient,
}

impl SummariesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns summaries for the specified agents.
    ///
    /// # Arguments
    ///
    /// * `agent_ids` - List of agent IDs to fetch summaries for
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
    ///         .summaries
    ///         .get(
    ///             &ConversationalAiAgentsSummariesGetQueryRequest {
    ///                 agent_ids: vec![Some("agent_ids".to_string())],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &ConversationalAiAgentsSummariesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, SummariesGetResponseValue>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/agents/summaries",
                None,
                QueryBuilder::new()
                    .string_array("agent_ids", request.agent_ids.clone())
                    .build(),
                options,
            )
            .await
    }
}
