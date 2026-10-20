pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhatsAppCallPermissionRequestState {
    Blocked,
    Sent,
    Accepted,
    Declined,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WhatsAppCallPermissionRequestState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WhatsAppCallPermissionRequestState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blocked" => Ok(Self::Blocked),
            "sent" => Ok(Self::Sent),
            "accepted" => Ok(Self::Accepted),
            "declined" => Ok(Self::Declined),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WhatsAppCallPermissionRequestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocked => write!(f, "blocked"),
            Self::Sent => write!(f, "sent"),
            Self::Accepted => write!(f, "accepted"),
            Self::Declined => write!(f, "declined"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
