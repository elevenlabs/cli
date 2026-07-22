pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Defines the tool-level approval policy.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpToolApprovalPolicy {
    AutoApproved,
    RequiresApproval,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpToolApprovalPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AutoApproved => serializer.serialize_str("auto_approved"),
            Self::RequiresApproval => serializer.serialize_str("requires_approval"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpToolApprovalPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto_approved" => Ok(Self::AutoApproved),
            "requires_approval" => Ok(Self::RequiresApproval),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpToolApprovalPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AutoApproved => write!(f, "auto_approved"),
            Self::RequiresApproval => write!(f, "requires_approval"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
