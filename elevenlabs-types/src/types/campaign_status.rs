pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignStatus {
    Active,
    Paused,
    Stopping,
    Stopped,
    Completed,
    Failed,
    Attention,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Stopping => serializer.serialize_str("stopping"),
            Self::Stopped => serializer.serialize_str("stopped"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Attention => serializer.serialize_str("attention"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "paused" => Ok(Self::Paused),
            "stopping" => Ok(Self::Stopping),
            "stopped" => Ok(Self::Stopped),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "attention" => Ok(Self::Attention),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Stopping => write!(f, "stopping"),
            Self::Stopped => write!(f, "stopped"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Attention => write!(f, "attention"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
