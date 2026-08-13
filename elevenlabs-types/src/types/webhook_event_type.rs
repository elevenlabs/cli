pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookEventType {
    Transcript,
    Audio,
    CallInitiationFailure,
    UnredactedTranscript,
    UnredactedAudio,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transcript => serializer.serialize_str("transcript"),
            Self::Audio => serializer.serialize_str("audio"),
            Self::CallInitiationFailure => serializer.serialize_str("call_initiation_failure"),
            Self::UnredactedTranscript => serializer.serialize_str("unredacted_transcript"),
            Self::UnredactedAudio => serializer.serialize_str("unredacted_audio"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transcript" => Ok(Self::Transcript),
            "audio" => Ok(Self::Audio),
            "call_initiation_failure" => Ok(Self::CallInitiationFailure),
            "unredacted_transcript" => Ok(Self::UnredactedTranscript),
            "unredacted_audio" => Ok(Self::UnredactedAudio),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transcript => write!(f, "transcript"),
            Self::Audio => write!(f, "audio"),
            Self::CallInitiationFailure => write!(f, "call_initiation_failure"),
            Self::UnredactedTranscript => write!(f, "unredacted_transcript"),
            Self::UnredactedAudio => write!(f, "unredacted_audio"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
