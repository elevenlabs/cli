pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsAnalysisItemsDeleteQueryRequest {
    /// If set to true, the analysis item will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AgentsAnalysisItemsDeleteQueryRequest {
    pub fn builder() -> AgentsAnalysisItemsDeleteQueryRequestBuilder {
        <AgentsAnalysisItemsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsAnalysisItemsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AgentsAnalysisItemsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsAnalysisItemsDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsAnalysisItemsDeleteQueryRequest, BuildError> {
        Ok(AgentsAnalysisItemsDeleteQueryRequest {
            force: self.force,
        })
    }
}

