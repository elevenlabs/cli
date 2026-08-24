pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveMessageMulti {
        AudioOutputMulti(AudioOutputMulti),

        FinalOutputMulti(FinalOutputMulti),
}

impl ReceiveMessageMulti {
    pub fn is_audio_output_multi(&self) -> bool {
        matches!(self, Self::AudioOutputMulti(_))
    }

    pub fn is_final_output_multi(&self) -> bool {
        matches!(self, Self::FinalOutputMulti(_))
    }


    pub fn as_audio_output_multi(&self) -> Option<&AudioOutputMulti> {
        match self {
                    Self::AudioOutputMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_audio_output_multi(self) -> Option<AudioOutputMulti> {
        match self {
                    Self::AudioOutputMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_final_output_multi(&self) -> Option<&FinalOutputMulti> {
        match self {
                    Self::FinalOutputMulti(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_final_output_multi(self) -> Option<FinalOutputMulti> {
        match self {
                    Self::FinalOutputMulti(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveMessageMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AudioOutputMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FinalOutputMulti(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
