pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestRunMetadataTestType {
    Llm,
    ToolCall,
    Simulation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TestRunMetadataTestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Llm => serializer.serialize_str("llm"),
            Self::ToolCall => serializer.serialize_str("tool_call"),
            Self::Simulation => serializer.serialize_str("simulation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TestRunMetadataTestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "llm" => Ok(Self::Llm),
            "tool_call" => Ok(Self::ToolCall),
            "simulation" => Ok(Self::Simulation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TestRunMetadataTestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Llm => write!(f, "llm"),
            Self::ToolCall => write!(f, "tool_call"),
            Self::Simulation => write!(f, "simulation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
