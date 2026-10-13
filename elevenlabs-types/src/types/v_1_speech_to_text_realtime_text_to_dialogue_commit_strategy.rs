pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Commit strategy for speech segmentation. 'manual' requires explicit commits, 'vad' automatically segments speech using silence detection, 'turn_prediction' additionally commits as soon as the model predicts the speaker has finished their turn (only for models that predict turns, where it is the default).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextToDialogueCommitStrategy {
    Manual,
    Vad,
    TurnPrediction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TextToDialogueCommitStrategy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Vad => serializer.serialize_str("vad"),
            Self::TurnPrediction => serializer.serialize_str("turn_prediction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TextToDialogueCommitStrategy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "vad" => Ok(Self::Vad),
            "turn_prediction" => Ok(Self::TurnPrediction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TextToDialogueCommitStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Vad => write!(f, "vad"),
            Self::TurnPrediction => write!(f, "turn_prediction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
