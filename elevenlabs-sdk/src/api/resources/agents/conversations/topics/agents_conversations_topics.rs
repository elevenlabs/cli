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
    /// * `page_size` - Number of top-level topic groups to return.
    /// * `sort_by` - Topic table column to sort by.
    /// * `sort_direction` - Direction to sort topics.
    /// * `from_unix_secs` - Start of the window to view topics for. When set with to_unix_secs, per-day topics in the range are aggregated together.
    /// * `to_unix_secs` - End of the window to view topics for.
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
    ///         .topics
    ///         .get(
    ///             &"agent_id".to_string(),
    ///             &AgentsConversationsTopicsGetQueryRequest {
    ///                 page_size: Some(1),
    ///                 sort_by: Some(TopicSortBy::Conversations),
    ///                 sort_direction: Some(SortDirection::Asc),
    ///                 from_unix_secs: Some(1),
    ///                 to_unix_secs: Some(1),
    ///                 cursor: Some("cursor".to_string()),
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
        request: &AgentsConversationsTopicsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAgentTopicsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/agents/{}/topics", agent_id),
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("sort_by", request.sort_by.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .int("from_unix_secs", request.from_unix_secs.clone())
                    .int("to_unix_secs", request.to_unix_secs.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
