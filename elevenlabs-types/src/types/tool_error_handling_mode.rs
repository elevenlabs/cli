pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Controls how tool errors are processed before being shared with the agent.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolErrorHandlingMode {
    Auto,
    Summarized,
    Passthrough,
    Hide,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolErrorHandlingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Summarized => serializer.serialize_str("summarized"),
            Self::Passthrough => serializer.serialize_str("passthrough"),
            Self::Hide => serializer.serialize_str("hide"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolErrorHandlingMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "summarized" => Ok(Self::Summarized),
            "passthrough" => Ok(Self::Passthrough),
            "hide" => Ok(Self::Hide),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolErrorHandlingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Summarized => write!(f, "summarized"),
            Self::Passthrough => write!(f, "passthrough"),
            Self::Hide => write!(f, "hide"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
