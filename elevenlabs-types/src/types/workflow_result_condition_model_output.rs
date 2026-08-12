pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowResultConditionModelOutput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether all tools in the previously executed tool node were executed successfully.
    #[serde(default)]
    pub successful: bool,
}

impl WorkflowResultConditionModelOutput {
    pub fn builder() -> WorkflowResultConditionModelOutputBuilder {
        <WorkflowResultConditionModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowResultConditionModelOutputBuilder {
    label: Option<String>,
    successful: Option<bool>,
}

impl WorkflowResultConditionModelOutputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn successful(mut self, value: bool) -> Self {
        self.successful = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowResultConditionModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`successful`](WorkflowResultConditionModelOutputBuilder::successful)
    pub fn build(self) -> Result<WorkflowResultConditionModelOutput, BuildError> {
        Ok(WorkflowResultConditionModelOutput {
            label: self.label,
            successful: self.successful.ok_or_else(|| BuildError::missing_field("successful"))?,
        })
    }
}
