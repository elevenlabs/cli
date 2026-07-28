pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    /// Output format, mp3 with 22.05kHz sample rate at 32kbps
    Mp32205032,
    /// Output format, mp3 with 44.1kHz sample rate at 32kbps
    Mp34410032,
    /// Output format, mp3 with 44.1kHz sample rate at 64kbps
    Mp34410064,
    /// Output format, mp3 with 44.1kHz sample rate at 96kbps
    Mp34410096,
    /// Default output format, mp3 with 44.1kHz sample rate at 128kbps
    Mp344100128,
    /// Output format, mp3 with 44.1kHz sample rate at 192kbps.
    Mp344100192,
    /// PCM format (S16LE) with 16kHz sample rate.
    Pcm16000,
    /// PCM format (S16LE) with 22.05kHz sample rate.
    Pcm22050,
    /// PCM format (S16LE) with 24kHz sample rate.
    Pcm24000,
    /// PCM format (S16LE) with 44.1kHz sample rate. Requires you to be subscribed to Independent Publisher tier or above.
    Pcm44100,
    /// μ-law format (sometimes written mu-law, often approximated as u-law) with 8kHz sample rate. Note that this format is commonly used for Twilio audio inputs.
    Ulaw8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mp32205032 => serializer.serialize_str("mp3_22050_32"),
            Self::Mp34410032 => serializer.serialize_str("mp3_44100_32"),
            Self::Mp34410064 => serializer.serialize_str("mp3_44100_64"),
            Self::Mp34410096 => serializer.serialize_str("mp3_44100_96"),
            Self::Mp344100128 => serializer.serialize_str("mp3_44100_128"),
            Self::Mp344100192 => serializer.serialize_str("mp3_44100_192"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm22050 => serializer.serialize_str("pcm_22050"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Ulaw8000 => serializer.serialize_str("ulaw_8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mp3_22050_32" => Ok(Self::Mp32205032),
            "mp3_44100_32" => Ok(Self::Mp34410032),
            "mp3_44100_64" => Ok(Self::Mp34410064),
            "mp3_44100_96" => Ok(Self::Mp34410096),
            "mp3_44100_128" => Ok(Self::Mp344100128),
            "mp3_44100_192" => Ok(Self::Mp344100192),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_22050" => Ok(Self::Pcm22050),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_44100" => Ok(Self::Pcm44100),
            "ulaw_8000" => Ok(Self::Ulaw8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mp32205032 => write!(f, "mp3_22050_32"),
            Self::Mp34410032 => write!(f, "mp3_44100_32"),
            Self::Mp34410064 => write!(f, "mp3_44100_64"),
            Self::Mp34410096 => write!(f, "mp3_44100_96"),
            Self::Mp344100128 => write!(f, "mp3_44100_128"),
            Self::Mp344100192 => write!(f, "mp3_44100_192"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm22050 => write!(f, "pcm_22050"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Ulaw8000 => write!(f, "ulaw_8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
