pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowEdgeModelOutput {
    /// ID of the source node.
    #[serde(default)]
    pub source: String,
    /// ID of the target node.
    #[serde(default)]
    pub target: String,
    /// Condition that must be met for the edge to be traversed in the forward direction (source to target).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_condition: Option<WorkflowEdgeModelOutputForwardCondition>,
    /// Condition that must be met for the edge to be traversed in the backward direction (target to source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward_condition: Option<WorkflowEdgeModelOutputBackwardCondition>,
}

impl WorkflowEdgeModelOutput {
    pub fn builder() -> WorkflowEdgeModelOutputBuilder {
        <WorkflowEdgeModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowEdgeModelOutputBuilder {
    source: Option<String>,
    target: Option<String>,
    forward_condition: Option<WorkflowEdgeModelOutputForwardCondition>,
    backward_condition: Option<WorkflowEdgeModelOutputBackwardCondition>,
}

impl WorkflowEdgeModelOutputBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn forward_condition(mut self, value: WorkflowEdgeModelOutputForwardCondition) -> Self {
        self.forward_condition = Some(value);
        self
    }

    pub fn backward_condition(mut self, value: WorkflowEdgeModelOutputBackwardCondition) -> Self {
        self.backward_condition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowEdgeModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](WorkflowEdgeModelOutputBuilder::source)
    /// - [`target`](WorkflowEdgeModelOutputBuilder::target)
    pub fn build(self) -> Result<WorkflowEdgeModelOutput, BuildError> {
        Ok(WorkflowEdgeModelOutput {
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            forward_condition: self.forward_condition,
            backward_condition: self.backward_condition,
        })
    }
}
