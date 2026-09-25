pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Snapshot of the MCP tool definition that was approved.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpApprovedToolDefinition {
    /// The MCP server-provided tool description at approval time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The MCP server-provided JSON input schema at approval time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<HashMap<String, serde_json::Value>>,
}

impl McpApprovedToolDefinition {
    pub fn builder() -> McpApprovedToolDefinitionBuilder {
        <McpApprovedToolDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpApprovedToolDefinitionBuilder {
    description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
}

impl McpApprovedToolDefinitionBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpApprovedToolDefinition`].
    pub fn build(self) -> Result<McpApprovedToolDefinition, BuildError> {
        Ok(McpApprovedToolDefinition {
            description: self.description,
            input_schema: self.input_schema,
        })
    }
}
