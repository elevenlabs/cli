pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompileProceduresResponseModel {
    /// Generated workflow from compilation
    #[serde(default)]
    pub workflow: AgentWorkflowResponseModel,
}

impl CompileProceduresResponseModel {
    pub fn builder() -> CompileProceduresResponseModelBuilder {
        <CompileProceduresResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompileProceduresResponseModelBuilder {
    workflow: Option<AgentWorkflowResponseModel>,
}

impl CompileProceduresResponseModelBuilder {
    pub fn workflow(mut self, value: AgentWorkflowResponseModel) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompileProceduresResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow`](CompileProceduresResponseModelBuilder::workflow)
    pub fn build(self) -> Result<CompileProceduresResponseModel, BuildError> {
        Ok(CompileProceduresResponseModel {
            workflow: self.workflow.ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
