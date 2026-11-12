pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How a settled call's decision left its lead.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecisionDisposition {
    Completed,
    RetryQueued,
    CallbackQueued,
    RedirectQueued,
    Attention,
    Cancelled,
    DoNotCall,
    Suppressed,
    WindowSuspended,
    AttemptCeilingReached,
    CallbackCeilingReached,
    RedirectCeilingReached,
    CallbackTimeInvalid,
    RedirectNumberInvalid,
    CouldntProcess,
    SuppressedDoNotCall,
    SuppressedAfterStop,
    PermissionNotGranted,
    PermissionDeclined,
    PermissionRequestBlocked,
    PermissionExpired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DecisionDisposition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Completed => serializer.serialize_str("completed"),
            Self::RetryQueued => serializer.serialize_str("retry_queued"),
            Self::CallbackQueued => serializer.serialize_str("callback_queued"),
            Self::RedirectQueued => serializer.serialize_str("redirect_queued"),
            Self::Attention => serializer.serialize_str("attention"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::DoNotCall => serializer.serialize_str("do_not_call"),
            Self::Suppressed => serializer.serialize_str("suppressed"),
            Self::WindowSuspended => serializer.serialize_str("window_suspended"),
            Self::AttemptCeilingReached => serializer.serialize_str("attempt_ceiling_reached"),
            Self::CallbackCeilingReached => serializer.serialize_str("callback_ceiling_reached"),
            Self::RedirectCeilingReached => serializer.serialize_str("redirect_ceiling_reached"),
            Self::CallbackTimeInvalid => serializer.serialize_str("callback_time_invalid"),
            Self::RedirectNumberInvalid => serializer.serialize_str("redirect_number_invalid"),
            Self::CouldntProcess => serializer.serialize_str("couldnt_process"),
            Self::SuppressedDoNotCall => serializer.serialize_str("suppressed_do_not_call"),
            Self::SuppressedAfterStop => serializer.serialize_str("suppressed_after_stop"),
            Self::PermissionNotGranted => serializer.serialize_str("permission_not_granted"),
            Self::PermissionDeclined => serializer.serialize_str("permission_declined"),
            Self::PermissionRequestBlocked => serializer.serialize_str("permission_request_blocked"),
            Self::PermissionExpired => serializer.serialize_str("permission_expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DecisionDisposition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "completed" => Ok(Self::Completed),
            "retry_queued" => Ok(Self::RetryQueued),
            "callback_queued" => Ok(Self::CallbackQueued),
            "redirect_queued" => Ok(Self::RedirectQueued),
            "attention" => Ok(Self::Attention),
            "cancelled" => Ok(Self::Cancelled),
            "do_not_call" => Ok(Self::DoNotCall),
            "suppressed" => Ok(Self::Suppressed),
            "window_suspended" => Ok(Self::WindowSuspended),
            "attempt_ceiling_reached" => Ok(Self::AttemptCeilingReached),
            "callback_ceiling_reached" => Ok(Self::CallbackCeilingReached),
            "redirect_ceiling_reached" => Ok(Self::RedirectCeilingReached),
            "callback_time_invalid" => Ok(Self::CallbackTimeInvalid),
            "redirect_number_invalid" => Ok(Self::RedirectNumberInvalid),
            "couldnt_process" => Ok(Self::CouldntProcess),
            "suppressed_do_not_call" => Ok(Self::SuppressedDoNotCall),
            "suppressed_after_stop" => Ok(Self::SuppressedAfterStop),
            "permission_not_granted" => Ok(Self::PermissionNotGranted),
            "permission_declined" => Ok(Self::PermissionDeclined),
            "permission_request_blocked" => Ok(Self::PermissionRequestBlocked),
            "permission_expired" => Ok(Self::PermissionExpired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DecisionDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completed => write!(f, "completed"),
            Self::RetryQueued => write!(f, "retry_queued"),
            Self::CallbackQueued => write!(f, "callback_queued"),
            Self::RedirectQueued => write!(f, "redirect_queued"),
            Self::Attention => write!(f, "attention"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::DoNotCall => write!(f, "do_not_call"),
            Self::Suppressed => write!(f, "suppressed"),
            Self::WindowSuspended => write!(f, "window_suspended"),
            Self::AttemptCeilingReached => write!(f, "attempt_ceiling_reached"),
            Self::CallbackCeilingReached => write!(f, "callback_ceiling_reached"),
            Self::RedirectCeilingReached => write!(f, "redirect_ceiling_reached"),
            Self::CallbackTimeInvalid => write!(f, "callback_time_invalid"),
            Self::RedirectNumberInvalid => write!(f, "redirect_number_invalid"),
            Self::CouldntProcess => write!(f, "couldnt_process"),
            Self::SuppressedDoNotCall => write!(f, "suppressed_do_not_call"),
            Self::SuppressedAfterStop => write!(f, "suppressed_after_stop"),
            Self::PermissionNotGranted => write!(f, "permission_not_granted"),
            Self::PermissionDeclined => write!(f, "permission_declined"),
            Self::PermissionRequestBlocked => write!(f, "permission_request_blocked"),
            Self::PermissionExpired => write!(f, "permission_expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
