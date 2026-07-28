pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StudioClipReferenceClipType {
    Video,
    Image,
    ExternalAudio,
    TtsNode,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StudioClipReferenceClipType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Video => serializer.serialize_str("video"),
            Self::Image => serializer.serialize_str("image"),
            Self::ExternalAudio => serializer.serialize_str("external_audio"),
            Self::TtsNode => serializer.serialize_str("tts_node"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StudioClipReferenceClipType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "video" => Ok(Self::Video),
            "image" => Ok(Self::Image),
            "external_audio" => Ok(Self::ExternalAudio),
            "tts_node" => Ok(Self::TtsNode),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StudioClipReferenceClipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Video => write!(f, "video"),
            Self::Image => write!(f, "image"),
            Self::ExternalAudio => write!(f, "external_audio"),
            Self::TtsNode => write!(f, "tts_node"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
