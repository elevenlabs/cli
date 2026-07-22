pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsrProvider {
    /// Deprecated: Use scribe_realtime instead.
    Elevenlabs,
    ScribeRealtime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AsrProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Elevenlabs => serializer.serialize_str("elevenlabs"),
            Self::ScribeRealtime => serializer.serialize_str("scribe_realtime"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AsrProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "elevenlabs" => Ok(Self::Elevenlabs),
            "scribe_realtime" => Ok(Self::ScribeRealtime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AsrProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Elevenlabs => write!(f, "elevenlabs"),
            Self::ScribeRealtime => write!(f, "scribe_realtime"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
