pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Format to return transcript in. For subtitles use either 'srt' or 'webvtt', and for a full transcript use 'json'. The 'json' format is not yet supported for Dubbing Studio.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptGetTranscriptForDubRequestFormatType {
    Srt,
    Webvtt,
    Json,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranscriptGetTranscriptForDubRequestFormatType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Srt => serializer.serialize_str("srt"),
            Self::Webvtt => serializer.serialize_str("webvtt"),
            Self::Json => serializer.serialize_str("json"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranscriptGetTranscriptForDubRequestFormatType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "srt" => Ok(Self::Srt),
            "webvtt" => Ok(Self::Webvtt),
            "json" => Ok(Self::Json),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranscriptGetTranscriptForDubRequestFormatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Srt => write!(f, "srt"),
            Self::Webvtt => write!(f, "webvtt"),
            Self::Json => write!(f, "json"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
