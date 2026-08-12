pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowToolLocator {
    #[serde(default)]
    pub tool_id: String,
    /// Per-node parameter overrides applied on top of the tool's own configuration. Keys are dotted parameter paths (webhook tools prefix keys with path_params./query_params./request_body.). These take precedence over any overrides already defined on the tool itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_overrides: Option<HashMap<String, Option<WorkflowToolLocatorSchemaOverridesValue>>>,
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
    schema_overrides: Option<HashMap<String, Option<WorkflowToolLocatorSchemaOverridesValue>>>,
}

impl WorkflowToolLocatorBuilder {
    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    pub fn schema_overrides(mut self, value: HashMap<String, Option<WorkflowToolLocatorSchemaOverridesValue>>) -> Self {
        self.schema_overrides = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowToolLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_id`](WorkflowToolLocatorBuilder::tool_id)
    pub fn build(self) -> Result<WorkflowToolLocator, BuildError> {
        Ok(WorkflowToolLocator {
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
            schema_overrides: self.schema_overrides,
        })
    }
}
