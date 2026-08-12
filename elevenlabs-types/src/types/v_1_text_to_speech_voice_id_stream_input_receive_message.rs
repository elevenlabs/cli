pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveMessage {
        AudioOutput(AudioOutput),

        FinalOutput(FinalOutput),
}

impl ReceiveMessage {
    pub fn is_audio_output(&self) -> bool {
        matches!(self, Self::AudioOutput(_))
    }

    pub fn is_final_output(&self) -> bool {
        matches!(self, Self::FinalOutput(_))
    }


    pub fn as_audio_output(&self) -> Option<&AudioOutput> {
        match self {
                    Self::AudioOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_audio_output(self) -> Option<AudioOutput> {
        match self {
                    Self::AudioOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_final_output(&self) -> Option<&FinalOutput> {
        match self {
                    Self::FinalOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_final_output(self) -> Option<FinalOutput> {
        match self {
                    Self::FinalOutput(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FinalOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
