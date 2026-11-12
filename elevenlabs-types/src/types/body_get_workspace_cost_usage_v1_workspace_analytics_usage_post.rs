pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost {
    /// Start of the time range as a Unix timestamp in milliseconds. Must be at least 2020-01-01.
    #[serde(default)]
    pub start_time: i64,
    /// End of the time range as a Unix timestamp in milliseconds. Must be at least 2020-01-01.
    #[serde(default)]
    pub end_time: i64,
    /// Bucket size in seconds, in whole UTC days. Pass 86400 for daily buckets or 604800 for weekly buckets, which start on Monday.
    #[serde(default)]
    pub interval_seconds: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MetronomeUsageColumnFilter>>,
}

impl BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost {
    pub fn builder() -> BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder {
        <BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    interval_seconds: Option<i64>,
    group_by: Option<Vec<BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem>>,
    filters: Option<Vec<MetronomeUsageColumnFilter>>,
}

impl BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder {
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

    pub fn group_by(mut self, value: Vec<BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostGroupByItem>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<MetronomeUsageColumnFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time`](BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder::start_time)
    /// - [`end_time`](BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder::end_time)
    /// - [`interval_seconds`](BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePostBuilder::interval_seconds)
    pub fn build(self) -> Result<BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost, BuildError> {
        Ok(BodyGetWorkspaceCostUsageV1WorkspaceAnalyticsUsagePost {
            start_time: self.start_time.ok_or_else(|| BuildError::missing_field("start_time"))?,
            end_time: self.end_time.ok_or_else(|| BuildError::missing_field("end_time"))?,
            interval_seconds: self.interval_seconds.ok_or_else(|| BuildError::missing_field("interval_seconds"))?,
            group_by: self.group_by,
            filters: self.filters,
        })
    }
}

