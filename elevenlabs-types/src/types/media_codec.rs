pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MediaCodec {
    G7228000,
    Pcmu8000,
    Pcma8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MediaCodec {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::G7228000 => serializer.serialize_str("G722/8000"),
            Self::Pcmu8000 => serializer.serialize_str("PCMU/8000"),
            Self::Pcma8000 => serializer.serialize_str("PCMA/8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MediaCodec {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "G722/8000" => Ok(Self::G7228000),
            "PCMU/8000" => Ok(Self::Pcmu8000),
            "PCMA/8000" => Ok(Self::Pcma8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MediaCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::G7228000 => write!(f, "G722/8000"),
            Self::Pcmu8000 => write!(f, "PCMU/8000"),
            Self::Pcma8000 => write!(f, "PCMA/8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
