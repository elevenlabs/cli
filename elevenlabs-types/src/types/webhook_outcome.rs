pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookOutcome {
    Delivered,
    FailedRetryScheduled,
    FailedExhausted,
    FailedNoRetry,
    FailedRetryError,
    DroppedQueueFull,
    DroppedPayloadTooLarge,
    SkippedDisabled,
    NotSentConfiguration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::FailedRetryScheduled => serializer.serialize_str("failed_retry_scheduled"),
            Self::FailedExhausted => serializer.serialize_str("failed_exhausted"),
            Self::FailedNoRetry => serializer.serialize_str("failed_no_retry"),
            Self::FailedRetryError => serializer.serialize_str("failed_retry_error"),
            Self::DroppedQueueFull => serializer.serialize_str("dropped_queue_full"),
            Self::DroppedPayloadTooLarge => serializer.serialize_str("dropped_payload_too_large"),
            Self::SkippedDisabled => serializer.serialize_str("skipped_disabled"),
            Self::NotSentConfiguration => serializer.serialize_str("not_sent_configuration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "delivered" => Ok(Self::Delivered),
            "failed_retry_scheduled" => Ok(Self::FailedRetryScheduled),
            "failed_exhausted" => Ok(Self::FailedExhausted),
            "failed_no_retry" => Ok(Self::FailedNoRetry),
            "failed_retry_error" => Ok(Self::FailedRetryError),
            "dropped_queue_full" => Ok(Self::DroppedQueueFull),
            "dropped_payload_too_large" => Ok(Self::DroppedPayloadTooLarge),
            "skipped_disabled" => Ok(Self::SkippedDisabled),
            "not_sent_configuration" => Ok(Self::NotSentConfiguration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Delivered => write!(f, "delivered"),
            Self::FailedRetryScheduled => write!(f, "failed_retry_scheduled"),
            Self::FailedExhausted => write!(f, "failed_exhausted"),
            Self::FailedNoRetry => write!(f, "failed_no_retry"),
            Self::FailedRetryError => write!(f, "failed_retry_error"),
            Self::DroppedQueueFull => write!(f, "dropped_queue_full"),
            Self::DroppedPayloadTooLarge => write!(f, "dropped_payload_too_large"),
            Self::SkippedDisabled => write!(f, "skipped_disabled"),
            Self::NotSentConfiguration => write!(f, "not_sent_configuration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
