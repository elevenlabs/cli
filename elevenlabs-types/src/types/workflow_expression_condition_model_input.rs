pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExpressionConditionModelInput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Expression to evaluate.
    pub expression: AstNodeInput,
}

impl WorkflowExpressionConditionModelInput {
    pub fn builder() -> WorkflowExpressionConditionModelInputBuilder {
        <WorkflowExpressionConditionModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExpressionConditionModelInputBuilder {
    label: Option<String>,
    expression: Option<AstNodeInput>,
}

impl WorkflowExpressionConditionModelInputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn expression(mut self, value: AstNodeInput) -> Self {
        self.expression = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExpressionConditionModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expression`](WorkflowExpressionConditionModelInputBuilder::expression)
    pub fn build(self) -> Result<WorkflowExpressionConditionModelInput, BuildError> {
        Ok(WorkflowExpressionConditionModelInput {
            label: self.label,
            expression: self.expression.ok_or_else(|| BuildError::missing_field("expression"))?,
        })
    }
}
