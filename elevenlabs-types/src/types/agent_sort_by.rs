pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentSortBy {
    Name,
    CreatedAt,
    CallCount7D,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Name => serializer.serialize_str("name"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::CallCount7D => serializer.serialize_str("call_count_7d"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "name" => Ok(Self::Name),
            "created_at" => Ok(Self::CreatedAt),
            "call_count_7d" => Ok(Self::CallCount7D),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::CallCount7D => write!(f, "call_count_7d"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
