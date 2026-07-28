pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolExecutionMode {
    Immediate,
    PostToolSpeech,
    Async,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolExecutionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Immediate => serializer.serialize_str("immediate"),
            Self::PostToolSpeech => serializer.serialize_str("post_tool_speech"),
            Self::Async => serializer.serialize_str("async"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolExecutionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "immediate" => Ok(Self::Immediate),
            "post_tool_speech" => Ok(Self::PostToolSpeech),
            "async" => Ok(Self::Async),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Immediate => write!(f, "immediate"),
            Self::PostToolSpeech => write!(f, "post_tool_speech"),
            Self::Async => write!(f, "async"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
