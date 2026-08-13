pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Model to use for the voice generation. Possible values: eleven_multilingual_ttv_v2, eleven_ttv_v3.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceDesignRequestModelModelId {
    ElevenMultilingualTtvV2,
    ElevenTtvV3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceDesignRequestModelModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenMultilingualTtvV2 => serializer.serialize_str("eleven_multilingual_ttv_v2"),
            Self::ElevenTtvV3 => serializer.serialize_str("eleven_ttv_v3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceDesignRequestModelModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_multilingual_ttv_v2" => Ok(Self::ElevenMultilingualTtvV2),
            "eleven_ttv_v3" => Ok(Self::ElevenTtvV3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceDesignRequestModelModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenMultilingualTtvV2 => write!(f, "eleven_multilingual_ttv_v2"),
            Self::ElevenTtvV3 => write!(f, "eleven_ttv_v3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
