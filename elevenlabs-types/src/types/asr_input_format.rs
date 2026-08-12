pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsrInputFormat {
    Pcm8000,
    Pcm16000,
    Pcm22050,
    Pcm24000,
    Pcm44100,
    Pcm48000,
    Ulaw8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AsrInputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pcm8000 => serializer.serialize_str("pcm_8000"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Pcm48000 => serializer.serialize_str("pcm_48000"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AsrInputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pcm_8000" => Ok(Self::Pcm8000),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "pcm_48000" => Ok(Self::Pcm48000),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AsrInputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pcm8000 => write!(f, "pcm_8000"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Pcm48000 => write!(f, "pcm_48000"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
