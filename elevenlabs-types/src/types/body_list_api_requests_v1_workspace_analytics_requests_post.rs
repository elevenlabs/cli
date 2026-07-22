pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost {
    /// Start of the time range as a Unix timestamp in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// End of the time range as a Unix timestamp in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Optional timestamp sort direction. If omitted, defaults to desc when end_time is provided, otherwise asc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ColumnFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost {
    pub fn builder() -> BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostBuilder {
        <BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostBuilder {
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i64>,
    sort: Option<BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort>,
    filters: Option<Vec<ColumnFilter>>,
    search: Option<String>,
}

impl BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostBuilder {
    pub fn start_time(mut self, value: i64) -> Self {
        self.start_time = Some(value);
        self
    }

    pub fn end_time(mut self, value: i64) -> Self {
        self.end_time = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: BodyListApiRequestsV1WorkspaceAnalyticsRequestsPostSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<ColumnFilter>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost`].
    pub fn build(self) -> Result<BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost, BuildError> {
        Ok(BodyListApiRequestsV1WorkspaceAnalyticsRequestsPost {
            start_time: self.start_time,
            end_time: self.end_time,
            limit: self.limit,
            sort: self.sort,
            filters: self.filters,
            search: self.search,
        })
    }
}

