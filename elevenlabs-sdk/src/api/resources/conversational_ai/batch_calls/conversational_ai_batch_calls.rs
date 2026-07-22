use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BatchCallsClient {
    pub http_client: HttpClient,
}

impl BatchCallsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Submit a batch call request to schedule calls for multiple recipients.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &BodySubmitABatchCallRequestV1ConvaiBatchCallingSubmitPost,
        options: Option<RequestOptions>,
    ) -> Result<BatchCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/convai/batch-calling/submit",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get all batch calls for the current workspace.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Filter batch calls to a single agent.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationalAiBatchCallsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceBatchCallsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/convai/batch-calling/workspace",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("last_doc", request.last_doc.clone())
                    .string("agent_id", request.agent_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get detailed information about a batch call including all recipients.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        batch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BatchCallDetailedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/convai/batch-calling/{}", batch_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a batch call and all recipient records. Conversations remain in history.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    pub async fn delete(
        &self,
        batch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/convai/batch-calling/{}", batch_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancel a running batch call and set all recipients to cancelled status.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel(
        &self,
        batch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BatchCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/batch-calling/{}/cancel", batch_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retry a batch call, calling failed and no-response recipients again.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retry(
        &self,
        batch_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BatchCallResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/convai/batch-calling/{}/retry", batch_id),
                None,
                None,
                options,
            )
            .await
    }
}
