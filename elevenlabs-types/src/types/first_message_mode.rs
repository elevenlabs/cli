pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the receptionist's greeting plays.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FirstMessageMode {
    Interruptible,
    Uninterruptible,
    RecordingNotice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FirstMessageMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Interruptible => serializer.serialize_str("interruptible"),
            Self::Uninterruptible => serializer.serialize_str("uninterruptible"),
            Self::RecordingNotice => serializer.serialize_str("recording_notice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FirstMessageMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "interruptible" => Ok(Self::Interruptible),
            "uninterruptible" => Ok(Self::Uninterruptible),
            "recording_notice" => Ok(Self::RecordingNotice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FirstMessageMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interruptible => write!(f, "interruptible"),
            Self::Uninterruptible => write!(f, "uninterruptible"),
            Self::RecordingNotice => write!(f, "recording_notice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
