pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechHistoryItemResponseSource {
    Tts,
    Sts,
    Projects,
    Pd,
    An,
    Dubbing,
    PlayApi,
    ConvAi,
    VoiceGeneration,
    InVpc,
    Flows,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechHistoryItemResponseSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tts => serializer.serialize_str("TTS"),
            Self::Sts => serializer.serialize_str("STS"),
            Self::Projects => serializer.serialize_str("Projects"),
            Self::Pd => serializer.serialize_str("PD"),
            Self::An => serializer.serialize_str("AN"),
            Self::Dubbing => serializer.serialize_str("Dubbing"),
            Self::PlayApi => serializer.serialize_str("PlayAPI"),
            Self::ConvAi => serializer.serialize_str("ConvAI"),
            Self::VoiceGeneration => serializer.serialize_str("VoiceGeneration"),
            Self::InVpc => serializer.serialize_str("InVPC"),
            Self::Flows => serializer.serialize_str("Flows"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechHistoryItemResponseSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TTS" => Ok(Self::Tts),
            "STS" => Ok(Self::Sts),
            "Projects" => Ok(Self::Projects),
            "PD" => Ok(Self::Pd),
            "AN" => Ok(Self::An),
            "Dubbing" => Ok(Self::Dubbing),
            "PlayAPI" => Ok(Self::PlayApi),
            "ConvAI" => Ok(Self::ConvAi),
            "VoiceGeneration" => Ok(Self::VoiceGeneration),
            "InVPC" => Ok(Self::InVpc),
            "Flows" => Ok(Self::Flows),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechHistoryItemResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tts => write!(f, "TTS"),
            Self::Sts => write!(f, "STS"),
            Self::Projects => write!(f, "Projects"),
            Self::Pd => write!(f, "PD"),
            Self::An => write!(f, "AN"),
            Self::Dubbing => write!(f, "Dubbing"),
            Self::PlayApi => write!(f, "PlayAPI"),
            Self::ConvAi => write!(f, "ConvAI"),
            Self::VoiceGeneration => write!(f, "VoiceGeneration"),
            Self::InVpc => write!(f, "InVPC"),
            Self::Flows => write!(f, "Flows"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
