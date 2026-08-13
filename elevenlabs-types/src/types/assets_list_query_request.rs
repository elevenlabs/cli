pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetsListQueryRequest {
    /// Number of assets to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Token from a previous response's `next_cursor`. Omit to fetch the first page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Optional free-text search filter over asset names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl AssetsListQueryRequest {
    pub fn builder() -> AssetsListQueryRequestBuilder {
        <AssetsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetsListQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
    search: Option<String>,
}

impl AssetsListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssetsListQueryRequest`].
    pub fn build(self) -> Result<AssetsListQueryRequest, BuildError> {
        Ok(AssetsListQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
            search: self.search,
        })
    }
}

