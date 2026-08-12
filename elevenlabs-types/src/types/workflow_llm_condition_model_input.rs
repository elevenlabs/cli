pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowLlmConditionModelInput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Condition to evaluate
    #[serde(default)]
    pub condition: String,
}

impl WorkflowLlmConditionModelInput {
    pub fn builder() -> WorkflowLlmConditionModelInputBuilder {
        <WorkflowLlmConditionModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowLlmConditionModelInputBuilder {
    label: Option<String>,
    condition: Option<String>,
}

impl WorkflowLlmConditionModelInputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowLlmConditionModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](WorkflowLlmConditionModelInputBuilder::condition)
    pub fn build(self) -> Result<WorkflowLlmConditionModelInput, BuildError> {
        Ok(WorkflowLlmConditionModelInput {
            label: self.label,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
        })
    }
}
