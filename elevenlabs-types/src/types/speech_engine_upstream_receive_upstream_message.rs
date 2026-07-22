pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ReceiveUpstreamMessage {
        InitPayload(InitPayload),

        UserTranscriptPayload(UserTranscriptPayload),

        PingPayload(PingPayload),

        ClosePayload(ClosePayload),

        ErrorPayload(ErrorPayload),
}

impl ReceiveUpstreamMessage {
    pub fn is_init_payload(&self) -> bool {
        matches!(self, Self::InitPayload(_))
    }

    pub fn is_user_transcript_payload(&self) -> bool {
        matches!(self, Self::UserTranscriptPayload(_))
    }

    pub fn is_ping_payload(&self) -> bool {
        matches!(self, Self::PingPayload(_))
    }

    pub fn is_close_payload(&self) -> bool {
        matches!(self, Self::ClosePayload(_))
    }

    pub fn is_error_payload(&self) -> bool {
        matches!(self, Self::ErrorPayload(_))
    }


    pub fn as_init_payload(&self) -> Option<&InitPayload> {
        match self {
                    Self::InitPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_init_payload(self) -> Option<InitPayload> {
        match self {
                    Self::InitPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_user_transcript_payload(&self) -> Option<&UserTranscriptPayload> {
        match self {
                    Self::UserTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_user_transcript_payload(self) -> Option<UserTranscriptPayload> {
        match self {
                    Self::UserTranscriptPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_ping_payload(&self) -> Option<&PingPayload> {
        match self {
                    Self::PingPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_ping_payload(self) -> Option<PingPayload> {
        match self {
                    Self::PingPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_close_payload(&self) -> Option<&ClosePayload> {
        match self {
                    Self::ClosePayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_close_payload(self) -> Option<ClosePayload> {
        match self {
                    Self::ClosePayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_error_payload(&self) -> Option<&ErrorPayload> {
        match self {
                    Self::ErrorPayload(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_error_payload(self) -> Option<ErrorPayload> {
        match self {
                    Self::ErrorPayload(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReceiveUpstreamMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UserTranscriptPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PingPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ClosePayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ErrorPayload(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
