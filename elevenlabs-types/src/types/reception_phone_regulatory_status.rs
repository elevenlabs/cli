pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Regulatory standing of a number in a country that requires paperwork.
/// 
/// Null for numbers that need no paperwork, such as US and Canadian numbers.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionPhoneRegulatoryStatus {
    Held,
    Approved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionPhoneRegulatoryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Held => serializer.serialize_str("held"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionPhoneRegulatoryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "held" => Ok(Self::Held),
            "approved" => Ok(Self::Approved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionPhoneRegulatoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Held => write!(f, "held"),
            Self::Approved => write!(f, "approved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
