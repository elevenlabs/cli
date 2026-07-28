pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TranscriptGetTranscriptForDubResponse {
        DubbingTranscriptResponseModel(DubbingTranscriptResponseModel),

        String(String),
}

impl TranscriptGetTranscriptForDubResponse {
    pub fn is_dubbing_transcript_response_model(&self) -> bool {
        matches!(self, Self::DubbingTranscriptResponseModel(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_dubbing_transcript_response_model(&self) -> Option<&DubbingTranscriptResponseModel> {
        match self {
                    Self::DubbingTranscriptResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_transcript_response_model(self) -> Option<DubbingTranscriptResponseModel> {
        match self {
                    Self::DubbingTranscriptResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for TranscriptGetTranscriptForDubResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DubbingTranscriptResponseModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
