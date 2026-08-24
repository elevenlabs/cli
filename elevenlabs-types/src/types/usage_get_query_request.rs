pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsageGetQueryRequest {
    /// UTC Unix timestamp for the start of the usage window, in milliseconds. To include the first day of the window, the timestamp should be at 00:00:00 of that day.
    #[serde(default)]
    pub start_unix: i64,
    /// UTC Unix timestamp for the end of the usage window, in milliseconds. To include the last day of the window, the timestamp should be at 23:59:59 of that day.
    #[serde(default)]
    pub end_unix: i64,
    /// Whether or not to include the statistics of the entire workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_workspace_metrics: Option<bool>,
    /// How to break down the information. Cannot be "user" if include_workspace_metrics is False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown_type: Option<BreakdownTypes>,
    /// How to aggregate usage data over time. Can be "hour", "day", "week", "month", or "cumulative".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_interval: Option<UsageAggregationInterval>,
    /// Aggregation bucket size in seconds. Overrides the aggregation interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_bucket_size: Option<i64>,
    /// Which metric to aggregate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<MetricType>,
}

impl UsageGetQueryRequest {
    pub fn builder() -> UsageGetQueryRequestBuilder {
        <UsageGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsageGetQueryRequestBuilder {
    start_unix: Option<i64>,
    end_unix: Option<i64>,
    include_workspace_metrics: Option<bool>,
    breakdown_type: Option<BreakdownTypes>,
    aggregation_interval: Option<UsageAggregationInterval>,
    aggregation_bucket_size: Option<i64>,
    metric: Option<MetricType>,
}

impl UsageGetQueryRequestBuilder {
    pub fn start_unix(mut self, value: i64) -> Self {
        self.start_unix = Some(value);
        self
    }

    pub fn end_unix(mut self, value: i64) -> Self {
        self.end_unix = Some(value);
        self
    }

    pub fn include_workspace_metrics(mut self, value: bool) -> Self {
        self.include_workspace_metrics = Some(value);
        self
    }

    pub fn breakdown_type(mut self, value: BreakdownTypes) -> Self {
        self.breakdown_type = Some(value);
        self
    }

    pub fn aggregation_interval(mut self, value: UsageAggregationInterval) -> Self {
        self.aggregation_interval = Some(value);
        self
    }

    pub fn aggregation_bucket_size(mut self, value: i64) -> Self {
        self.aggregation_bucket_size = Some(value);
        self
    }

    pub fn metric(mut self, value: MetricType) -> Self {
        self.metric = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsageGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_unix`](UsageGetQueryRequestBuilder::start_unix)
    /// - [`end_unix`](UsageGetQueryRequestBuilder::end_unix)
    pub fn build(self) -> Result<UsageGetQueryRequest, BuildError> {
        Ok(UsageGetQueryRequest {
            start_unix: self.start_unix.ok_or_else(|| BuildError::missing_field("start_unix"))?,
            end_unix: self.end_unix.ok_or_else(|| BuildError::missing_field("end_unix"))?,
            include_workspace_metrics: self.include_workspace_metrics,
            breakdown_type: self.breakdown_type,
            aggregation_interval: self.aggregation_interval,
            aggregation_bucket_size: self.aggregation_bucket_size,
            metric: self.metric,
        })
    }
}

