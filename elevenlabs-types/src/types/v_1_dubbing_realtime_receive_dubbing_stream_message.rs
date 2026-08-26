pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveDubbingStreamMessage {
        DubbingSessionStarted(DubbingSessionStarted),

        DubbingStatus(DubbingStatus),

        DubbingPartialTranscript(DubbingPartialTranscript),

        DubbingCommittedTranscript(DubbingCommittedTranscript),

        DubbingTranslation(DubbingTranslation),

        DubbingAudio(DubbingAudio),

        DubbingError(DubbingError),

        DubbingAuthError(DubbingAuthError),

        DubbingRateLimitedError(DubbingRateLimitedError),

        DubbingQueueOverflowError(DubbingQueueOverflowError),

        DubbingInvalidRequestError(DubbingInvalidRequestError),

        DubbingInputError(DubbingInputError),

        DubbingTranscriberError(DubbingTranscriberError),

        DubbingTranslationError(DubbingTranslationError),

        DubbingAudioOutputError(DubbingAudioOutputError),
}

impl ReceiveDubbingStreamMessage {
    pub fn is_dubbing_session_started(&self) -> bool {
        matches!(self, Self::DubbingSessionStarted(_))
    }

    pub fn is_dubbing_status(&self) -> bool {
        matches!(self, Self::DubbingStatus(_))
    }

    pub fn is_dubbing_partial_transcript(&self) -> bool {
        matches!(self, Self::DubbingPartialTranscript(_))
    }

    pub fn is_dubbing_committed_transcript(&self) -> bool {
        matches!(self, Self::DubbingCommittedTranscript(_))
    }

    pub fn is_dubbing_translation(&self) -> bool {
        matches!(self, Self::DubbingTranslation(_))
    }

    pub fn is_dubbing_audio(&self) -> bool {
        matches!(self, Self::DubbingAudio(_))
    }

    pub fn is_dubbing_error(&self) -> bool {
        matches!(self, Self::DubbingError(_))
    }

    pub fn is_dubbing_auth_error(&self) -> bool {
        matches!(self, Self::DubbingAuthError(_))
    }

    pub fn is_dubbing_rate_limited_error(&self) -> bool {
        matches!(self, Self::DubbingRateLimitedError(_))
    }

    pub fn is_dubbing_queue_overflow_error(&self) -> bool {
        matches!(self, Self::DubbingQueueOverflowError(_))
    }

    pub fn is_dubbing_invalid_request_error(&self) -> bool {
        matches!(self, Self::DubbingInvalidRequestError(_))
    }

    pub fn is_dubbing_input_error(&self) -> bool {
        matches!(self, Self::DubbingInputError(_))
    }

    pub fn is_dubbing_transcriber_error(&self) -> bool {
        matches!(self, Self::DubbingTranscriberError(_))
    }

    pub fn is_dubbing_translation_error(&self) -> bool {
        matches!(self, Self::DubbingTranslationError(_))
    }

    pub fn is_dubbing_audio_output_error(&self) -> bool {
        matches!(self, Self::DubbingAudioOutputError(_))
    }


    pub fn as_dubbing_session_started(&self) -> Option<&DubbingSessionStarted> {
        match self {
                    Self::DubbingSessionStarted(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_session_started(self) -> Option<DubbingSessionStarted> {
        match self {
                    Self::DubbingSessionStarted(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_status(&self) -> Option<&DubbingStatus> {
        match self {
                    Self::DubbingStatus(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_status(self) -> Option<DubbingStatus> {
        match self {
                    Self::DubbingStatus(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_partial_transcript(&self) -> Option<&DubbingPartialTranscript> {
        match self {
                    Self::DubbingPartialTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_partial_transcript(self) -> Option<DubbingPartialTranscript> {
        match self {
                    Self::DubbingPartialTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_committed_transcript(&self) -> Option<&DubbingCommittedTranscript> {
        match self {
                    Self::DubbingCommittedTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_committed_transcript(self) -> Option<DubbingCommittedTranscript> {
        match self {
                    Self::DubbingCommittedTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_translation(&self) -> Option<&DubbingTranslation> {
        match self {
                    Self::DubbingTranslation(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_translation(self) -> Option<DubbingTranslation> {
        match self {
                    Self::DubbingTranslation(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_audio(&self) -> Option<&DubbingAudio> {
        match self {
                    Self::DubbingAudio(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_audio(self) -> Option<DubbingAudio> {
        match self {
                    Self::DubbingAudio(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_error(&self) -> Option<&DubbingError> {
        match self {
                    Self::DubbingError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_error(self) -> Option<DubbingError> {
        match self {
                    Self::DubbingError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_auth_error(&self) -> Option<&DubbingAuthError> {
        match self {
                    Self::DubbingAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_auth_error(self) -> Option<DubbingAuthError> {
        match self {
                    Self::DubbingAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_rate_limited_error(&self) -> Option<&DubbingRateLimitedError> {
        match self {
                    Self::DubbingRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_rate_limited_error(self) -> Option<DubbingRateLimitedError> {
        match self {
                    Self::DubbingRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_queue_overflow_error(&self) -> Option<&DubbingQueueOverflowError> {
        match self {
                    Self::DubbingQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_queue_overflow_error(self) -> Option<DubbingQueueOverflowError> {
        match self {
                    Self::DubbingQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_invalid_request_error(&self) -> Option<&DubbingInvalidRequestError> {
        match self {
                    Self::DubbingInvalidRequestError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_invalid_request_error(self) -> Option<DubbingInvalidRequestError> {
        match self {
                    Self::DubbingInvalidRequestError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_input_error(&self) -> Option<&DubbingInputError> {
        match self {
                    Self::DubbingInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_input_error(self) -> Option<DubbingInputError> {
        match self {
                    Self::DubbingInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_transcriber_error(&self) -> Option<&DubbingTranscriberError> {
        match self {
                    Self::DubbingTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_transcriber_error(self) -> Option<DubbingTranscriberError> {
        match self {
                    Self::DubbingTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_translation_error(&self) -> Option<&DubbingTranslationError> {
        match self {
                    Self::DubbingTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_translation_error(self) -> Option<DubbingTranslationError> {
        match self {
                    Self::DubbingTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_dubbing_audio_output_error(&self) -> Option<&DubbingAudioOutputError> {
        match self {
                    Self::DubbingAudioOutputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_dubbing_audio_output_error(self) -> Option<DubbingAudioOutputError> {
        match self {
                    Self::DubbingAudioOutputError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveDubbingStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DubbingSessionStarted(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingStatus(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingPartialTranscript(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingCommittedTranscript(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingTranslation(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingAudio(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingAuthError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingRateLimitedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingQueueOverflowError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingInvalidRequestError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingInputError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingTranscriberError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingTranslationError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DubbingAudioOutputError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
