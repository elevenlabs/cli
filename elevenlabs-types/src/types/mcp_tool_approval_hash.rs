pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model for storing tool approval hashes for per-tool approval.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpToolApprovalHash {
    /// The name of the MCP tool
    #[serde(default)]
    pub tool_name: String,
    /// SHA256 hash of the tool's parameters and description
    #[serde(default)]
    pub tool_hash: String,
    /// The approval policy for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy: Option<McpToolApprovalPolicy>,
}

impl McpToolApprovalHash {
    pub fn builder() -> McpToolApprovalHashBuilder {
        <McpToolApprovalHashBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpToolApprovalHashBuilder {
    tool_name: Option<String>,
    tool_hash: Option<String>,
    approval_policy: Option<McpToolApprovalPolicy>,
}

impl McpToolApprovalHashBuilder {
    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn tool_hash(mut self, value: impl Into<String>) -> Self {
        self.tool_hash = Some(value.into());
        self
    }

    pub fn approval_policy(mut self, value: McpToolApprovalPolicy) -> Self {
        self.approval_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpToolApprovalHash`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](McpToolApprovalHashBuilder::tool_name)
    /// - [`tool_hash`](McpToolApprovalHashBuilder::tool_hash)
    pub fn build(self) -> Result<McpToolApprovalHash, BuildError> {
        Ok(McpToolApprovalHash {
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            tool_hash: self.tool_hash.ok_or_else(|| BuildError::missing_field("tool_hash"))?,
            approval_policy: self.approval_policy,
        })
    }
}
