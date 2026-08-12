pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PronunciationDictionariesListRequestSort {
    CreationTimeUnix,
    Name,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PronunciationDictionariesListRequestSort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreationTimeUnix => serializer.serialize_str("creation_time_unix"),
            Self::Name => serializer.serialize_str("name"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PronunciationDictionariesListRequestSort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "creation_time_unix" => Ok(Self::CreationTimeUnix),
            "name" => Ok(Self::Name),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PronunciationDictionariesListRequestSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreationTimeUnix => write!(f, "creation_time_unix"),
            Self::Name => write!(f, "name"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
