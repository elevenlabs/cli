pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The output audio format
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextToSpeechOutputFormatEnum {
    Mp32205032,
    Mp34410032,
    Mp34410064,
    Mp34410096,
    Mp344100128,
    Mp344100192,
    Pcm8000,
    Pcm16000,
    Pcm22050,
    Pcm24000,
    Pcm44100,
    Ulaw8000,
    Alaw8000,
    Opus4800032,
    Opus4800064,
    Opus4800096,
    Opus48000128,
    Opus48000192,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextToSpeechOutputFormatEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp32205032 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp34410032 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp34410064 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp34410096 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp344100128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp344100192 => serializer.serialize_str("mp3_44100_192"),
            Self::Pcm8000 => serializer.serialize_str("pcm_8000"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::Alaw8000 => serializer.serialize_str("alaw_8000"),
            Self::Opus4800032 => serializer.serialize_str("opus_48000_32"),
            Self::Opus4800064 => serializer.serialize_str("opus_48000_64"),
            Self::Opus4800096 => serializer.serialize_str("opus_48000_96"),
            Self::Opus48000128 => serializer.serialize_str("opus_48000_128"),
            Self::Opus48000192 => serializer.serialize_str("opus_48000_192"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextToSpeechOutputFormatEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_22050_32" => Ok(Self::Mp32205032),
            "mp3_44100_32" => Ok(Self::Mp34410032),
            "mp3_44100_64" => Ok(Self::Mp34410064),
            "mp3_44100_96" => Ok(Self::Mp34410096),
            "mp3_44100_128" => Ok(Self::Mp344100128),
            "mp3_44100_192" => Ok(Self::Mp344100192),
            "pcm_8000" => Ok(Self::Pcm8000),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            "alaw_8000" => Ok(Self::Alaw8000),
            "opus_48000_32" => Ok(Self::Opus4800032),
            "opus_48000_64" => Ok(Self::Opus4800064),
            "opus_48000_96" => Ok(Self::Opus4800096),
            "opus_48000_128" => Ok(Self::Opus48000128),
            "opus_48000_192" => Ok(Self::Opus48000192),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextToSpeechOutputFormatEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp32205032 => write!(f, "mp3_22050_32"),
            Self::Mp34410032 => write!(f, "mp3_44100_32"),
            Self::Mp34410064 => write!(f, "mp3_44100_64"),
            Self::Mp34410096 => write!(f, "mp3_44100_96"),
            Self::Mp344100128 => write!(f, "mp3_44100_128"),
            Self::Mp344100192 => write!(f, "mp3_44100_192"),
            Self::Pcm8000 => write!(f, "pcm_8000"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::Alaw8000 => write!(f, "alaw_8000"),
            Self::Opus4800032 => write!(f, "opus_48000_32"),
            Self::Opus4800064 => write!(f, "opus_48000_64"),
            Self::Opus4800096 => write!(f, "opus_48000_96"),
            Self::Opus48000128 => write!(f, "opus_48000_128"),
            Self::Opus48000192 => write!(f, "opus_48000_192"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
