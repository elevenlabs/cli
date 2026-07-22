pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_dependent_agents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDependentAgentsQueryRequest {
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl GetDependentAgentsQueryRequest {
    pub fn builder() -> GetDependentAgentsQueryRequestBuilder {
        <GetDependentAgentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDependentAgentsQueryRequestBuilder {
    cursor: Option<String>,
    page_size: Option<i64>,
}

impl GetDependentAgentsQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDependentAgentsQueryRequest`].
    pub fn build(self) -> Result<GetDependentAgentsQueryRequest, BuildError> {
        Ok(GetDependentAgentsQueryRequest {
            cursor: self.cursor,
            page_size: self.page_size,
        })
    }
}

