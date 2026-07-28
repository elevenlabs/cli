pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TtsModelFamily {
    /// Deprecated: Use flash instead.
    Turbo,
    Flash,
    Multilingual,
    V3Conversational,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TtsModelFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Turbo => serializer.serialize_str("turbo"),
            Self::Flash => serializer.serialize_str("flash"),
            Self::Multilingual => serializer.serialize_str("multilingual"),
            Self::V3Conversational => serializer.serialize_str("v3_conversational"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TtsModelFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "turbo" => Ok(Self::Turbo),
            "flash" => Ok(Self::Flash),
            "multilingual" => Ok(Self::Multilingual),
            "v3_conversational" => Ok(Self::V3Conversational),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TtsModelFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Turbo => write!(f, "turbo"),
            Self::Flash => write!(f, "flash"),
            Self::Multilingual => write!(f, "multilingual"),
            Self::V3Conversational => write!(f, "v3_conversational"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
