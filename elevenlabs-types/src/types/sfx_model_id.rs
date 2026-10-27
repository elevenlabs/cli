pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SfxModelId {
    ElevenTextToSoundV2,
    ElevenTextToSoundV3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SfxModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenTextToSoundV2 => serializer.serialize_str("eleven_text_to_sound_v2"),
            Self::ElevenTextToSoundV3 => serializer.serialize_str("eleven_text_to_sound_v3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SfxModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_text_to_sound_v2" => Ok(Self::ElevenTextToSoundV2),
            "eleven_text_to_sound_v3" => Ok(Self::ElevenTextToSoundV3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SfxModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenTextToSoundV2 => write!(f, "eleven_text_to_sound_v2"),
            Self::ElevenTextToSoundV3 => write!(f, "eleven_text_to_sound_v3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
