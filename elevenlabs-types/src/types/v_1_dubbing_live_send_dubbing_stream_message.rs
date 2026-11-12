pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SendDubbingStreamMessage {
        DubbingInputAudioChunk(DubbingInputAudioChunk),

        DubbingEndOfStream(DubbingEndOfStream),
}

impl SendDubbingStreamMessage {
    pub fn is_dubbing_input_audio_chunk(&self) -> bool {
        matches!(self, Self::DubbingInputAudioChunk(_))
    }

    pub fn is_dubbing_end_of_stream(&self) -> bool {
        matches!(self, Self::DubbingEndOfStream(_))
    }


    pub fn as_dubbing_input_audio_chunk(&self) -> Option<&DubbingInputAudioChunk> {
        match self {
                    Self::DubbingInputAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_input_audio_chunk(self) -> Option<DubbingInputAudioChunk> {
        match self {
                    Self::DubbingInputAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_end_of_stream(&self) -> Option<&DubbingEndOfStream> {
        match self {
                    Self::DubbingEndOfStream(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_end_of_stream(self) -> Option<DubbingEndOfStream> {
        match self {
                    Self::DubbingEndOfStream(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendDubbingStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DubbingInputAudioChunk(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingEndOfStream(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
