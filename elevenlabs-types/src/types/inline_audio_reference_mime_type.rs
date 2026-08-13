pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The MIME type of the encoded audio.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InlineAudioReferenceMimeType {
    AudioMpeg,
    AudioWav,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InlineAudioReferenceMimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AudioMpeg => serializer.serialize_str("audio/mpeg"),
            Self::AudioWav => serializer.serialize_str("audio/wav"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InlineAudioReferenceMimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "audio/mpeg" => Ok(Self::AudioMpeg),
            "audio/wav" => Ok(Self::AudioWav),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InlineAudioReferenceMimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioMpeg => write!(f, "audio/mpeg"),
            Self::AudioWav => write!(f, "audio/wav"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
