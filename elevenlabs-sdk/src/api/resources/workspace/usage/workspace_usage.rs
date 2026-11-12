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

    /// Returns credit usage broken down by product type over time, for workspaces billed in credits. The response is a tabular structure with columns, column_types, column_units, and rows. Some id columns also get a display-name column, such as user_label or voice_name, appended after the existing columns. For workspaces billed by cost, ranges that reach the date the workspace moved to cost billing return an error; use /v1/workspace/analytics/usage instead.
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

    /// Returns the cost of the workspace's usage over time, for workspaces billed by cost. Usage is bucketed by UTC day. The response is a tabular structure with columns, column_types, column_units, and rows, with one row per group and bucket. `total_cost` is the cost in the workspace's currency and `total_charge_count` the billed quantity. Usage can be grouped or filtered by at most two of `user_id`, `hashed_xi_api_key`, `region`, `reporting_workspace_id`, `billing_group_id`, `product_type`, `surface` and `actor`, and grouping by two of them requires an `eq` or `in` filter on one. `model`, `request_source`, `fiat_charge_type` and `fiat_currency` can always be added. Some id columns also get a display-name column, such as user_label, appended after the existing columns. Workspaces billed in credits should use /v1/workspace/analytics/query/usage-by-product-over-time.
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
    ///         .get_usage(
    ///             &BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost {
    ///                 start_time: 1,
    ///                 end_time: 1,
    ///                 interval_seconds: 1,
    ///                 group_by: None,
    ///                 filters: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_usage(
        &self,
        request: &BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceAnalyticsQueryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/workspace/analytics/usage",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
