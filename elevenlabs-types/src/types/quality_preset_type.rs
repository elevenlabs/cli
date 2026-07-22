pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QualityPresetType {
    Standard,
    High,
    Ultra,
    UltraLossless,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QualityPresetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::High => serializer.serialize_str("high"),
            Self::Ultra => serializer.serialize_str("ultra"),
            Self::UltraLossless => serializer.serialize_str("ultra_lossless"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QualityPresetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "high" => Ok(Self::High),
            "ultra" => Ok(Self::Ultra),
            "ultra_lossless" => Ok(Self::UltraLossless),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QualityPresetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::High => write!(f, "high"),
            Self::Ultra => write!(f, "ultra"),
            Self::UltraLossless => write!(f, "ultra_lossless"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
