pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictResolutionChoice {
    Source,
    Target,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConflictResolutionChoice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Source => serializer.serialize_str("source"),
            Self::Target => serializer.serialize_str("target"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConflictResolutionChoice {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "source" => Ok(Self::Source),
            "target" => Ok(Self::Target),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConflictResolutionChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source => write!(f, "source"),
            Self::Target => write!(f, "target"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
