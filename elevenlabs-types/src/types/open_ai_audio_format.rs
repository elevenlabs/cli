pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenAiAudioFormat {
    AudioPcm,
    AudioPcmu,
    AudioPcma,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OpenAiAudioFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AudioPcm => serializer.serialize_str("audio/pcm"),
            Self::AudioPcmu => serializer.serialize_str("audio/pcmu"),
            Self::AudioPcma => serializer.serialize_str("audio/pcma"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OpenAiAudioFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "audio/pcm" => Ok(Self::AudioPcm),
            "audio/pcmu" => Ok(Self::AudioPcmu),
            "audio/pcma" => Ok(Self::AudioPcma),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OpenAiAudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioPcm => write!(f, "audio/pcm"),
            Self::AudioPcmu => write!(f, "audio/pcmu"),
            Self::AudioPcma => write!(f, "audio/pcma"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
