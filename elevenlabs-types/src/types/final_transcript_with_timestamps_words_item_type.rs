pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of the word or sound. 'audio_event' is used for non-word sounds like laughter or footsteps.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinalTranscriptWithTimestampsWordsItemType {
    Word,
    Spacing,
    AudioEvent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinalTranscriptWithTimestampsWordsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Word => serializer.serialize_str("word"),
            Self::Spacing => serializer.serialize_str("spacing"),
            Self::AudioEvent => serializer.serialize_str("audio_event"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinalTranscriptWithTimestampsWordsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "word" => Ok(Self::Word),
            "spacing" => Ok(Self::Spacing),
            "audio_event" => Ok(Self::AudioEvent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinalTranscriptWithTimestampsWordsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word => write!(f, "word"),
            Self::Spacing => write!(f, "spacing"),
            Self::AudioEvent => write!(f, "audio_event"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
