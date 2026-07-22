pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The id of the stem variation to use.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MusicSeparateStemsRequestStemVariationId {
    TwoStemsV1,
    SixStemsV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MusicSeparateStemsRequestStemVariationId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TwoStemsV1 => serializer.serialize_str("two_stems_v1"),
            Self::SixStemsV1 => serializer.serialize_str("six_stems_v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MusicSeparateStemsRequestStemVariationId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "two_stems_v1" => Ok(Self::TwoStemsV1),
            "six_stems_v1" => Ok(Self::SixStemsV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MusicSeparateStemsRequestStemVariationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoStemsV1 => write!(f, "two_stems_v1"),
            Self::SixStemsV1 => write!(f, "six_stems_v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
