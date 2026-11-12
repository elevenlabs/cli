pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The furthest the message got, or `failed` if sending it failed.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LeadDeliveryViewStage {
    Queued,
    Sent,
    Delivered,
    Read,
    Replied,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LeadDeliveryViewStage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Read => serializer.serialize_str("read"),
            Self::Replied => serializer.serialize_str("replied"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LeadDeliveryViewStage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "sent" => Ok(Self::Sent),
            "delivered" => Ok(Self::Delivered),
            "read" => Ok(Self::Read),
            "replied" => Ok(Self::Replied),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LeadDeliveryViewStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Sent => write!(f, "sent"),
            Self::Delivered => write!(f, "delivered"),
            Self::Read => write!(f, "read"),
            Self::Replied => write!(f, "replied"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
