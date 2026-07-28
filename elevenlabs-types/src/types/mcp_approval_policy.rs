pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Defines the MCP server-level approval policy for tool execution.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpApprovalPolicy {
    AutoApproveAll,
    RequireApprovalAll,
    RequireApprovalPerTool,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpApprovalPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AutoApproveAll => serializer.serialize_str("auto_approve_all"),
            Self::RequireApprovalAll => serializer.serialize_str("require_approval_all"),
            Self::RequireApprovalPerTool => serializer.serialize_str("require_approval_per_tool"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpApprovalPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto_approve_all" => Ok(Self::AutoApproveAll),
            "require_approval_all" => Ok(Self::RequireApprovalAll),
            "require_approval_per_tool" => Ok(Self::RequireApprovalPerTool),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpApprovalPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AutoApproveAll => write!(f, "auto_approve_all"),
            Self::RequireApprovalAll => write!(f, "require_approval_all"),
            Self::RequireApprovalPerTool => write!(f, "require_approval_per_tool"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
