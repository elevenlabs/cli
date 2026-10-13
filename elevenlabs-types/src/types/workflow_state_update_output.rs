pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowStateUpdateOutput {
    pub r#type: String,
    /// Name of the dynamic variable to assign.
    #[serde(default)]
    pub variable_name: String,
    /// Expression whose result is assigned to the variable. It can combine constants, other dynamic variables, LLM-extracted values and operators.
    pub expression: AstNodeOutput,
    /// When false, the LLM may omit the values this update relies on. The update is then skipped and the variable keeps its current value.
    #[serde(default)]
    pub required: bool,
}

impl WorkflowStateUpdateOutput {
    pub fn builder() -> WorkflowStateUpdateOutputBuilder {
        <WorkflowStateUpdateOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowStateUpdateOutputBuilder {
    r#type: Option<String>,
    variable_name: Option<String>,
    expression: Option<AstNodeOutput>,
    required: Option<bool>,
}

impl WorkflowStateUpdateOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn variable_name(mut self, value: impl Into<String>) -> Self {
        self.variable_name = Some(value.into());
        self
    }

    pub fn expression(mut self, value: AstNodeOutput) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowStateUpdateOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WorkflowStateUpdateOutputBuilder::r#type)
    /// - [`variable_name`](WorkflowStateUpdateOutputBuilder::variable_name)
    /// - [`expression`](WorkflowStateUpdateOutputBuilder::expression)
    /// - [`required`](WorkflowStateUpdateOutputBuilder::required)
    pub fn build(self) -> Result<WorkflowStateUpdateOutput, BuildError> {
        Ok(WorkflowStateUpdateOutput {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            variable_name: self.variable_name.ok_or_else(|| BuildError::missing_field("variable_name"))?,
            expression: self.expression.ok_or_else(|| BuildError::missing_field("expression"))?,
            required: self.required.ok_or_else(|| BuildError::missing_field("required"))?,
        })
    }
}
