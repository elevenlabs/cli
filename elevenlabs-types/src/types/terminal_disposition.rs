pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerminalDisposition {
    CampaignEnded,
    Removed,
    DoNotCall,
    Completed,
    Cancelled,
    Suppressed,
    AttemptCeilingReached,
    CallbackCeilingReached,
    RedirectCeilingReached,
    PermissionNotGranted,
    PermissionDeclined,
    PermissionRequestBlocked,
    PermissionExpired,
    CallbackTimeInvalid,
    RedirectNumberInvalid,
    CouldntProcess,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TerminalDisposition {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CampaignEnded => serializer.serialize_str("campaign_ended"),
            Self::Removed => serializer.serialize_str("removed"),
            Self::DoNotCall => serializer.serialize_str("do_not_call"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Suppressed => serializer.serialize_str("suppressed"),
            Self::AttemptCeilingReached => serializer.serialize_str("attempt_ceiling_reached"),
            Self::CallbackCeilingReached => serializer.serialize_str("callback_ceiling_reached"),
            Self::RedirectCeilingReached => serializer.serialize_str("redirect_ceiling_reached"),
            Self::PermissionNotGranted => serializer.serialize_str("permission_not_granted"),
            Self::PermissionDeclined => serializer.serialize_str("permission_declined"),
            Self::PermissionRequestBlocked => serializer.serialize_str("permission_request_blocked"),
            Self::PermissionExpired => serializer.serialize_str("permission_expired"),
            Self::CallbackTimeInvalid => serializer.serialize_str("callback_time_invalid"),
            Self::RedirectNumberInvalid => serializer.serialize_str("redirect_number_invalid"),
            Self::CouldntProcess => serializer.serialize_str("couldnt_process"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TerminalDisposition {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "campaign_ended" => Ok(Self::CampaignEnded),
            "removed" => Ok(Self::Removed),
            "do_not_call" => Ok(Self::DoNotCall),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            "suppressed" => Ok(Self::Suppressed),
            "attempt_ceiling_reached" => Ok(Self::AttemptCeilingReached),
            "callback_ceiling_reached" => Ok(Self::CallbackCeilingReached),
            "redirect_ceiling_reached" => Ok(Self::RedirectCeilingReached),
            "permission_not_granted" => Ok(Self::PermissionNotGranted),
            "permission_declined" => Ok(Self::PermissionDeclined),
            "permission_request_blocked" => Ok(Self::PermissionRequestBlocked),
            "permission_expired" => Ok(Self::PermissionExpired),
            "callback_time_invalid" => Ok(Self::CallbackTimeInvalid),
            "redirect_number_invalid" => Ok(Self::RedirectNumberInvalid),
            "couldnt_process" => Ok(Self::CouldntProcess),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TerminalDisposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CampaignEnded => write!(f, "campaign_ended"),
            Self::Removed => write!(f, "removed"),
            Self::DoNotCall => write!(f, "do_not_call"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Suppressed => write!(f, "suppressed"),
            Self::AttemptCeilingReached => write!(f, "attempt_ceiling_reached"),
            Self::CallbackCeilingReached => write!(f, "callback_ceiling_reached"),
            Self::RedirectCeilingReached => write!(f, "redirect_ceiling_reached"),
            Self::PermissionNotGranted => write!(f, "permission_not_granted"),
            Self::PermissionDeclined => write!(f, "permission_declined"),
            Self::PermissionRequestBlocked => write!(f, "permission_request_blocked"),
            Self::PermissionExpired => write!(f, "permission_expired"),
            Self::CallbackTimeInvalid => write!(f, "callback_time_invalid"),
            Self::RedirectNumberInvalid => write!(f, "redirect_number_invalid"),
            Self::CouldntProcess => write!(f, "couldnt_process"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
