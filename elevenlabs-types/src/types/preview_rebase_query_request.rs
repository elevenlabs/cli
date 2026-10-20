pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for preview_rebase
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewRebaseQueryRequest {
    /// The ID of the branch to pull changes from. Defaults to the agent's main branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_branch_id: Option<String>,
    /// Whether to include the workflow nodes and edges generated from structured procedures. When false, the workflow contains only authored elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_procedure_generated_workflow_elements: Option<bool>,
}

impl PreviewRebaseQueryRequest {
    pub fn builder() -> PreviewRebaseQueryRequestBuilder {
        <PreviewRebaseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewRebaseQueryRequestBuilder {
    source_branch_id: Option<String>,
    include_procedure_generated_workflow_elements: Option<bool>,
}

impl PreviewRebaseQueryRequestBuilder {
    pub fn source_branch_id(mut self, value: impl Into<String>) -> Self {
        self.source_branch_id = Some(value.into());
        self
    }

    pub fn include_procedure_generated_workflow_elements(mut self, value: bool) -> Self {
        self.include_procedure_generated_workflow_elements = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewRebaseQueryRequest`].
    pub fn build(self) -> Result<PreviewRebaseQueryRequest, BuildError> {
        Ok(PreviewRebaseQueryRequest {
            source_branch_id: self.source_branch_id,
            include_procedure_generated_workflow_elements: self.include_procedure_generated_workflow_elements,
        })
    }
}

