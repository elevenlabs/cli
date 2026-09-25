pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Derived approval state for a currently discovered MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpToolApprovalStatus {
    /// Canonical MCP tool identifier in the form mcp:<server_id>:<tool_name>
    #[serde(default)]
    pub tool_id: String,
    /// Whether a stored approval exists and still matches the live tool definition
    pub state: McpToolApprovalState,
    /// Stored execution policy. Set when the tool has an approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpToolApprovalPolicy>,
    /// Previously approved definition, included when the tool needs review and a snapshot exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_definition: Option<McpApprovedToolDefinition>,
}

impl McpToolApprovalStatus {
    pub fn builder() -> McpToolApprovalStatusBuilder {
        <McpToolApprovalStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolApprovalStatusBuilder {
    tool_id: Option<String>,
    state: Option<McpToolApprovalState>,
    approval_policy: Option<McpToolApprovalPolicy>,
    approved_definition: Option<McpApprovedToolDefinition>,
}

impl McpToolApprovalStatusBuilder {
    pub fn tool_id(mut self, value: impl Into<String>) -> Self {
        self.tool_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: McpToolApprovalState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn approval_policy(mut self, value: McpToolApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    pub fn approved_definition(mut self, value: McpApprovedToolDefinition) -> Self {
        self.approved_definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolApprovalStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_id`](McpToolApprovalStatusBuilder::tool_id)
    /// - [`state`](McpToolApprovalStatusBuilder::state)
    pub fn build(self) -> Result<McpToolApprovalStatus, BuildError> {
        Ok(McpToolApprovalStatus {
            tool_id: self.tool_id.ok_or_else(|| BuildError::missing_field("tool_id"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            approval_policy: self.approval_policy,
            approved_definition: self.approved_definition,
        })
    }
}
