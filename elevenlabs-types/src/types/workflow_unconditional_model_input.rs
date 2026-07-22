pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowUnconditionalModelInput {
    /// Optional human-readable label for the condition used throughout the UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl WorkflowUnconditionalModelInput {
    pub fn builder() -> WorkflowUnconditionalModelInputBuilder {
        <WorkflowUnconditionalModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUnconditionalModelInputBuilder {
    label: Option<String>,
}

impl WorkflowUnconditionalModelInputBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUnconditionalModelInput`].
    pub fn build(self) -> Result<WorkflowUnconditionalModelInput, BuildError> {
        Ok(WorkflowUnconditionalModelInput {
            label: self.label,
        })
    }
}
