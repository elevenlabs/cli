pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowResultConditionModelInput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether all tools in the previously executed tool node were executed successfully.
    #[serde(default)]
    pub successful: bool,
}

impl WorkflowResultConditionModelInput {
    pub fn builder() -> WorkflowResultConditionModelInputBuilder {
        <WorkflowResultConditionModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowResultConditionModelInputBuilder {
    label: Option<String>,
    successful: Option<bool>,
}

impl WorkflowResultConditionModelInputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn successful(mut self, value: bool) -> Self {
        self.successful = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowResultConditionModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`successful`](WorkflowResultConditionModelInputBuilder::successful)
    pub fn build(self) -> Result<WorkflowResultConditionModelInput, BuildError> {
        Ok(WorkflowResultConditionModelInput {
            label: self.label,
            successful: self.successful.ok_or_else(|| BuildError::missing_field("successful"))?,
        })
    }
}
