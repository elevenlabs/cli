pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderState {
    Open,
    Submitted,
    Paid,
    Accepted,
    Rejected,
    Done,
    Cancelling,
    Cancelled,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrderState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Done => serializer.serialize_str("done"),
            Self::Cancelling => serializer.serialize_str("cancelling"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrderState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "submitted" => Ok(Self::Submitted),
            "paid" => Ok(Self::Paid),
            "accepted" => Ok(Self::Accepted),
            "rejected" => Ok(Self::Rejected),
            "done" => Ok(Self::Done),
            "cancelling" => Ok(Self::Cancelling),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrderState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Submitted => write!(f, "submitted"),
            Self::Paid => write!(f, "paid"),
            Self::Accepted => write!(f, "accepted"),
            Self::Rejected => write!(f, "rejected"),
            Self::Done => write!(f, "done"),
            Self::Cancelling => write!(f, "cancelling"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
