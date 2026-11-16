pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a time-off entry blocks: a staff member, an asset, or a location.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimeOffType {
    Staff,
    Asset,
    Location,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimeOffType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Staff => serializer.serialize_str("staff"),
            Self::Asset => serializer.serialize_str("asset"),
            Self::Location => serializer.serialize_str("location"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimeOffType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "staff" => Ok(Self::Staff),
            "asset" => Ok(Self::Asset),
            "location" => Ok(Self::Location),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimeOffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Staff => write!(f, "staff"),
            Self::Asset => write!(f, "asset"),
            Self::Location => write!(f, "location"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
