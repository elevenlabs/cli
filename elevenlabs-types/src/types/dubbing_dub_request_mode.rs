pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The mode in which to run this Dubbing job. Defaults to automatic, use manual if specifically providing a CSV transcript to use. Note that manual mode is experimental and production use is strongly discouraged.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubRequestMode {
    Automatic,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubRequestMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automatic => serializer.serialize_str("automatic"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubRequestMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubRequestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => write!(f, "automatic"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
