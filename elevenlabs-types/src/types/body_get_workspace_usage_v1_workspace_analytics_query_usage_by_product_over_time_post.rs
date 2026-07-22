pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost {
    /// Start of the time range as a Unix timestamp in milliseconds. Must be at least 2020-01-01.
    #[serde(default)]
    pub start_time: i64,
    /// End of the time range as a Unix timestamp in milliseconds. Must be at least 2020-01-01.
    #[serde(default)]
    pub end_time: i64,
    /// Bucket size in seconds. Each row in the response covers this many seconds of the selected time range. For example, pass 3600 for hourly buckets or 86400 for daily buckets. Whether `time_zone` shifts bucket boundaries depends on this value: whole-day multiples (e.g. 86400) align to local midnight; whole-hour multiples up to 24 hours (e.g. 3600, 14400) align to local hour boundaries from midnight; sub-hour values and other sizes remain UTC-anchored regardless of `time_zone`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ColumnFilter>>,
    /// IANA time zone identifier (e.g. 'America/New_York', 'Europe/London', 'UTC') used to align bucket boundaries for eligible `interval_seconds` values. Whole-day multiples start at local midnight; whole-hour multiples up to 24 hours align to local hour boundaries from midnight. Sub-hour intervals and other bucket sizes remain UTC-anchored regardless of this setting. Defaults to UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost {
    pub fn builder() -> BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder {
        <BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    interval_seconds: Option<i64>,
    group_by: Option<Vec<BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem>>,
    filters: Option<Vec<ColumnFilter>>,
    time_zone: Option<String>,
}

impl BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn interval_seconds(mut self, value: i64) -> Self {
        self.interval_seconds = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostGroupByItem>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<ColumnFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder::start_time)
    /// - [`end_time`](BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePostBuilder::end_time)
    pub fn build(self) -> Result<BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost, BuildError> {
        Ok(BodyGetWorkspaceUsageV1WorkspaceAnalyticsQueryUsageByProductOverTimePost {
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            interval_seconds: self.interval_seconds,
            group_by: self.group_by,
            filters: self.filters,
            time_zone: self.time_zone,
        })
    }
}

