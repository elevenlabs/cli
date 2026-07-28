pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsToolsDeleteQueryRequest {
    /// If set to true, the tool will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AgentsToolsDeleteQueryRequest {
    pub fn builder() -> AgentsToolsDeleteQueryRequestBuilder {
        <AgentsToolsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsToolsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AgentsToolsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsToolsDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsToolsDeleteQueryRequest, BuildError> {
        Ok(AgentsToolsDeleteQueryRequest {
            force: self.force,
        })
    }
}

