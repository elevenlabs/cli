pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RealtimeModel {
    ElevenRealtimeV1Mini,
    ElevenRealtimeV1Max,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RealtimeModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ElevenRealtimeV1Mini => serializer.serialize_str("eleven_realtime_v1_mini"),
            Self::ElevenRealtimeV1Max => serializer.serialize_str("eleven_realtime_v1_max"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RealtimeModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eleven_realtime_v1_mini" => Ok(Self::ElevenRealtimeV1Mini),
            "eleven_realtime_v1_max" => Ok(Self::ElevenRealtimeV1Max),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RealtimeModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElevenRealtimeV1Mini => write!(f, "eleven_realtime_v1_mini"),
            Self::ElevenRealtimeV1Max => write!(f, "eleven_realtime_v1_max"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
