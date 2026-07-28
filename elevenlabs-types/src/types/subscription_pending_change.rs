pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SubscriptionPendingChange {
        PendingSubscriptionSwitchResponseModel(PendingSubscriptionSwitchResponseModel),

        PendingCancellationResponseModel(PendingCancellationResponseModel),
}

impl SubscriptionPendingChange {
    pub fn is_pending_subscription_switch_response_model(&self) -> bool {
        matches!(self, Self::PendingSubscriptionSwitchResponseModel(_))
    }

    pub fn is_pending_cancellation_response_model(&self) -> bool {
        matches!(self, Self::PendingCancellationResponseModel(_))
    }


    pub fn as_pending_subscription_switch_response_model(&self) -> Option<&PendingSubscriptionSwitchResponseModel> {
        match self {
                    Self::PendingSubscriptionSwitchResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_pending_subscription_switch_response_model(self) -> Option<PendingSubscriptionSwitchResponseModel> {
        match self {
                    Self::PendingSubscriptionSwitchResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_pending_cancellation_response_model(&self) -> Option<&PendingCancellationResponseModel> {
        match self {
                    Self::PendingCancellationResponseModel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_pending_cancellation_response_model(self) -> Option<PendingCancellationResponseModel> {
        match self {
                    Self::PendingCancellationResponseModel(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for SubscriptionPendingChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PendingSubscriptionSwitchResponseModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::PendingCancellationResponseModel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
