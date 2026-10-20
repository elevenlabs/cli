pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ``BURSTY`` clients stream ahead of wall time, so decisions use the audio clock.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioInputPacing {
    Realtime,
    Bursty,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudioInputPacing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Realtime => serializer.serialize_str("realtime"),
            Self::Bursty => serializer.serialize_str("bursty"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudioInputPacing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "realtime" => Ok(Self::Realtime),
            "bursty" => Ok(Self::Bursty),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudioInputPacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Realtime => write!(f, "realtime"),
            Self::Bursty => write!(f, "bursty"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
