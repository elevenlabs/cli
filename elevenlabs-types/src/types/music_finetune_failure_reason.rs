pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MusicFinetuneFailureReason {
    AudioProcessingFailed,
    CopyrightViolation,
    TrainingFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MusicFinetuneFailureReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AudioProcessingFailed => serializer.serialize_str("audio_processing_failed"),
            Self::CopyrightViolation => serializer.serialize_str("copyright_violation"),
            Self::TrainingFailed => serializer.serialize_str("training_failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MusicFinetuneFailureReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "audio_processing_failed" => Ok(Self::AudioProcessingFailed),
            "copyright_violation" => Ok(Self::CopyrightViolation),
            "training_failed" => Ok(Self::TrainingFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MusicFinetuneFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioProcessingFailed => write!(f, "audio_processing_failed"),
            Self::CopyrightViolation => write!(f, "copyright_violation"),
            Self::TrainingFailed => write!(f, "training_failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
