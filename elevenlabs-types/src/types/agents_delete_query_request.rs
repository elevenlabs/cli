pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeleteQueryRequest {
    /// If set to true, the agent is deleted even if other agents use it as a subagent, and it is removed from those agents and their branches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AgentsDeleteQueryRequest {
    pub fn builder() -> AgentsDeleteQueryRequestBuilder {
        <AgentsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AgentsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsDeleteQueryRequest, BuildError> {
        Ok(AgentsDeleteQueryRequest {
            force: self.force,
        })
    }
}

