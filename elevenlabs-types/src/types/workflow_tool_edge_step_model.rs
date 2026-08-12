pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolEdgeStepModel {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub step_latency_secs: f64,
    #[serde(default)]
    pub edge_id: String,
    #[serde(default)]
    pub target_node_id: String,
}

impl WorkflowToolEdgeStepModel {
    pub fn builder() -> WorkflowToolEdgeStepModelBuilder {
        <WorkflowToolEdgeStepModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolEdgeStepModelBuilder {
    step_latency_secs: Option<f64>,
    edge_id: Option<String>,
    target_node_id: Option<String>,
}

impl WorkflowToolEdgeStepModelBuilder {
    pub fn step_latency_secs(mut self, value: f64) -> Self {
        self.step_latency_secs = Some(value);
        self
    }

    pub fn edge_id(mut self, value: impl Into<String>) -> Self {
        self.edge_id = Some(value.into());
        self
    }

    pub fn target_node_id(mut self, value: impl Into<String>) -> Self {
        self.target_node_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolEdgeStepModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`step_latency_secs`](WorkflowToolEdgeStepModelBuilder::step_latency_secs)
    /// - [`edge_id`](WorkflowToolEdgeStepModelBuilder::edge_id)
    /// - [`target_node_id`](WorkflowToolEdgeStepModelBuilder::target_node_id)
    pub fn build(self) -> Result<WorkflowToolEdgeStepModel, BuildError> {
        Ok(WorkflowToolEdgeStepModel {
            step_latency_secs: self.step_latency_secs.ok_or_else(|| BuildError::missing_field("step_latency_secs"))?,
            edge_id: self.edge_id.ok_or_else(|| BuildError::missing_field("edge_id"))?,
            target_node_id: self.target_node_id.ok_or_else(|| BuildError::missing_field("target_node_id"))?,
        })
    }
}
