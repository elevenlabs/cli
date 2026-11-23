pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a promotion discounts: the service price or add-on prices.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PromotionTarget {
    Service,
    AddOn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PromotionTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Service => serializer.serialize_str("service"),
            Self::AddOn => serializer.serialize_str("add_on"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PromotionTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "service" => Ok(Self::Service),
            "add_on" => Ok(Self::AddOn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PromotionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Service => write!(f, "service"),
            Self::AddOn => write!(f, "add_on"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
