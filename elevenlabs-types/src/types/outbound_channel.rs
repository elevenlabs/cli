pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutboundChannel {
    Phone,
    Whatsapp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutboundChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Phone => serializer.serialize_str("phone"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutboundChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "phone" => Ok(Self::Phone),
            "whatsapp" => Ok(Self::Whatsapp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutboundChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Phone => write!(f, "phone"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
