use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExecutionsClient {
    pub http_client: HttpClient,
}

impl ExecutionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get paginated list of tool executions for a specific tool.
    ///
    /// # Arguments
    ///
    /// * `tool_id` - ID of the requested tool.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `page_size` - How many documents to return at maximum. Can not exceed 100, defaults to 30.
    /// * `is_error` - Filter by error status. If not provided, returns all executions.
    /// * `agent_id` - Filter by agent ID.
    /// * `branch_id` - Filter by agent branch ID.
    /// * `start_time` - Filter executions from this Unix timestamp (inclusive).
    /// * `end_time` - Filter executions until this Unix timestamp (inclusive).
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
    ///         .tools
    ///         .executions
    ///         .get(
    ///             &"tool_id".to_string(),
    ///             &AgentsToolsExecutionsGetQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 is_error: Some(true),
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 start_time: Some(1.1),
    ///                 end_time: Some(1.1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        tool_id: &str,
        request: &AgentsToolsExecutionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetToolExecutionsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/tools/{}/executions", tool_id),
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .bool("is_error", request.is_error.clone())
                    .string("agent_id", request.agent_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .float("start_time", request.start_time.clone())
                    .float("end_time", request.end_time.clone())
                    .build(),
                options,
            )
            .await
    }
}
