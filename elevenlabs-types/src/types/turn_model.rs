pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Version of the turn detection model to use.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnModel {
    TurnV2,
    TurnV3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TurnModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TurnV2 => serializer.serialize_str("turn_v2"),
            Self::TurnV3 => serializer.serialize_str("turn_v3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TurnModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "turn_v2" => Ok(Self::TurnV2),
            "turn_v3" => Ok(Self::TurnV3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TurnModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TurnV2 => write!(f, "turn_v2"),
            Self::TurnV3 => write!(f, "turn_v3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
