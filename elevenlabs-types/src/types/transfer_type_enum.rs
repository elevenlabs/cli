pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransferTypeEnum {
    Blind,
    Conference,
    SipRefer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TransferTypeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blind => serializer.serialize_str("blind"),
            Self::Conference => serializer.serialize_str("conference"),
            Self::SipRefer => serializer.serialize_str("sip_refer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TransferTypeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blind" => Ok(Self::Blind),
            "conference" => Ok(Self::Conference),
            "sip_refer" => Ok(Self::SipRefer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TransferTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blind => write!(f, "blind"),
            Self::Conference => write!(f, "conference"),
            Self::SipRefer => write!(f, "sip_refer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
