pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVersionsQueryRequest {
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListVersionsQueryRequest {
    pub fn builder() -> ListVersionsQueryRequestBuilder {
        <ListVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVersionsQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl ListVersionsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListVersionsQueryRequest`].
    pub fn build(self) -> Result<ListVersionsQueryRequest, BuildError> {
        Ok(ListVersionsQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

