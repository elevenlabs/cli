pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether a discovered MCP tool has an approval and whether it still matches.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpToolApprovalState {
    UpToDate,
    NeedsReview,
    NotApproved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpToolApprovalState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UpToDate => serializer.serialize_str("up_to_date"),
            Self::NeedsReview => serializer.serialize_str("needs_review"),
            Self::NotApproved => serializer.serialize_str("not_approved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpToolApprovalState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "up_to_date" => Ok(Self::UpToDate),
            "needs_review" => Ok(Self::NeedsReview),
            "not_approved" => Ok(Self::NotApproved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpToolApprovalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpToDate => write!(f, "up_to_date"),
            Self::NeedsReview => write!(f, "needs_review"),
            Self::NotApproved => write!(f, "not_approved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
