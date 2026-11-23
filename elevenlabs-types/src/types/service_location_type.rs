pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where a classic service is delivered.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceLocationType {
    Business,
    Customer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ServiceLocationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Business => serializer.serialize_str("business"),
            Self::Customer => serializer.serialize_str("customer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ServiceLocationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "business" => Ok(Self::Business),
            "customer" => Ok(Self::Customer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ServiceLocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Business => write!(f, "business"),
            Self::Customer => write!(f, "customer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
