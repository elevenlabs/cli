pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsTestsFoldersDeleteQueryRequest {
    /// Force delete. Required for deleting non-empty folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AgentsTestsFoldersDeleteQueryRequest {
    pub fn builder() -> AgentsTestsFoldersDeleteQueryRequestBuilder {
        <AgentsTestsFoldersDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsTestsFoldersDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AgentsTestsFoldersDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsTestsFoldersDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsTestsFoldersDeleteQueryRequest, BuildError> {
        Ok(AgentsTestsFoldersDeleteQueryRequest {
            force: self.force,
        })
    }
}

