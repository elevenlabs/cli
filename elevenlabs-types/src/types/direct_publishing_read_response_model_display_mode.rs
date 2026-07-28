pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectPublishingReadResponseModelDisplayMode {
    Text,
    AudioOnly,
    TextWithAudio,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DirectPublishingReadResponseModelDisplayMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::AudioOnly => serializer.serialize_str("audio-only"),
            Self::TextWithAudio => serializer.serialize_str("text-with-audio"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DirectPublishingReadResponseModelDisplayMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "audio-only" => Ok(Self::AudioOnly),
            "text-with-audio" => Ok(Self::TextWithAudio),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DirectPublishingReadResponseModelDisplayMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::AudioOnly => write!(f, "audio-only"),
            Self::TextWithAudio => write!(f, "text-with-audio"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
