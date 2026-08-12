pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A common model for workflow tool responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolResponseModelOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Box<WorkflowToolResponseModelOutputStepsItem>>>,
}

impl WorkflowToolResponseModelOutput {
    pub fn builder() -> WorkflowToolResponseModelOutputBuilder {
        <WorkflowToolResponseModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolResponseModelOutputBuilder {
    steps: Option<Vec<Box<WorkflowToolResponseModelOutputStepsItem>>>,
}

impl WorkflowToolResponseModelOutputBuilder {
    pub fn steps(mut self, value: Vec<Box<WorkflowToolResponseModelOutputStepsItem>>) -> Self {
        self.steps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolResponseModelOutput`].
    pub fn build(self) -> Result<WorkflowToolResponseModelOutput, BuildError> {
        Ok(WorkflowToolResponseModelOutput {
            steps: self.steps,
        })
    }
}
