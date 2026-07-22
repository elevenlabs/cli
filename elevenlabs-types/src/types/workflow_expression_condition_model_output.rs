pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExpressionConditionModelOutput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Expression to evaluate.
    pub expression: AstNodeOutput,
}

impl WorkflowExpressionConditionModelOutput {
    pub fn builder() -> WorkflowExpressionConditionModelOutputBuilder {
        <WorkflowExpressionConditionModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExpressionConditionModelOutputBuilder {
    label: Option<String>,
    expression: Option<AstNodeOutput>,
}

impl WorkflowExpressionConditionModelOutputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn expression(mut self, value: AstNodeOutput) -> Self {
        self.expression = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExpressionConditionModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expression`](WorkflowExpressionConditionModelOutputBuilder::expression)
    pub fn build(self) -> Result<WorkflowExpressionConditionModelOutput, BuildError> {
        Ok(WorkflowExpressionConditionModelOutput {
            label: self.label,
            expression: self.expression.ok_or_else(|| BuildError::missing_field("expression"))?,
        })
    }
}
