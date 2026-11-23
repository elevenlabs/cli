pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TwilioTransferInputConfig {
        #[serde(rename = "twilio_cold_blind_transfer")]
        #[non_exhaustive]
        TwilioColdBlindTransfer {
            transfer_destination: TwilioColdBlindTransferConfigInputTransferDestination,
            #[serde(skip_serializing_if = "Option::is_none")]
            post_dial_digits: Option<TwilioColdBlindTransferConfigInputPostDialDigits>,
        },

        #[serde(rename = "twilio_warm_conference_transfer")]
        #[non_exhaustive]
        TwilioWarmConferenceTransfer {
            transfer_destination: TwilioWarmConferenceTransferConfigInputTransferDestination,
            #[serde(skip_serializing_if = "Option::is_none")]
            post_dial_digits: Option<TwilioWarmConferenceTransferConfigInputPostDialDigits>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TwilioTransferInputConfig {
    pub fn twilio_cold_blind_transfer(transfer_destination: TwilioColdBlindTransferConfigInputTransferDestination) -> Self {
        Self::TwilioColdBlindTransfer { transfer_destination, post_dial_digits: None }
    }

    pub fn twilio_warm_conference_transfer(transfer_destination: TwilioWarmConferenceTransferConfigInputTransferDestination) -> Self {
        Self::TwilioWarmConferenceTransfer { transfer_destination, post_dial_digits: None }
    }

    pub fn twilio_cold_blind_transfer_with_post_dial_digits(transfer_destination: TwilioColdBlindTransferConfigInputTransferDestination, post_dial_digits: TwilioColdBlindTransferConfigInputPostDialDigits) -> Self {
        Self::TwilioColdBlindTransfer { transfer_destination, post_dial_digits: Some(post_dial_digits) }
    }

    pub fn twilio_warm_conference_transfer_with_post_dial_digits(transfer_destination: TwilioWarmConferenceTransferConfigInputTransferDestination, post_dial_digits: TwilioWarmConferenceTransferConfigInputPostDialDigits) -> Self {
        Self::TwilioWarmConferenceTransfer { transfer_destination, post_dial_digits: Some(post_dial_digits) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
