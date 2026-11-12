pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutboundMode {
    Voice,
    Message,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutboundMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Voice => serializer.serialize_str("voice"),
            Self::Message => serializer.serialize_str("message"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutboundMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "voice" => Ok(Self::Voice),
            "message" => Ok(Self::Message),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutboundMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Voice => write!(f, "voice"),
            Self::Message => write!(f, "message"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
