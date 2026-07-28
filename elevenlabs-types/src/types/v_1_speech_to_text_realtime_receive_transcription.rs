pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiveTranscription {
        SessionStartedPayload(SessionStartedPayload),

        PartialTranscriptPayload(PartialTranscriptPayload),

        FinalTranscript(FinalTranscript),

        FinalTranscriptWithTimestamps(FinalTranscriptWithTimestamps),

        CommittedTranscriptPayload(CommittedTranscriptPayload),

        CommittedTranscriptWithTimestampsPayload(CommittedTranscriptWithTimestampsPayload),

        CommittedTranscriptEntitiesPayload(CommittedTranscriptEntitiesPayload),

        ScribeErrorPayload(ScribeErrorPayload),

        ScribeAuthErrorPayload(ScribeAuthErrorPayload),

        ScribeQuotaExceededErrorPayload(ScribeQuotaExceededErrorPayload),

        ScribeThrottledErrorPayload(ScribeThrottledErrorPayload),

        ScribeUnacceptedTermsErrorPayload(ScribeUnacceptedTermsErrorPayload),

        ScribeRateLimitedErrorPayload(ScribeRateLimitedErrorPayload),

        ScribeQueueOverflowErrorPayload(ScribeQueueOverflowErrorPayload),

        ScribeResourceExhaustedErrorPayload(ScribeResourceExhaustedErrorPayload),

        ScribeSessionTimeLimitExceededErrorPayload(ScribeSessionTimeLimitExceededErrorPayload),

        ScribeInputErrorPayload(ScribeInputErrorPayload),

        ScribeChunkSizeExceededErrorPayload(ScribeChunkSizeExceededErrorPayload),

        ScribeInsufficientAudioActivityErrorPayload(ScribeInsufficientAudioActivityErrorPayload),

        ScribeTranscriberErrorPayload(ScribeTranscriberErrorPayload),
}

impl ReceiveTranscription {
    pub fn is_session_started_payload(&self) -> bool {
        matches!(self, Self::SessionStartedPayload(_))
    }

    pub fn is_partial_transcript_payload(&self) -> bool {
        matches!(self, Self::PartialTranscriptPayload(_))
    }

    pub fn is_final_transcript(&self) -> bool {
        matches!(self, Self::FinalTranscript(_))
    }

    pub fn is_final_transcript_with_timestamps(&self) -> bool {
        matches!(self, Self::FinalTranscriptWithTimestamps(_))
    }

    pub fn is_committed_transcript_payload(&self) -> bool {
        matches!(self, Self::CommittedTranscriptPayload(_))
    }

    pub fn is_committed_transcript_with_timestamps_payload(&self) -> bool {
        matches!(self, Self::CommittedTranscriptWithTimestampsPayload(_))
    }

    pub fn is_committed_transcript_entities_payload(&self) -> bool {
        matches!(self, Self::CommittedTranscriptEntitiesPayload(_))
    }

    pub fn is_scribe_error_payload(&self) -> bool {
        matches!(self, Self::ScribeErrorPayload(_))
    }

    pub fn is_scribe_auth_error_payload(&self) -> bool {
        matches!(self, Self::ScribeAuthErrorPayload(_))
    }

    pub fn is_scribe_quota_exceeded_error_payload(&self) -> bool {
        matches!(self, Self::ScribeQuotaExceededErrorPayload(_))
    }

    pub fn is_scribe_throttled_error_payload(&self) -> bool {
        matches!(self, Self::ScribeThrottledErrorPayload(_))
    }

    pub fn is_scribe_unaccepted_terms_error_payload(&self) -> bool {
        matches!(self, Self::ScribeUnacceptedTermsErrorPayload(_))
    }

    pub fn is_scribe_rate_limited_error_payload(&self) -> bool {
        matches!(self, Self::ScribeRateLimitedErrorPayload(_))
    }

    pub fn is_scribe_queue_overflow_error_payload(&self) -> bool {
        matches!(self, Self::ScribeQueueOverflowErrorPayload(_))
    }

    pub fn is_scribe_resource_exhausted_error_payload(&self) -> bool {
        matches!(self, Self::ScribeResourceExhaustedErrorPayload(_))
    }

    pub fn is_scribe_session_time_limit_exceeded_error_payload(&self) -> bool {
        matches!(self, Self::ScribeSessionTimeLimitExceededErrorPayload(_))
    }

    pub fn is_scribe_input_error_payload(&self) -> bool {
        matches!(self, Self::ScribeInputErrorPayload(_))
    }

    pub fn is_scribe_chunk_size_exceeded_error_payload(&self) -> bool {
        matches!(self, Self::ScribeChunkSizeExceededErrorPayload(_))
    }

    pub fn is_scribe_insufficient_audio_activity_error_payload(&self) -> bool {
        matches!(self, Self::ScribeInsufficientAudioActivityErrorPayload(_))
    }

    pub fn is_scribe_transcriber_error_payload(&self) -> bool {
        matches!(self, Self::ScribeTranscriberErrorPayload(_))
    }


    pub fn as_session_started_payload(&self) -> Option<&SessionStartedPayload> {
        match self {
                    Self::SessionStartedPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_session_started_payload(self) -> Option<SessionStartedPayload> {
        match self {
                    Self::SessionStartedPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_partial_transcript_payload(&self) -> Option<&PartialTranscriptPayload> {
        match self {
                    Self::PartialTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_partial_transcript_payload(self) -> Option<PartialTranscriptPayload> {
        match self {
                    Self::PartialTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_final_transcript(&self) -> Option<&FinalTranscript> {
        match self {
                    Self::FinalTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_final_transcript(self) -> Option<FinalTranscript> {
        match self {
                    Self::FinalTranscript(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_final_transcript_with_timestamps(&self) -> Option<&FinalTranscriptWithTimestamps> {
        match self {
                    Self::FinalTranscriptWithTimestamps(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_final_transcript_with_timestamps(self) -> Option<FinalTranscriptWithTimestamps> {
        match self {
                    Self::FinalTranscriptWithTimestamps(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_committed_transcript_payload(&self) -> Option<&CommittedTranscriptPayload> {
        match self {
                    Self::CommittedTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_committed_transcript_payload(self) -> Option<CommittedTranscriptPayload> {
        match self {
                    Self::CommittedTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_committed_transcript_with_timestamps_payload(&self) -> Option<&CommittedTranscriptWithTimestampsPayload> {
        match self {
                    Self::CommittedTranscriptWithTimestampsPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_committed_transcript_with_timestamps_payload(self) -> Option<CommittedTranscriptWithTimestampsPayload> {
        match self {
                    Self::CommittedTranscriptWithTimestampsPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_committed_transcript_entities_payload(&self) -> Option<&CommittedTranscriptEntitiesPayload> {
        match self {
                    Self::CommittedTranscriptEntitiesPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_committed_transcript_entities_payload(self) -> Option<CommittedTranscriptEntitiesPayload> {
        match self {
                    Self::CommittedTranscriptEntitiesPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_error_payload(&self) -> Option<&ScribeErrorPayload> {
        match self {
                    Self::ScribeErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_error_payload(self) -> Option<ScribeErrorPayload> {
        match self {
                    Self::ScribeErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_auth_error_payload(&self) -> Option<&ScribeAuthErrorPayload> {
        match self {
                    Self::ScribeAuthErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_auth_error_payload(self) -> Option<ScribeAuthErrorPayload> {
        match self {
                    Self::ScribeAuthErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_quota_exceeded_error_payload(&self) -> Option<&ScribeQuotaExceededErrorPayload> {
        match self {
                    Self::ScribeQuotaExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_quota_exceeded_error_payload(self) -> Option<ScribeQuotaExceededErrorPayload> {
        match self {
                    Self::ScribeQuotaExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_throttled_error_payload(&self) -> Option<&ScribeThrottledErrorPayload> {
        match self {
                    Self::ScribeThrottledErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_throttled_error_payload(self) -> Option<ScribeThrottledErrorPayload> {
        match self {
                    Self::ScribeThrottledErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_unaccepted_terms_error_payload(&self) -> Option<&ScribeUnacceptedTermsErrorPayload> {
        match self {
                    Self::ScribeUnacceptedTermsErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_unaccepted_terms_error_payload(self) -> Option<ScribeUnacceptedTermsErrorPayload> {
        match self {
                    Self::ScribeUnacceptedTermsErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_rate_limited_error_payload(&self) -> Option<&ScribeRateLimitedErrorPayload> {
        match self {
                    Self::ScribeRateLimitedErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_rate_limited_error_payload(self) -> Option<ScribeRateLimitedErrorPayload> {
        match self {
                    Self::ScribeRateLimitedErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_queue_overflow_error_payload(&self) -> Option<&ScribeQueueOverflowErrorPayload> {
        match self {
                    Self::ScribeQueueOverflowErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_queue_overflow_error_payload(self) -> Option<ScribeQueueOverflowErrorPayload> {
        match self {
                    Self::ScribeQueueOverflowErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_resource_exhausted_error_payload(&self) -> Option<&ScribeResourceExhaustedErrorPayload> {
        match self {
                    Self::ScribeResourceExhaustedErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_resource_exhausted_error_payload(self) -> Option<ScribeResourceExhaustedErrorPayload> {
        match self {
                    Self::ScribeResourceExhaustedErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_session_time_limit_exceeded_error_payload(&self) -> Option<&ScribeSessionTimeLimitExceededErrorPayload> {
        match self {
                    Self::ScribeSessionTimeLimitExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_session_time_limit_exceeded_error_payload(self) -> Option<ScribeSessionTimeLimitExceededErrorPayload> {
        match self {
                    Self::ScribeSessionTimeLimitExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_input_error_payload(&self) -> Option<&ScribeInputErrorPayload> {
        match self {
                    Self::ScribeInputErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_input_error_payload(self) -> Option<ScribeInputErrorPayload> {
        match self {
                    Self::ScribeInputErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_chunk_size_exceeded_error_payload(&self) -> Option<&ScribeChunkSizeExceededErrorPayload> {
        match self {
                    Self::ScribeChunkSizeExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_chunk_size_exceeded_error_payload(self) -> Option<ScribeChunkSizeExceededErrorPayload> {
        match self {
                    Self::ScribeChunkSizeExceededErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_insufficient_audio_activity_error_payload(&self) -> Option<&ScribeInsufficientAudioActivityErrorPayload> {
        match self {
                    Self::ScribeInsufficientAudioActivityErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_insufficient_audio_activity_error_payload(self) -> Option<ScribeInsufficientAudioActivityErrorPayload> {
        match self {
                    Self::ScribeInsufficientAudioActivityErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_scribe_transcriber_error_payload(&self) -> Option<&ScribeTranscriberErrorPayload> {
        match self {
                    Self::ScribeTranscriberErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_scribe_transcriber_error_payload(self) -> Option<ScribeTranscriberErrorPayload> {
        match self {
                    Self::ScribeTranscriberErrorPayload(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveTranscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionStartedPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PartialTranscriptPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FinalTranscript(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::FinalTranscriptWithTimestamps(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CommittedTranscriptPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CommittedTranscriptWithTimestampsPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CommittedTranscriptEntitiesPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeAuthErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeQuotaExceededErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeThrottledErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeUnacceptedTermsErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeRateLimitedErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeQueueOverflowErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeResourceExhaustedErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeSessionTimeLimitExceededErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeInputErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeChunkSizeExceededErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeInsufficientAudioActivityErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ScribeTranscriberErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
