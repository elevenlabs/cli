pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BanReasonType {
    Safety,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BanReasonType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Safety => serializer.serialize_str("safety"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BanReasonType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "safety" => Ok(Self::Safety),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BanReasonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safety => write!(f, "safety"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
