pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderItemKind {
    Dub,
    Subtitles,
    Transcription,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrderItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Dub => serializer.serialize_str("dub"),
            Self::Subtitles => serializer.serialize_str("subtitles"),
            Self::Transcription => serializer.serialize_str("transcription"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrderItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "dub" => Ok(Self::Dub),
            "subtitles" => Ok(Self::Subtitles),
            "transcription" => Ok(Self::Transcription),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrderItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dub => write!(f, "dub"),
            Self::Subtitles => write!(f, "subtitles"),
            Self::Transcription => write!(f, "transcription"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
