pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreToolSpeechMode {
    Auto,
    Force,
    Off,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreToolSpeechMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Force => serializer.serialize_str("force"),
            Self::Off => serializer.serialize_str("off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreToolSpeechMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "force" => Ok(Self::Force),
            "off" => Ok(Self::Off),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreToolSpeechMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Force => write!(f, "force"),
            Self::Off => write!(f, "off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
