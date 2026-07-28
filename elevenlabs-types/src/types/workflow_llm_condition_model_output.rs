pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowLlmConditionModelOutput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Condition to evaluate
    #[serde(default)]
    pub condition: String,
}

impl WorkflowLlmConditionModelOutput {
    pub fn builder() -> WorkflowLlmConditionModelOutputBuilder {
        <WorkflowLlmConditionModelOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowLlmConditionModelOutputBuilder {
    label: Option<String>,
    condition: Option<String>,
}

impl WorkflowLlmConditionModelOutputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn condition(mut self, value: impl Into<String>) -> Self {
        self.condition = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowLlmConditionModelOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](WorkflowLlmConditionModelOutputBuilder::condition)
    pub fn build(self) -> Result<WorkflowLlmConditionModelOutput, BuildError> {
        Ok(WorkflowLlmConditionModelOutput {
            label: self.label,
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
        })
    }
}
