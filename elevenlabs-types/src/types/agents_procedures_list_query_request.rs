pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsProceduresListQueryRequest {
    /// The agent version ID to retrieve the procedure for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version_id: Option<String>,
}

impl AgentsProceduresListQueryRequest {
    pub fn builder() -> AgentsProceduresListQueryRequestBuilder {
        <AgentsProceduresListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsProceduresListQueryRequestBuilder {
    agent_version_id: Option<String>,
}

impl AgentsProceduresListQueryRequestBuilder {
    pub fn agent_version_id(mut self, value: impl Into<String>) -> Self {
        self.agent_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsProceduresListQueryRequest`].
    pub fn build(self) -> Result<AgentsProceduresListQueryRequest, BuildError> {
        Ok(AgentsProceduresListQueryRequest {
            agent_version_id: self.agent_version_id,
        })
    }
}

