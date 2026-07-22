pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowEdgeModelInput {
    /// ID of the source node.
    #[serde(default)]
    pub source: String,
    /// ID of the target node.
    #[serde(default)]
    pub target: String,
    /// Condition that must be met for the edge to be traversed in the forward direction (source to target).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_condition: Option<WorkflowEdgeModelInputForwardCondition>,
    /// Condition that must be met for the edge to be traversed in the backward direction (target to source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_condition: Option<WorkflowEdgeModelInputBackwardCondition>,
}

impl WorkflowEdgeModelInput {
    pub fn builder() -> WorkflowEdgeModelInputBuilder {
        <WorkflowEdgeModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowEdgeModelInputBuilder {
    source: Option<String>,
    target: Option<String>,
    forward_condition: Option<WorkflowEdgeModelInputForwardCondition>,
    backward_condition: Option<WorkflowEdgeModelInputBackwardCondition>,
}

impl WorkflowEdgeModelInputBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn forward_condition(mut self, value: WorkflowEdgeModelInputForwardCondition) -> Self {
        self.forward_condition = Some(value);
        self
    }

    pub fn backward_condition(mut self, value: WorkflowEdgeModelInputBackwardCondition) -> Self {
        self.backward_condition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowEdgeModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](WorkflowEdgeModelInputBuilder::source)
    /// - [`target`](WorkflowEdgeModelInputBuilder::target)
    pub fn build(self) -> Result<WorkflowEdgeModelInput, BuildError> {
        Ok(WorkflowEdgeModelInput {
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            forward_condition: self.forward_condition,
            backward_condition: self.backward_condition,
        })
    }
}
