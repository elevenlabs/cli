pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallResult {
    Answered,
    Voicemail,
    Busy,
    NoAnswer,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CallResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Answered => serializer.serialize_str("answered"),
            Self::Voicemail => serializer.serialize_str("voicemail"),
            Self::Busy => serializer.serialize_str("busy"),
            Self::NoAnswer => serializer.serialize_str("no_answer"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CallResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "answered" => Ok(Self::Answered),
            "voicemail" => Ok(Self::Voicemail),
            "busy" => Ok(Self::Busy),
            "no_answer" => Ok(Self::NoAnswer),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CallResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Answered => write!(f, "answered"),
            Self::Voicemail => write!(f, "voicemail"),
            Self::Busy => write!(f, "busy"),
            Self::NoAnswer => write!(f, "no_answer"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
