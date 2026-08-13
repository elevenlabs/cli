pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The audio encoding of the output, as `codec_sampleRateHz_bitrateKbps`. `mp3_44100_192` requires the Creator tier or above.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElevenFlashV25RequestOutputFormat {
    Mp32205032,
    Mp32400048,
    Mp34410032,
    Mp34410064,
    Mp34410096,
    Mp344100128,
    Mp344100192,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ElevenFlashV25RequestOutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp32205032 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp32400048 => serializer.serialize_str("mp3_24000_48"),
            Self::Mp34410032 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp34410064 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp34410096 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp344100128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp344100192 => serializer.serialize_str("mp3_44100_192"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ElevenFlashV25RequestOutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_22050_32" => Ok(Self::Mp32205032),
            "mp3_24000_48" => Ok(Self::Mp32400048),
            "mp3_44100_32" => Ok(Self::Mp34410032),
            "mp3_44100_64" => Ok(Self::Mp34410064),
            "mp3_44100_96" => Ok(Self::Mp34410096),
            "mp3_44100_128" => Ok(Self::Mp344100128),
            "mp3_44100_192" => Ok(Self::Mp344100192),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ElevenFlashV25RequestOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp32205032 => write!(f, "mp3_22050_32"),
            Self::Mp32400048 => write!(f, "mp3_24000_48"),
            Self::Mp34410032 => write!(f, "mp3_44100_32"),
            Self::Mp34410064 => write!(f, "mp3_44100_64"),
            Self::Mp34410096 => write!(f, "mp3_44100_96"),
            Self::Mp344100128 => write!(f, "mp3_44100_128"),
            Self::Mp344100192 => write!(f, "mp3_44100_192"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
