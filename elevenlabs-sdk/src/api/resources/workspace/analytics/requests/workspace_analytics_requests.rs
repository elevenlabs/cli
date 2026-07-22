use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RequestsClient {
    pub http_client: HttpClient,
}

impl RequestsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a list of API requests. Supports filtering by time range, column filters, and search terms. At least one of start_time or end_time must be provided. An optional sort parameter controls timestamp ordering. Results are ordered by timestamp. Descending if end_time is used, ascending if start_time is used. The response is a tabular structure with columns, column_types, column_units, and rows.
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
        request: &BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAnalyticsQueryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspace/analytics/requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
