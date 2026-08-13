pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Method for converting numbers to words before sending to TTS
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextNormalisationType {
    SystemPrompt,
    Elevenlabs,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextNormalisationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SystemPrompt => serializer.serialize_str("system_prompt"),
            Self::Elevenlabs => serializer.serialize_str("elevenlabs"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextNormalisationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "system_prompt" => Ok(Self::SystemPrompt),
            "elevenlabs" => Ok(Self::Elevenlabs),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextNormalisationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SystemPrompt => write!(f, "system_prompt"),
            Self::Elevenlabs => write!(f, "elevenlabs"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
