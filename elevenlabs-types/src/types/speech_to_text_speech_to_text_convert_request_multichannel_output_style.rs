pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Controls the response shape when use_multi_channel is enabled. 'separate' (default) returns one transcript per channel under 'transcripts'. 'combined' merges all channels into a single transcript whose words are sorted by start time, each carrying a 'channel_index' - matching the single-channel response shape. 'combined' requires timestamps (timestamps_granularity must not be 'none') and does not support entity detection or redaction.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechToTextConvertRequestMultichannelOutputStyle {
    Separate,
    Combined,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechToTextConvertRequestMultichannelOutputStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Separate => serializer.serialize_str("separate"),
            Self::Combined => serializer.serialize_str("combined"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechToTextConvertRequestMultichannelOutputStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "separate" => Ok(Self::Separate),
            "combined" => Ok(Self::Combined),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechToTextConvertRequestMultichannelOutputStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Separate => write!(f, "separate"),
            Self::Combined => write!(f, "combined"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
