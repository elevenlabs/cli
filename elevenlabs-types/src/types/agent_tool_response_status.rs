pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentToolResponseStatus {
    Success,
    Error,
    Blocked,
    Skipped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentToolResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Error => serializer.serialize_str("error"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentToolResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "error" => Ok(Self::Error),
            "blocked" => Ok(Self::Blocked),
            "skipped" => Ok(Self::Skipped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentToolResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Error => write!(f, "error"),
            Self::Blocked => write!(f, "blocked"),
            Self::Skipped => write!(f, "skipped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
