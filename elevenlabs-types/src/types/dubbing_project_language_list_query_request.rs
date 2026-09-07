pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingProjectLanguageListQueryRequest {
    /// Pass the `next_cursor` from a previous response to fetch the page after it. Omit for the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of language targets per page. Clamped to between 1 and 100 rather than rejected, so a larger value returns a full page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Filter to targets in this status: `queued`, `processing`, `completed`, `stale`, or `failed`. Omit to return every status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DubbingProjectLanguageListQueryRequest {
    pub fn builder() -> DubbingProjectLanguageListQueryRequestBuilder {
        <DubbingProjectLanguageListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingProjectLanguageListQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
    status: Option<String>,
}

impl DubbingProjectLanguageListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`DubbingProjectLanguageListQueryRequest`].
    pub fn build(self) -> Result<DubbingProjectLanguageListQueryRequest, BuildError> {
        Ok(DubbingProjectLanguageListQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
            status: self.status,
        })
    }
}

