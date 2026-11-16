pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentOutputStatus {
    InProgress,
    Completed,
    RequiresAction,
    Failed,
    TimedOut,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentOutputStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::RequiresAction => serializer.serialize_str("requires_action"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::TimedOut => serializer.serialize_str("timed_out"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentOutputStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "requires_action" => Ok(Self::RequiresAction),
            "failed" => Ok(Self::Failed),
            "timed_out" => Ok(Self::TimedOut),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentOutputStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::RequiresAction => write!(f, "requires_action"),
            Self::Failed => write!(f, "failed"),
            Self::TimedOut => write!(f, "timed_out"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
