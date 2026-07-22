pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpToolAddApprovalRequestModel {
    /// The name of the MCP tool
    #[serde(default)]
    pub tool_name: String,
    /// The description of the MCP tool
    #[serde(default)]
    pub tool_description: String,
    /// The input schema of the MCP tool (the schema defined on the MCP server before ElevenLabs does any extra processing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<HashMap<String, serde_json::Value>>,
    /// The tool-level approval policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpToolApprovalPolicy>,
}

impl McpToolAddApprovalRequestModel {
    pub fn builder() -> McpToolAddApprovalRequestModelBuilder {
        <McpToolAddApprovalRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolAddApprovalRequestModelBuilder {
    tool_name: Option<String>,
    tool_description: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    approval_policy: Option<McpToolApprovalPolicy>,
}

impl McpToolAddApprovalRequestModelBuilder {
    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_description(mut self, value: impl Into<String>) -> Self {
        self.tool_description = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn approval_policy(mut self, value: McpToolApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolAddApprovalRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](McpToolAddApprovalRequestModelBuilder::tool_name)
    /// - [`tool_description`](McpToolAddApprovalRequestModelBuilder::tool_description)
    pub fn build(self) -> Result<McpToolAddApprovalRequestModel, BuildError> {
        Ok(McpToolAddApprovalRequestModel {
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_description: self.tool_description.ok_or_else(|| BuildError::missing_field("tool_description"))?,
            input_schema: self.input_schema,
            approval_policy: self.approval_policy,
        })
    }
}

