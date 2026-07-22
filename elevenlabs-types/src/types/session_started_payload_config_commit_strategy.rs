pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Strategy for committing transcriptions.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionStartedPayloadConfigCommitStrategy {
    Manual,
    Vad,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SessionStartedPayloadConfigCommitStrategy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Vad => serializer.serialize_str("vad"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SessionStartedPayloadConfigCommitStrategy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "vad" => Ok(Self::Vad),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SessionStartedPayloadConfigCommitStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Vad => write!(f, "vad"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
