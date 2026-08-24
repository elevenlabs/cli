pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The granularity of the timestamps in the transcription. 'word' provides word-level timestamps and 'character' provides character-level timestamps per word.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechToTextConvertRequestTimestampsGranularity {
    None,
    Word,
    Character,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechToTextConvertRequestTimestampsGranularity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Word => serializer.serialize_str("word"),
            Self::Character => serializer.serialize_str("character"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechToTextConvertRequestTimestampsGranularity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "word" => Ok(Self::Word),
            "character" => Ok(Self::Character),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechToTextConvertRequestTimestampsGranularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Word => write!(f, "word"),
            Self::Character => write!(f, "character"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
