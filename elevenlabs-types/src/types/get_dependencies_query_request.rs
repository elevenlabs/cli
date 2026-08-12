pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_dependencies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDependenciesQueryRequest {
    /// How many dependency items to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDependenciesQueryRequest {
    pub fn builder() -> GetDependenciesQueryRequestBuilder {
        <GetDependenciesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDependenciesQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl GetDependenciesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDependenciesQueryRequest`].
    pub fn build(self) -> Result<GetDependenciesQueryRequest, BuildError> {
        Ok(GetDependenciesQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

