pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsGetQueryRequest {
    /// The ID of the agent version to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// The ID of the branch to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl AgentsGetQueryRequest {
    pub fn builder() -> AgentsGetQueryRequestBuilder {
        <AgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsGetQueryRequestBuilder {
    version_id: Option<String>,
    branch_id: Option<String>,
}

impl AgentsGetQueryRequestBuilder {
    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsGetQueryRequest, BuildError> {
        Ok(AgentsGetQueryRequest {
            version_id: self.version_id,
            branch_id: self.branch_id,
        })
    }
}

