use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsageClient {
    pub http_client: HttpClient,
}

impl UsageClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// (Deprecated) This endpoint is deprecated. Use /v1/workspace/analytics/query/usage-by-product-over-time instead, which exposes the bucket size as `interval_seconds` (an integer in seconds) rather than `aggregation_interval`. Returns the usage metrics for the current user or the entire workspace they are part of. The response provides a time axis based on the specified aggregation interval (default: day), with usage values for each interval along that axis. Usage is broken down by the selected breakdown type. For example, breakdown type "voice" will return the usage of each voice for each interval along the time axis.
    ///
    /// # Arguments
    ///
    /// * `start_unix` - UTC Unix timestamp for the start of the usage window, in milliseconds. To include the first day of the window, the timestamp should be at 00:00:00 of that day.
    /// * `end_unix` - UTC Unix timestamp for the end of the usage window, in milliseconds. To include the last day of the window, the timestamp should be at 23:59:59 of that day.
    /// * `include_workspace_metrics` - Whether or not to include the statistics of the entire workspace.
    /// * `breakdown_type` - How to break down the information. Cannot be "user" if include_workspace_metrics is False.
    /// * `aggregation_interval` - How to aggregate usage data over time. Can be "hour", "day", "week", "month", or "cumulative".
    /// * `aggregation_bucket_size` - Aggregation bucket size in seconds. Overrides the aggregation interval.
    /// * `metric` - Which metric to aggregate.
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
    ///         .usage
    ///         .get(
    ///             &UsageGetQueryRequest {
    ///                 start_unix: 1,
    ///                 end_unix: 1,
    ///                 include_workspace_metrics: Some(true),
    ///                 breakdown_type: Some(BreakdownTypes::None),
    ///                 aggregation_interval: Some(UsageAggregationInterval::Hour),
    ///                 aggregation_bucket_size: Some(1),
    ///                 metric: Some(MetricType::Credits),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &UsageGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UsageCharactersResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/usage/character-stats",
                None,
                QueryBuilder::new()
                    .int("start_unix", request.start_unix.clone())
                    .int("end_unix", request.end_unix.clone())
                    .bool(
                        "include_workspace_metrics",
                        request.include_workspace_metrics.clone(),
                    )
                    .serialize("breakdown_type", request.breakdown_type.clone())
                    .serialize("aggregation_interval", request.aggregation_interval.clone())
                    .int(
                        "aggregation_bucket_size",
                        request.aggregation_bucket_size.clone(),
                    )
                    .serialize("metric", request.metric.clone())
                    .build(),
                options,
            )
            .await
    }
}
