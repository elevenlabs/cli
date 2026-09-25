pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for testing tools available on an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMcpToolsResponseModel {
    /// Indicates if the operation was successful.
    #[serde(default)]
    pub success: bool,
    /// A list of tools available on the MCP server.
    #[serde(default)]
    pub tools: Vec<Tool>,
    /// Derived approval states for currently discovered tools. Populated only for persisted MCP servers using per-tool approval; otherwise empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_approval_statuses: Option<Vec<McpToolApprovalStatus>>,
    /// Error message if the operation was not successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ListMcpToolsResponseModel {
    pub fn builder() -> ListMcpToolsResponseModelBuilder {
        <ListMcpToolsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMcpToolsResponseModelBuilder {
    success: Option<bool>,
    tools: Option<Vec<Tool>>,
    tool_approval_statuses: Option<Vec<McpToolApprovalStatus>>,
    error_message: Option<String>,
}

impl ListMcpToolsResponseModelBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<Tool>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_approval_statuses(mut self, value: Vec<McpToolApprovalStatus>) -> Self {
        self.tool_approval_statuses = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMcpToolsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ListMcpToolsResponseModelBuilder::success)
    /// - [`tools`](ListMcpToolsResponseModelBuilder::tools)
    pub fn build(self) -> Result<ListMcpToolsResponseModel, BuildError> {
        Ok(ListMcpToolsResponseModel {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            tool_approval_statuses: self.tool_approval_statuses,
            error_message: self.error_message,
        })
    }
}
