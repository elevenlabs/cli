pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Voice category used for filtering
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicesGetSharedRequestCategory {
    Professional,
    Famous,
    HighQuality,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoicesGetSharedRequestCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Professional => serializer.serialize_str("professional"),
            Self::Famous => serializer.serialize_str("famous"),
            Self::HighQuality => serializer.serialize_str("high_quality"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoicesGetSharedRequestCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "professional" => Ok(Self::Professional),
            "famous" => Ok(Self::Famous),
            "high_quality" => Ok(Self::HighQuality),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoicesGetSharedRequestCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Professional => write!(f, "professional"),
            Self::Famous => write!(f, "famous"),
            Self::HighQuality => write!(f, "high_quality"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
