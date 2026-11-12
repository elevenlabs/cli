pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutoRebaseOutcome {
    Succeeded,
    Conflict,
    NoCommonAncestor,
    Rejected,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutoRebaseOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Conflict => serializer.serialize_str("conflict"),
            Self::NoCommonAncestor => serializer.serialize_str("no_common_ancestor"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutoRebaseOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "succeeded" => Ok(Self::Succeeded),
            "conflict" => Ok(Self::Conflict),
            "no_common_ancestor" => Ok(Self::NoCommonAncestor),
            "rejected" => Ok(Self::Rejected),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutoRebaseOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded => write!(f, "succeeded"),
            Self::Conflict => write!(f, "conflict"),
            Self::NoCommonAncestor => write!(f, "no_common_ancestor"),
            Self::Rejected => write!(f, "rejected"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
