pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingProjectListQueryRequest {
    /// Pagination cursor from a previous response's next_cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of projects per page (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter to projects in this status (preparing, ready, failed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Sort by creation time (default 'DESCENDING').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<ProjectListRequestSortDirection>,
}

impl DubbingProjectListQueryRequest {
    pub fn builder() -> DubbingProjectListQueryRequestBuilder {
        <DubbingProjectListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingProjectListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<String>,
    sort_direction: Option<ProjectListRequestSortDirection>,
}

impl DubbingProjectListQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: ProjectListRequestSortDirection) -> Self {
        self.sort_direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingProjectListQueryRequest`].
    pub fn build(self) -> Result<DubbingProjectListQueryRequest, BuildError> {
        Ok(DubbingProjectListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
            sort_direction: self.sort_direction,
        })
    }
}

