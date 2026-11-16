pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How an order reaches the customer.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderFulfillmentType {
    Pickup,
    Delivery,
    Digital,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrderFulfillmentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pickup => serializer.serialize_str("pickup"),
            Self::Delivery => serializer.serialize_str("delivery"),
            Self::Digital => serializer.serialize_str("digital"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrderFulfillmentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pickup" => Ok(Self::Pickup),
            "delivery" => Ok(Self::Delivery),
            "digital" => Ok(Self::Digital),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrderFulfillmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pickup => write!(f, "pickup"),
            Self::Delivery => write!(f, "delivery"),
            Self::Digital => write!(f, "digital"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
