pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Identifiers for audio voice filters.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudioFilterId {
    Phone,
    LowQualityPhone,
    BrightPhone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudioFilterId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Phone => serializer.serialize_str("phone"),
            Self::LowQualityPhone => serializer.serialize_str("low_quality_phone"),
            Self::BrightPhone => serializer.serialize_str("bright_phone"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudioFilterId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "phone" => Ok(Self::Phone),
            "low_quality_phone" => Ok(Self::LowQualityPhone),
            "bright_phone" => Ok(Self::BrightPhone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudioFilterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Phone => write!(f, "phone"),
            Self::LowQualityPhone => write!(f, "low_quality_phone"),
            Self::BrightPhone => write!(f, "bright_phone"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
