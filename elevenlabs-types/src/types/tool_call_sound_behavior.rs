pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines how the tool call sound should be played.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolCallSoundBehavior {
    Auto,
    Always,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolCallSoundBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Always => serializer.serialize_str("always"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolCallSoundBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolCallSoundBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
