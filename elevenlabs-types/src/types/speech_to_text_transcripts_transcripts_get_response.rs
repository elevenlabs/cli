pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TranscriptsGetResponse {
        SpeechToTextChunkResponseModel(SpeechToTextChunkResponseModel),

        MultichannelSpeechToTextResponseModel(MultichannelSpeechToTextResponseModel),
}

impl TranscriptsGetResponse {
    pub fn is_speech_to_text_chunk_response_model(&self) -> bool {
        matches!(self, Self::SpeechToTextChunkResponseModel(_))
    }

    pub fn is_multichannel_speech_to_text_response_model(&self) -> bool {
        matches!(self, Self::MultichannelSpeechToTextResponseModel(_))
    }


    pub fn as_speech_to_text_chunk_response_model(&self) -> Option<&SpeechToTextChunkResponseModel> {
        match self {
                    Self::SpeechToTextChunkResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_speech_to_text_chunk_response_model(self) -> Option<SpeechToTextChunkResponseModel> {
        match self {
                    Self::SpeechToTextChunkResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_multichannel_speech_to_text_response_model(&self) -> Option<&MultichannelSpeechToTextResponseModel> {
        match self {
                    Self::MultichannelSpeechToTextResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_multichannel_speech_to_text_response_model(self) -> Option<MultichannelSpeechToTextResponseModel> {
        match self {
                    Self::MultichannelSpeechToTextResponseModel(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TranscriptsGetResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpeechToTextChunkResponseModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::MultichannelSpeechToTextResponseModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
