use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LiveCountClient {
    pub http_client: HttpClient,
}

impl LiveCountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get the live count of the ongoing conversations.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The id of an agent to restrict the analytics to.
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
    ///         .analytics
    ///         .live_count
    ///         .get(
    ///             &AgentsAnalyticsLiveCountGetQueryRequest {
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &AgentsAnalyticsLiveCountGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLiveCountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/analytics/live-count",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
