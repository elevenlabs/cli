pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether to wait for the reply, or keep going and get it later in the conversation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubAgentOutputExecutionMode {
    Immediate,
    Async,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubAgentOutputExecutionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Immediate => serializer.serialize_str("immediate"),
            Self::Async => serializer.serialize_str("async"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubAgentOutputExecutionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "immediate" => Ok(Self::Immediate),
            "async" => Ok(Self::Async),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubAgentOutputExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immediate => write!(f, "immediate"),
            Self::Async => write!(f, "async"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
