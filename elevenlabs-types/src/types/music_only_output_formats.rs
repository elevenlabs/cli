pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MusicOnlyOutputFormats {
    Mp348000128,
    Mp348000192,
    Mp348000240,
    Mp348000320,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MusicOnlyOutputFormats {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp348000128 => serializer.serialize_str("mp3_48000_128"),
            Self::Mp348000192 => serializer.serialize_str("mp3_48000_192"),
            Self::Mp348000240 => serializer.serialize_str("mp3_48000_240"),
            Self::Mp348000320 => serializer.serialize_str("mp3_48000_320"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MusicOnlyOutputFormats {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_48000_128" => Ok(Self::Mp348000128),
            "mp3_48000_192" => Ok(Self::Mp348000192),
            "mp3_48000_240" => Ok(Self::Mp348000240),
            "mp3_48000_320" => Ok(Self::Mp348000320),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MusicOnlyOutputFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp348000128 => write!(f, "mp3_48000_128"),
            Self::Mp348000192 => write!(f, "mp3_48000_192"),
            Self::Mp348000240 => write!(f, "mp3_48000_240"),
            Self::Mp348000320 => write!(f, "mp3_48000_320"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
