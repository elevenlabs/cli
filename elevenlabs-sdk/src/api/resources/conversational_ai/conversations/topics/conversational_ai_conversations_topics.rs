use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TopicsClient {
    pub http_client: HttpClient,
}

impl TopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the latest topic discovery run results for a given agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - ID of the agent
    /// * `from_unix_secs` - Start of the window to view topics for. When set with to_unix_secs, per-day topics in the range are aggregated together.
    /// * `to_unix_secs` - End of the window to view topics for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        agent_id: &str,
        request: &ConversationalAiConversationsTopicsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentTopicsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/topics", agent_id),
                None,
                QueryBuilder::new()
                    .int("from_unix_secs", request.from_unix_secs.clone())
                    .int("to_unix_secs", request.to_unix_secs.clone())
                    .build(),
                options,
            )
            .await
    }
}
