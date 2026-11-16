pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether a catalog item's price and duration are known up front.
/// 
/// - `fixed`: the catalog price and duration apply (the default).
/// - `quote`: neither is known until the owner has seen the job. The
/// receptionist collects intake answers and files a quote request instead
/// of booking a slot or taking an order.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PricingMode {
    Fixed,
    Quote,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PricingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::Quote => serializer.serialize_str("quote"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PricingMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fixed" => Ok(Self::Fixed),
            "quote" => Ok(Self::Quote),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PricingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixed => write!(f, "fixed"),
            Self::Quote => write!(f, "quote"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
