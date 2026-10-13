pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether reaching an End node completes successfully or fails.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowEndNodeOutcome {
    Success,
    Failure,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowEndNodeOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Failure => serializer.serialize_str("failure"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowEndNodeOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "failure" => Ok(Self::Failure),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowEndNodeOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure => write!(f, "failure"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
