pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// `open` while the card is still being retried; `void` once retries ran out and the workspace's plan was ended.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResellerInvoiceStatus {
    Paid,
    Open,
    Void,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResellerInvoiceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Paid => serializer.serialize_str("paid"),
            Self::Open => serializer.serialize_str("open"),
            Self::Void => serializer.serialize_str("void"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResellerInvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "paid" => Ok(Self::Paid),
            "open" => Ok(Self::Open),
            "void" => Ok(Self::Void),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResellerInvoiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Paid => write!(f, "paid"),
            Self::Open => write!(f, "open"),
            Self::Void => write!(f, "void"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
