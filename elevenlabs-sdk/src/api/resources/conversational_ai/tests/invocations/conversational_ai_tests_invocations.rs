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
    /// * `page_size` - How many Tests to return at maximum. Can not exceed 100, defaults to 30.
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiTestsInvocationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTestInvocationsPageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/test-invocations",
                None,
                QueryBuilder::new()
                    .string("agent_id", request.agent_id.clone())
                    .int("page_size", request.page_size.clone())
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
