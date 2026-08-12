pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The ID of the model to use for transcription.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechToTextConvertRequestModelId {
    ScribeV2,
    ScribeV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechToTextConvertRequestModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ScribeV2 => serializer.serialize_str("scribe_v2"),
            Self::ScribeV1 => serializer.serialize_str("scribe_v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechToTextConvertRequestModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "scribe_v2" => Ok(Self::ScribeV2),
            "scribe_v1" => Ok(Self::ScribeV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechToTextConvertRequestModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScribeV2 => write!(f, "scribe_v2"),
            Self::ScribeV1 => write!(f, "scribe_v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
