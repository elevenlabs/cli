use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InvocationsClient {
    pub http_client: HttpClient,
}

impl InvocationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists all test invocations with pagination support and optional search filtering.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Filter by agent ID
    /// * `branch_id` - Filter by branch ID
    /// * `test_id` - Filter to invocations that include a run of this test id
    /// * `start_time` - Inclusive lower bound on created_at, in unix seconds
    /// * `end_time` - Inclusive upper bound on created_at, in unix seconds
    /// * `version_id` - Filter by the agent version the tests ran against
    /// * `page_size` - How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    /// * `search` - Search query to filter tests and folders by name.
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
    ///         .tests
    ///         .invocations
    ///         .list(
    ///             &AgentsTestsInvocationsListQueryRequest {
    ///                 agent_id: Some("agent_id".to_string()),
    ///                 branch_id: Some("branch_id".to_string()),
    ///                 test_id: Some("test_id".to_string()),
    ///                 start_time: Some(1),
    ///                 end_time: Some(1),
    ///                 version_id: Some("version_id".to_string()),
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
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
        request: &AgentsTestsInvocationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTestInvocationsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/test-invocations",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .string("test_id", request.test_id.clone())
                    .int("start_time", request.start_time.clone())
                    .int("end_time", request.end_time.clone())
                    .string("version_id", request.version_id.clone())
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Gets a test invocation by ID.
    ///
    /// # Arguments
    ///
    /// * `test_invocation_id` - The id of a test invocation. This is returned when tests are run.
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
    ///         .tests
    ///         .invocations
    ///         .get(&"test_invocation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        test_invocation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetTestSuiteInvocationResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/test-invocations/{}", test_invocation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancels all pending runs in a test invocation.
    ///
    /// # Arguments
    ///
    /// * `test_invocation_id` - The id of a test invocation. This is returned when tests are run.
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
    ///         .tests
    ///         .invocations
    ///         .cancel(&"test_invocation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        test_invocation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/test-invocations/{}/cancel", test_invocation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resubmits specific test runs from a test invocation.
    ///
    /// # Arguments
    ///
    /// * `test_invocation_id` - The id of a test invocation. This is returned when tests are run.
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
    ///         .tests
    ///         .invocations
    ///         .resubmit(
    ///             &"test_invocation_id".to_string(),
    ///             &ResubmitTestsRequestModel {
    ///                 test_run_ids: vec!["test_run_ids".to_string()],
    ///                 agent_id: "agent_id".to_string(),
    ///                 agent_config_override: None,
    ///                 branch_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resubmit(
        &self,
        test_invocation_id: &str,
        request: &ResubmitTestsRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/test-invocations/{}/resubmit", test_invocation_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
