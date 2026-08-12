pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsProceduresGetQueryRequest {
    /// The version ID to retrieve. If omitted, returns the version at branch HEAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl AgentsProceduresGetQueryRequest {
    pub fn builder() -> AgentsProceduresGetQueryRequestBuilder {
        <AgentsProceduresGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsProceduresGetQueryRequestBuilder {
    version_id: Option<String>,
}

impl AgentsProceduresGetQueryRequestBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsProceduresGetQueryRequest`].
    pub fn build(self) -> Result<AgentsProceduresGetQueryRequest, BuildError> {
        Ok(AgentsProceduresGetQueryRequest {
            version_id: self.version_id,
        })
    }
}

