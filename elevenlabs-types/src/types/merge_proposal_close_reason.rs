pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MergeProposalCloseReason {
    Withdrawn,
    Rejected,
    BranchArchived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MergeProposalCloseReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Withdrawn => serializer.serialize_str("withdrawn"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::BranchArchived => serializer.serialize_str("branch_archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MergeProposalCloseReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "withdrawn" => Ok(Self::Withdrawn),
            "rejected" => Ok(Self::Rejected),
            "branch_archived" => Ok(Self::BranchArchived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MergeProposalCloseReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::Rejected => write!(f, "rejected"),
            Self::BranchArchived => write!(f, "branch_archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
