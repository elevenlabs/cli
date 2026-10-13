pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeploymentsListQueryRequest {
    /// Page number, starting at 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// How many results at most should be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl AgentsDeploymentsListQueryRequest {
    pub fn builder() -> AgentsDeploymentsListQueryRequestBuilder {
        <AgentsDeploymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeploymentsListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl AgentsDeploymentsListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeploymentsListQueryRequest`].
    pub fn build(self) -> Result<AgentsDeploymentsListQueryRequest, BuildError> {
        Ok(AgentsDeploymentsListQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}

