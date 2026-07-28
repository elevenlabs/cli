pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchCallRecipientStatus {
    Pending,
    Dispatched,
    Initiated,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Voicemail,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BatchCallRecipientStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Dispatched => serializer.serialize_str("dispatched"),
            Self::Initiated => serializer.serialize_str("initiated"),
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Voicemail => serializer.serialize_str("voicemail"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BatchCallRecipientStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "dispatched" => Ok(Self::Dispatched),
            "initiated" => Ok(Self::Initiated),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "voicemail" => Ok(Self::Voicemail),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BatchCallRecipientStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Dispatched => write!(f, "dispatched"),
            Self::Initiated => write!(f, "initiated"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Voicemail => write!(f, "voicemail"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
