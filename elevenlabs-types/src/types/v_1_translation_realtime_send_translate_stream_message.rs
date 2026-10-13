pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SendTranslateStreamMessage {
        TranslateInputAudioChunkPayload(TranslateInputAudioChunkPayload),

        TranslateEndOfStreamPayload(TranslateEndOfStreamPayload),
}

impl SendTranslateStreamMessage {
    pub fn is_translate_input_audio_chunk_payload(&self) -> bool {
        matches!(self, Self::TranslateInputAudioChunkPayload(_))
    }

    pub fn is_translate_end_of_stream_payload(&self) -> bool {
        matches!(self, Self::TranslateEndOfStreamPayload(_))
    }


    pub fn as_translate_input_audio_chunk_payload(&self) -> Option<&TranslateInputAudioChunkPayload> {
        match self {
                    Self::TranslateInputAudioChunkPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_input_audio_chunk_payload(self) -> Option<TranslateInputAudioChunkPayload> {
        match self {
                    Self::TranslateInputAudioChunkPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_end_of_stream_payload(&self) -> Option<&TranslateEndOfStreamPayload> {
        match self {
                    Self::TranslateEndOfStreamPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_end_of_stream_payload(self) -> Option<TranslateEndOfStreamPayload> {
        match self {
                    Self::TranslateEndOfStreamPayload(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SendTranslateStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TranslateInputAudioChunkPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateEndOfStreamPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
