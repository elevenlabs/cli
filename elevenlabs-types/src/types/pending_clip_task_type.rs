pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PendingClipTaskType {
    Preprocessing,
    SpeechImport,
    Dubbing,
    VideoToMusic,
    MediaGeneration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PendingClipTaskType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Preprocessing => serializer.serialize_str("preprocessing"),
            Self::SpeechImport => serializer.serialize_str("speech_import"),
            Self::Dubbing => serializer.serialize_str("dubbing"),
            Self::VideoToMusic => serializer.serialize_str("video_to_music"),
            Self::MediaGeneration => serializer.serialize_str("media_generation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PendingClipTaskType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "preprocessing" => Ok(Self::Preprocessing),
            "speech_import" => Ok(Self::SpeechImport),
            "dubbing" => Ok(Self::Dubbing),
            "video_to_music" => Ok(Self::VideoToMusic),
            "media_generation" => Ok(Self::MediaGeneration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PendingClipTaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preprocessing => write!(f, "preprocessing"),
            Self::SpeechImport => write!(f, "speech_import"),
            Self::Dubbing => write!(f, "dubbing"),
            Self::VideoToMusic => write!(f, "video_to_music"),
            Self::MediaGeneration => write!(f, "media_generation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
