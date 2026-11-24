pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SendSubtitlingStreamMessage {
        SubtitlingInputAudioChunk(SubtitlingInputAudioChunk),

        SubtitlingEndOfStream(SubtitlingEndOfStream),
}

impl SendSubtitlingStreamMessage {
    pub fn is_subtitling_input_audio_chunk(&self) -> bool {
        matches!(self, Self::SubtitlingInputAudioChunk(_))
    }

    pub fn is_subtitling_end_of_stream(&self) -> bool {
        matches!(self, Self::SubtitlingEndOfStream(_))
    }


    pub fn as_subtitling_input_audio_chunk(&self) -> Option<&SubtitlingInputAudioChunk> {
        match self {
                    Self::SubtitlingInputAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_input_audio_chunk(self) -> Option<SubtitlingInputAudioChunk> {
        match self {
                    Self::SubtitlingInputAudioChunk(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_end_of_stream(&self) -> Option<&SubtitlingEndOfStream> {
        match self {
                    Self::SubtitlingEndOfStream(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_end_of_stream(self) -> Option<SubtitlingEndOfStream> {
        match self {
                    Self::SubtitlingEndOfStream(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendSubtitlingStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubtitlingInputAudioChunk(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingEndOfStream(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
