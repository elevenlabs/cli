pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttentionStatus {
    Open,
    Resolved,
    Retrying,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AttentionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Resolved => serializer.serialize_str("resolved"),
            Self::Retrying => serializer.serialize_str("retrying"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AttentionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "resolved" => Ok(Self::Resolved),
            "retrying" => Ok(Self::Retrying),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AttentionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Resolved => write!(f, "resolved"),
            Self::Retrying => write!(f, "retrying"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
