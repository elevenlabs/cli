pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Controls if the agent should be more patient when user is spelling numbers and named entities.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpellingPatience {
    Auto,
    Off,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpellingPatience {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Off => serializer.serialize_str("off"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpellingPatience {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "off" => Ok(Self::Off),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpellingPatience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Off => write!(f, "off"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
