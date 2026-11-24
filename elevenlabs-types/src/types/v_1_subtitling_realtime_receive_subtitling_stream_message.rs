pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveSubtitlingStreamMessage {
        SubtitlingSessionStarted(SubtitlingSessionStarted),

        SubtitlingStatus(SubtitlingStatus),

        SubtitlingPartialTranscript(SubtitlingPartialTranscript),

        SubtitlingFinalTranscript(SubtitlingFinalTranscript),

        SubtitlingPartialSubtitle(SubtitlingPartialSubtitle),

        SubtitlingCommittedSubtitle(SubtitlingCommittedSubtitle),

        SubtitlingError(SubtitlingError),

        SubtitlingAuthError(SubtitlingAuthError),

        SubtitlingRateLimitedError(SubtitlingRateLimitedError),

        SubtitlingQueueOverflowError(SubtitlingQueueOverflowError),

        SubtitlingInputError(SubtitlingInputError),

        SubtitlingTranscriberError(SubtitlingTranscriberError),

        SubtitlingTranslationError(SubtitlingTranslationError),

        SubtitlingQuotaExceededError(SubtitlingQuotaExceededError),

        SubtitlingPaymentIssueError(SubtitlingPaymentIssueError),

        SubtitlingUnacceptedTermsError(SubtitlingUnacceptedTermsError),
}

impl ReceiveSubtitlingStreamMessage {
    pub fn is_subtitling_session_started(&self) -> bool {
        matches!(self, Self::SubtitlingSessionStarted(_))
    }

    pub fn is_subtitling_status(&self) -> bool {
        matches!(self, Self::SubtitlingStatus(_))
    }

    pub fn is_subtitling_partial_transcript(&self) -> bool {
        matches!(self, Self::SubtitlingPartialTranscript(_))
    }

    pub fn is_subtitling_final_transcript(&self) -> bool {
        matches!(self, Self::SubtitlingFinalTranscript(_))
    }

    pub fn is_subtitling_partial_subtitle(&self) -> bool {
        matches!(self, Self::SubtitlingPartialSubtitle(_))
    }

    pub fn is_subtitling_committed_subtitle(&self) -> bool {
        matches!(self, Self::SubtitlingCommittedSubtitle(_))
    }

    pub fn is_subtitling_error(&self) -> bool {
        matches!(self, Self::SubtitlingError(_))
    }

    pub fn is_subtitling_auth_error(&self) -> bool {
        matches!(self, Self::SubtitlingAuthError(_))
    }

    pub fn is_subtitling_rate_limited_error(&self) -> bool {
        matches!(self, Self::SubtitlingRateLimitedError(_))
    }

    pub fn is_subtitling_queue_overflow_error(&self) -> bool {
        matches!(self, Self::SubtitlingQueueOverflowError(_))
    }

    pub fn is_subtitling_input_error(&self) -> bool {
        matches!(self, Self::SubtitlingInputError(_))
    }

    pub fn is_subtitling_transcriber_error(&self) -> bool {
        matches!(self, Self::SubtitlingTranscriberError(_))
    }

    pub fn is_subtitling_translation_error(&self) -> bool {
        matches!(self, Self::SubtitlingTranslationError(_))
    }

    pub fn is_subtitling_quota_exceeded_error(&self) -> bool {
        matches!(self, Self::SubtitlingQuotaExceededError(_))
    }

    pub fn is_subtitling_payment_issue_error(&self) -> bool {
        matches!(self, Self::SubtitlingPaymentIssueError(_))
    }

    pub fn is_subtitling_unaccepted_terms_error(&self) -> bool {
        matches!(self, Self::SubtitlingUnacceptedTermsError(_))
    }


    pub fn as_subtitling_session_started(&self) -> Option<&SubtitlingSessionStarted> {
        match self {
                    Self::SubtitlingSessionStarted(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_session_started(self) -> Option<SubtitlingSessionStarted> {
        match self {
                    Self::SubtitlingSessionStarted(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_status(&self) -> Option<&SubtitlingStatus> {
        match self {
                    Self::SubtitlingStatus(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_status(self) -> Option<SubtitlingStatus> {
        match self {
                    Self::SubtitlingStatus(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_partial_transcript(&self) -> Option<&SubtitlingPartialTranscript> {
        match self {
                    Self::SubtitlingPartialTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_partial_transcript(self) -> Option<SubtitlingPartialTranscript> {
        match self {
                    Self::SubtitlingPartialTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_final_transcript(&self) -> Option<&SubtitlingFinalTranscript> {
        match self {
                    Self::SubtitlingFinalTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_final_transcript(self) -> Option<SubtitlingFinalTranscript> {
        match self {
                    Self::SubtitlingFinalTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_partial_subtitle(&self) -> Option<&SubtitlingPartialSubtitle> {
        match self {
                    Self::SubtitlingPartialSubtitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_partial_subtitle(self) -> Option<SubtitlingPartialSubtitle> {
        match self {
                    Self::SubtitlingPartialSubtitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_committed_subtitle(&self) -> Option<&SubtitlingCommittedSubtitle> {
        match self {
                    Self::SubtitlingCommittedSubtitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_committed_subtitle(self) -> Option<SubtitlingCommittedSubtitle> {
        match self {
                    Self::SubtitlingCommittedSubtitle(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_error(&self) -> Option<&SubtitlingError> {
        match self {
                    Self::SubtitlingError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_error(self) -> Option<SubtitlingError> {
        match self {
                    Self::SubtitlingError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_auth_error(&self) -> Option<&SubtitlingAuthError> {
        match self {
                    Self::SubtitlingAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_auth_error(self) -> Option<SubtitlingAuthError> {
        match self {
                    Self::SubtitlingAuthError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_rate_limited_error(&self) -> Option<&SubtitlingRateLimitedError> {
        match self {
                    Self::SubtitlingRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_rate_limited_error(self) -> Option<SubtitlingRateLimitedError> {
        match self {
                    Self::SubtitlingRateLimitedError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_queue_overflow_error(&self) -> Option<&SubtitlingQueueOverflowError> {
        match self {
                    Self::SubtitlingQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_queue_overflow_error(self) -> Option<SubtitlingQueueOverflowError> {
        match self {
                    Self::SubtitlingQueueOverflowError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_input_error(&self) -> Option<&SubtitlingInputError> {
        match self {
                    Self::SubtitlingInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_input_error(self) -> Option<SubtitlingInputError> {
        match self {
                    Self::SubtitlingInputError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_transcriber_error(&self) -> Option<&SubtitlingTranscriberError> {
        match self {
                    Self::SubtitlingTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_transcriber_error(self) -> Option<SubtitlingTranscriberError> {
        match self {
                    Self::SubtitlingTranscriberError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_translation_error(&self) -> Option<&SubtitlingTranslationError> {
        match self {
                    Self::SubtitlingTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_translation_error(self) -> Option<SubtitlingTranslationError> {
        match self {
                    Self::SubtitlingTranslationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_quota_exceeded_error(&self) -> Option<&SubtitlingQuotaExceededError> {
        match self {
                    Self::SubtitlingQuotaExceededError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_quota_exceeded_error(self) -> Option<SubtitlingQuotaExceededError> {
        match self {
                    Self::SubtitlingQuotaExceededError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_payment_issue_error(&self) -> Option<&SubtitlingPaymentIssueError> {
        match self {
                    Self::SubtitlingPaymentIssueError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_payment_issue_error(self) -> Option<SubtitlingPaymentIssueError> {
        match self {
                    Self::SubtitlingPaymentIssueError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_subtitling_unaccepted_terms_error(&self) -> Option<&SubtitlingUnacceptedTermsError> {
        match self {
                    Self::SubtitlingUnacceptedTermsError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_subtitling_unaccepted_terms_error(self) -> Option<SubtitlingUnacceptedTermsError> {
        match self {
                    Self::SubtitlingUnacceptedTermsError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveSubtitlingStreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubtitlingSessionStarted(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingStatus(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingPartialTranscript(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingFinalTranscript(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingPartialSubtitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingCommittedSubtitle(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingAuthError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingRateLimitedError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingQueueOverflowError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingInputError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingTranscriberError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingTranslationError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingQuotaExceededError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingPaymentIssueError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SubtitlingUnacceptedTermsError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
