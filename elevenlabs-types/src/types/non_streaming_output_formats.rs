pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonStreamingOutputFormats {
    Wav8000,
    Wav16000,
    Wav22050,
    Wav24000,
    Wav32000,
    Wav44100,
    Wav48000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NonStreamingOutputFormats {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Wav8000 => serializer.serialize_str("wav_8000"),
            Self::Wav16000 => serializer.serialize_str("wav_16000"),
            Self::Wav22050 => serializer.serialize_str("wav_22050"),
            Self::Wav24000 => serializer.serialize_str("wav_24000"),
            Self::Wav32000 => serializer.serialize_str("wav_32000"),
            Self::Wav44100 => serializer.serialize_str("wav_44100"),
            Self::Wav48000 => serializer.serialize_str("wav_48000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NonStreamingOutputFormats {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wav_8000" => Ok(Self::Wav8000),
            "wav_16000" => Ok(Self::Wav16000),
            "wav_22050" => Ok(Self::Wav22050),
            "wav_24000" => Ok(Self::Wav24000),
            "wav_32000" => Ok(Self::Wav32000),
            "wav_44100" => Ok(Self::Wav44100),
            "wav_48000" => Ok(Self::Wav48000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NonStreamingOutputFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wav8000 => write!(f, "wav_8000"),
            Self::Wav16000 => write!(f, "wav_16000"),
            Self::Wav22050 => write!(f, "wav_22050"),
            Self::Wav24000 => write!(f, "wav_24000"),
            Self::Wav32000 => write!(f, "wav_32000"),
            Self::Wav44100 => write!(f, "wav_44100"),
            Self::Wav48000 => write!(f, "wav_48000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
