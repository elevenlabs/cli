pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallStatus {
    Queued,
    Submitted,
    Completed,
    Cancelled,
    Attention,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CallStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Attention => serializer.serialize_str("attention"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CallStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "submitted" => Ok(Self::Submitted),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "attention" => Ok(Self::Attention),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CallStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Submitted => write!(f, "submitted"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Attention => write!(f, "attention"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
