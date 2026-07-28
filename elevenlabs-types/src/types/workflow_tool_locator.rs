pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowToolLocator {
    #[serde(default)]
    pub tool_id: String,
}

impl WorkflowToolLocator {
    pub fn builder() -> WorkflowToolLocatorBuilder {
        <WorkflowToolLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowToolLocatorBuilder {
    tool_id: Option<String>,
}

impl WorkflowToolLocatorBuilder {
    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_id`](WorkflowToolLocatorBuilder::tool_id)
    pub fn build(self) -> Result<WorkflowToolLocator, BuildError> {
        Ok(WorkflowToolLocator {
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
        })
    }
}
