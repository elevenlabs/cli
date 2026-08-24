pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TtsConversationalModel {
    /// Deprecated: Use eleven_flash_v2 instead.
    ElevenTurboV2,
    /// Deprecated: Use eleven_flash_v2_5 instead.
    ElevenTurboV25,
    ElevenFlashV2,
    ElevenFlashV25,
    ElevenMultilingualV2,
    ElevenV3Conversational,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TtsConversationalModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenTurboV2 => serializer.serialize_str("eleven_turbo_v2"),
            Self::ElevenTurboV25 => serializer.serialize_str("eleven_turbo_v2_5"),
            Self::ElevenFlashV2 => serializer.serialize_str("eleven_flash_v2"),
            Self::ElevenFlashV25 => serializer.serialize_str("eleven_flash_v2_5"),
            Self::ElevenMultilingualV2 => serializer.serialize_str("eleven_multilingual_v2"),
            Self::ElevenV3Conversational => serializer.serialize_str("eleven_v3_conversational"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TtsConversationalModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_turbo_v2" => Ok(Self::ElevenTurboV2),
            "eleven_turbo_v2_5" => Ok(Self::ElevenTurboV25),
            "eleven_flash_v2" => Ok(Self::ElevenFlashV2),
            "eleven_flash_v2_5" => Ok(Self::ElevenFlashV25),
            "eleven_multilingual_v2" => Ok(Self::ElevenMultilingualV2),
            "eleven_v3_conversational" => Ok(Self::ElevenV3Conversational),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TtsConversationalModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenTurboV2 => write!(f, "eleven_turbo_v2"),
            Self::ElevenTurboV25 => write!(f, "eleven_turbo_v2_5"),
            Self::ElevenFlashV2 => write!(f, "eleven_flash_v2"),
            Self::ElevenFlashV25 => write!(f, "eleven_flash_v2_5"),
            Self::ElevenMultilingualV2 => write!(f, "eleven_multilingual_v2"),
            Self::ElevenV3Conversational => write!(f, "eleven_v3_conversational"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
