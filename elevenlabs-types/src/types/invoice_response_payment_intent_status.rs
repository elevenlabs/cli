pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceResponsePaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceResponsePaymentIntentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::RequiresAction => serializer.serialize_str("requires_action"),
            Self::RequiresCapture => serializer.serialize_str("requires_capture"),
            Self::RequiresConfirmation => serializer.serialize_str("requires_confirmation"),
            Self::RequiresPaymentMethod => serializer.serialize_str("requires_payment_method"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceResponsePaymentIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "canceled" => Ok(Self::Canceled),
            "processing" => Ok(Self::Processing),
            "requires_action" => Ok(Self::RequiresAction),
            "requires_capture" => Ok(Self::RequiresCapture),
            "requires_confirmation" => Ok(Self::RequiresConfirmation),
            "requires_payment_method" => Ok(Self::RequiresPaymentMethod),
            "succeeded" => Ok(Self::Succeeded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceResponsePaymentIntentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canceled => write!(f, "canceled"),
            Self::Processing => write!(f, "processing"),
            Self::RequiresAction => write!(f, "requires_action"),
            Self::RequiresCapture => write!(f, "requires_capture"),
            Self::RequiresConfirmation => write!(f, "requires_confirmation"),
            Self::RequiresPaymentMethod => write!(f, "requires_payment_method"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
