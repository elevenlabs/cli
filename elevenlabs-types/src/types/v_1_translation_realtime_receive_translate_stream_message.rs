pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveTranslateStreamMessage {
        TranslateSessionStartedPayload(TranslateSessionStartedPayload),

        TranslateStatusPayload(TranslateStatusPayload),

        TranslatePartialTranscriptPayload(TranslatePartialTranscriptPayload),

        TranslateFinalTranscriptPayload(TranslateFinalTranscriptPayload),

        TranslateTranslationPayload(TranslateTranslationPayload),

        TranslateAudioPayload(TranslateAudioPayload),

        TranslateErrorPayload(TranslateErrorPayload),

        TranslateAuthError(TranslateAuthError),

        TranslateRateLimitedError(TranslateRateLimitedError),

        TranslateQueueOverflowError(TranslateQueueOverflowError),

        TranslateInputError(TranslateInputError),

        TranslateTranscriberError(TranslateTranscriberError),

        TranslateTranslationError(TranslateTranslationError),

        TranslateAudioOutputError(TranslateAudioOutputError),
}

impl ReceiveTranslateStreamMessage {
    pub fn is_translate_session_started_payload(&self) -> bool {
        matches!(self, Self::TranslateSessionStartedPayload(_))
    }

    pub fn is_translate_status_payload(&self) -> bool {
        matches!(self, Self::TranslateStatusPayload(_))
    }

    pub fn is_translate_partial_transcript_payload(&self) -> bool {
        matches!(self, Self::TranslatePartialTranscriptPayload(_))
    }

    pub fn is_translate_final_transcript_payload(&self) -> bool {
        matches!(self, Self::TranslateFinalTranscriptPayload(_))
    }

    pub fn is_translate_translation_payload(&self) -> bool {
        matches!(self, Self::TranslateTranslationPayload(_))
    }

    pub fn is_translate_audio_payload(&self) -> bool {
        matches!(self, Self::TranslateAudioPayload(_))
    }

    pub fn is_translate_error_payload(&self) -> bool {
        matches!(self, Self::TranslateErrorPayload(_))
    }

    pub fn is_translate_auth_error(&self) -> bool {
        matches!(self, Self::TranslateAuthError(_))
    }

    pub fn is_translate_rate_limited_error(&self) -> bool {
        matches!(self, Self::TranslateRateLimitedError(_))
    }

    pub fn is_translate_queue_overflow_error(&self) -> bool {
        matches!(self, Self::TranslateQueueOverflowError(_))
    }

    pub fn is_translate_input_error(&self) -> bool {
        matches!(self, Self::TranslateInputError(_))
    }

    pub fn is_translate_transcriber_error(&self) -> bool {
        matches!(self, Self::TranslateTranscriberError(_))
    }

    pub fn is_translate_translation_error(&self) -> bool {
        matches!(self, Self::TranslateTranslationError(_))
    }

    pub fn is_translate_audio_output_error(&self) -> bool {
        matches!(self, Self::TranslateAudioOutputError(_))
    }


    pub fn as_translate_session_started_payload(&self) -> Option<&TranslateSessionStartedPayload> {
        match self {
                    Self::TranslateSessionStartedPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_session_started_payload(self) -> Option<TranslateSessionStartedPayload> {
        match self {
                    Self::TranslateSessionStartedPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_status_payload(&self) -> Option<&TranslateStatusPayload> {
        match self {
                    Self::TranslateStatusPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_status_payload(self) -> Option<TranslateStatusPayload> {
        match self {
                    Self::TranslateStatusPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_partial_transcript_payload(&self) -> Option<&TranslatePartialTranscriptPayload> {
        match self {
                    Self::TranslatePartialTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_partial_transcript_payload(self) -> Option<TranslatePartialTranscriptPayload> {
        match self {
                    Self::TranslatePartialTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_final_transcript_payload(&self) -> Option<&TranslateFinalTranscriptPayload> {
        match self {
                    Self::TranslateFinalTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_final_transcript_payload(self) -> Option<TranslateFinalTranscriptPayload> {
        match self {
                    Self::TranslateFinalTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_translation_payload(&self) -> Option<&TranslateTranslationPayload> {
        match self {
                    Self::TranslateTranslationPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_translation_payload(self) -> Option<TranslateTranslationPayload> {
        match self {
                    Self::TranslateTranslationPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_audio_payload(&self) -> Option<&TranslateAudioPayload> {
        match self {
                    Self::TranslateAudioPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_audio_payload(self) -> Option<TranslateAudioPayload> {
        match self {
                    Self::TranslateAudioPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_error_payload(&self) -> Option<&TranslateErrorPayload> {
        match self {
                    Self::TranslateErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_error_payload(self) -> Option<TranslateErrorPayload> {
        match self {
                    Self::TranslateErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_auth_error(&self) -> Option<&TranslateAuthError> {
        match self {
                    Self::TranslateAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_auth_error(self) -> Option<TranslateAuthError> {
        match self {
                    Self::TranslateAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_rate_limited_error(&self) -> Option<&TranslateRateLimitedError> {
        match self {
                    Self::TranslateRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_rate_limited_error(self) -> Option<TranslateRateLimitedError> {
        match self {
                    Self::TranslateRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_queue_overflow_error(&self) -> Option<&TranslateQueueOverflowError> {
        match self {
                    Self::TranslateQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_queue_overflow_error(self) -> Option<TranslateQueueOverflowError> {
        match self {
                    Self::TranslateQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_input_error(&self) -> Option<&TranslateInputError> {
        match self {
                    Self::TranslateInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_input_error(self) -> Option<TranslateInputError> {
        match self {
                    Self::TranslateInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_transcriber_error(&self) -> Option<&TranslateTranscriberError> {
        match self {
                    Self::TranslateTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_transcriber_error(self) -> Option<TranslateTranscriberError> {
        match self {
                    Self::TranslateTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_translation_error(&self) -> Option<&TranslateTranslationError> {
        match self {
                    Self::TranslateTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_translation_error(self) -> Option<TranslateTranslationError> {
        match self {
                    Self::TranslateTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_translate_audio_output_error(&self) -> Option<&TranslateAudioOutputError> {
        match self {
                    Self::TranslateAudioOutputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_translate_audio_output_error(self) -> Option<TranslateAudioOutputError> {
        match self {
                    Self::TranslateAudioOutputError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveTranslateStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TranslateSessionStartedPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateStatusPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslatePartialTranscriptPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateFinalTranscriptPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateTranslationPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateAudioPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateAuthError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateRateLimitedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateQueueOverflowError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateInputError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateTranscriberError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateTranslationError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::TranslateAudioOutputError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
