use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct UsageClient2 {
    pub http_client: HttpClient,
}

impl UsageClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns credit usage broken down by product type over time. The response is a tabular structure with columns, column_types, column_units, and rows.
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
    ///         .workspace
    ///         .usage
    ///         .get_usage_by_product_over_time(
    ///             &BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost {
    ///                 start_time: 1,
    ///                 end_time: 1,
    ///                 interval_seconds: None,
    ///                 group_by: None,
    ///                 filters: None,
    ///                 time_zone: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_usage_by_product_over_time(
        &self,
        request: &BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAnalyticsQueryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspace/analytics/query/usage-by-product-over-time",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
