pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolInterruptionMode {
    Allow,
    DisableDuringTool,
    DisableDuringToolAndTurn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolInterruptionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allow => serializer.serialize_str("allow"),
            Self::DisableDuringTool => serializer.serialize_str("disable_during_tool"),
            Self::DisableDuringToolAndTurn => serializer.serialize_str("disable_during_tool_and_turn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolInterruptionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allow" => Ok(Self::Allow),
            "disable_during_tool" => Ok(Self::DisableDuringTool),
            "disable_during_tool_and_turn" => Ok(Self::DisableDuringToolAndTurn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolInterruptionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::DisableDuringTool => write!(f, "disable_during_tool"),
            Self::DisableDuringToolAndTurn => write!(f, "disable_during_tool_and_turn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
