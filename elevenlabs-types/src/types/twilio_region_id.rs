pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Valid Twilio region IDs.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TwilioRegionId {
    Us1,
    Ie1,
    Au1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TwilioRegionId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Us1 => serializer.serialize_str("us1"),
            Self::Ie1 => serializer.serialize_str("ie1"),
            Self::Au1 => serializer.serialize_str("au1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TwilioRegionId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "us1" => Ok(Self::Us1),
            "ie1" => Ok(Self::Ie1),
            "au1" => Ok(Self::Au1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TwilioRegionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Us1 => write!(f, "us1"),
            Self::Ie1 => write!(f, "ie1"),
            Self::Au1 => write!(f, "au1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
