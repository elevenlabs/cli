pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// TTS model used to synthesize the translated speech.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextToDialogueTtsModelId {
    ElevenFlashV25,
    ElevenV3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextToDialogueTtsModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenFlashV25 => serializer.serialize_str("eleven_flash_v2_5"),
            Self::ElevenV3 => serializer.serialize_str("eleven_v3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextToDialogueTtsModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_flash_v2_5" => Ok(Self::ElevenFlashV25),
            "eleven_v3" => Ok(Self::ElevenV3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextToDialogueTtsModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenFlashV25 => write!(f, "eleven_flash_v2_5"),
            Self::ElevenV3 => write!(f, "eleven_v3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
