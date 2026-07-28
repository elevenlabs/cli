pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SingleUseTokenType {
    RealtimeScribe,
    BatchScribe,
    TtsWebsocket,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SingleUseTokenType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RealtimeScribe => serializer.serialize_str("realtime_scribe"),
            Self::BatchScribe => serializer.serialize_str("batch_scribe"),
            Self::TtsWebsocket => serializer.serialize_str("tts_websocket"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SingleUseTokenType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "realtime_scribe" => Ok(Self::RealtimeScribe),
            "batch_scribe" => Ok(Self::BatchScribe),
            "tts_websocket" => Ok(Self::TtsWebsocket),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SingleUseTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RealtimeScribe => write!(f, "realtime_scribe"),
            Self::BatchScribe => write!(f, "batch_scribe"),
            Self::TtsWebsocket => write!(f, "tts_websocket"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
