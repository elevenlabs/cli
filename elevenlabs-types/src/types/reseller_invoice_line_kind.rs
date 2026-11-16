pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResellerInvoiceLineKind {
    Plan,
    Telephony,
    Overage,
    Proration,
    Adjustment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResellerInvoiceLineKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Plan => serializer.serialize_str("plan"),
            Self::Telephony => serializer.serialize_str("telephony"),
            Self::Overage => serializer.serialize_str("overage"),
            Self::Proration => serializer.serialize_str("proration"),
            Self::Adjustment => serializer.serialize_str("adjustment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResellerInvoiceLineKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "plan" => Ok(Self::Plan),
            "telephony" => Ok(Self::Telephony),
            "overage" => Ok(Self::Overage),
            "proration" => Ok(Self::Proration),
            "adjustment" => Ok(Self::Adjustment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResellerInvoiceLineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plan => write!(f, "plan"),
            Self::Telephony => write!(f, "telephony"),
            Self::Overage => write!(f, "overage"),
            Self::Proration => write!(f, "proration"),
            Self::Adjustment => write!(f, "adjustment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
