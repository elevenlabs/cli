pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolMaxIterationsExceededStepModel {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub step_latency_secs: f64,
    #[serde(default)]
    pub max_iterations: i64,
}

impl WorkflowToolMaxIterationsExceededStepModel {
    pub fn builder() -> WorkflowToolMaxIterationsExceededStepModelBuilder {
        <WorkflowToolMaxIterationsExceededStepModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolMaxIterationsExceededStepModelBuilder {
    step_latency_secs: Option<f64>,
    max_iterations: Option<i64>,
}

impl WorkflowToolMaxIterationsExceededStepModelBuilder {
    pub fn step_latency_secs(mut self, value: f64) -> Self {
        self.step_latency_secs = Some(value);
        self
    }

    pub fn max_iterations(mut self, value: i64) -> Self {
        self.max_iterations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolMaxIterationsExceededStepModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`step_latency_secs`](WorkflowToolMaxIterationsExceededStepModelBuilder::step_latency_secs)
    /// - [`max_iterations`](WorkflowToolMaxIterationsExceededStepModelBuilder::max_iterations)
    pub fn build(self) -> Result<WorkflowToolMaxIterationsExceededStepModel, BuildError> {
        Ok(WorkflowToolMaxIterationsExceededStepModel {
            step_latency_secs: self.step_latency_secs.ok_or_else(|| BuildError::missing_field("step_latency_secs"))?,
            max_iterations: self.max_iterations.ok_or_else(|| BuildError::missing_field("max_iterations"))?,
        })
    }
}
