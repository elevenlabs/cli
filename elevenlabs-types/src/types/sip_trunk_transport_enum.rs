pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SipTrunkTransportEnum {
    Auto,
    Udp,
    Tcp,
    Tls,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SipTrunkTransportEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Udp => serializer.serialize_str("udp"),
            Self::Tcp => serializer.serialize_str("tcp"),
            Self::Tls => serializer.serialize_str("tls"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SipTrunkTransportEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "udp" => Ok(Self::Udp),
            "tcp" => Ok(Self::Tcp),
            "tls" => Ok(Self::Tls),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SipTrunkTransportEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Udp => write!(f, "udp"),
            Self::Tcp => write!(f, "tcp"),
            Self::Tls => write!(f, "tls"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
