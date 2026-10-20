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
    /// Whether to include the workflow nodes and edges generated from structured procedures. When false, the workflow contains only authored elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_procedure_generated_workflow_elements: Option<bool>,
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
    include_procedure_generated_workflow_elements: Option<bool>,
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

    pub fn include_procedure_generated_workflow_elements(mut self, value: bool) -> Self {
        self.include_procedure_generated_workflow_elements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsGetQueryRequest, BuildError> {
        Ok(AgentsGetQueryRequest {
            version_id: self.version_id,
            branch_id: self.branch_id,
            include_procedure_generated_workflow_elements: self.include_procedure_generated_workflow_elements,
        })
    }
}

