pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptPlatformEvent {
    UserTurnTimeout,
    SkipTurnWaitElapsed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranscriptPlatformEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserTurnTimeout => serializer.serialize_str("user_turn_timeout"),
            Self::SkipTurnWaitElapsed => serializer.serialize_str("skip_turn_wait_elapsed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranscriptPlatformEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user_turn_timeout" => Ok(Self::UserTurnTimeout),
            "skip_turn_wait_elapsed" => Ok(Self::SkipTurnWaitElapsed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranscriptPlatformEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserTurnTimeout => write!(f, "user_turn_timeout"),
            Self::SkipTurnWaitElapsed => write!(f, "skip_turn_wait_elapsed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
