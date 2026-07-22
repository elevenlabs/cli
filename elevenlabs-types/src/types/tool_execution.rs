pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Execution-related properties for a tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolExecution {
    #[serde(rename = "taskSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_support: Option<ToolExecutionTaskSupport>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ToolExecution {
    pub fn builder() -> ToolExecutionBuilder {
        <ToolExecutionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolExecutionBuilder {
    task_support: Option<ToolExecutionTaskSupport>,
}

impl ToolExecutionBuilder {
    pub fn task_support(mut self, value: ToolExecutionTaskSupport) -> Self {
        self.task_support = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolExecution`].
    pub fn build(self) -> Result<ToolExecution, BuildError> {
        Ok(ToolExecution {
            task_support: self.task_support,
            extra: Default::default(),
        })
    }
}
