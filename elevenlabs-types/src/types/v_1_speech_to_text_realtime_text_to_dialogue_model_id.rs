pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The ID of the model to use for speech-to-text transcription.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextToDialogueModelId {
    ScribeV2Realtime,
    ScribeV3Live,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextToDialogueModelId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ScribeV2Realtime => serializer.serialize_str("scribe_v2_realtime"),
            Self::ScribeV3Live => serializer.serialize_str("scribe_v3_live"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextToDialogueModelId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "scribe_v2_realtime" => Ok(Self::ScribeV2Realtime),
            "scribe_v3_live" => Ok(Self::ScribeV3Live),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextToDialogueModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScribeV2Realtime => write!(f, "scribe_v2_realtime"),
            Self::ScribeV3Live => write!(f, "scribe_v3_live"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
