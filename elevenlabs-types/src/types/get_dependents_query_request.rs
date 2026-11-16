pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_dependents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDependentsQueryRequest {
    /// How many Agents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDependentsQueryRequest {
    pub fn builder() -> GetDependentsQueryRequestBuilder {
        <GetDependentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDependentsQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl GetDependentsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDependentsQueryRequest`].
    pub fn build(self) -> Result<GetDependentsQueryRequest, BuildError> {
        Ok(GetDependentsQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

