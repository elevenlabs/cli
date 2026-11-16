pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SipTransferInputConfig {
        #[serde(rename = "sip_cold_conference_transfer")]
        #[non_exhaustive]
        SipColdConferenceTransfer {
            transfer_destination: SipColdConferenceTransferConfigInputTransferDestination,
        },

        #[serde(rename = "sip_cold_refer_transfer")]
        #[non_exhaustive]
        SipColdReferTransfer {
            #[serde(skip_serializing_if = "Option::is_none")]
            custom_sip_headers: Option<Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>>,
            transfer_destination: SipColdReferTransferConfigInputTransferDestination,
            #[serde(skip_serializing_if = "Option::is_none")]
            uui: Option<UuiTransferConfig>,
            #[serde(skip_serializing_if = "Option::is_none")]
            play_dialtone: Option<bool>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SipTransferInputConfig {
    pub fn sip_cold_conference_transfer(transfer_destination: SipColdConferenceTransferConfigInputTransferDestination) -> Self {
        Self::SipColdConferenceTransfer { transfer_destination }
    }

    pub fn sip_cold_refer_transfer(transfer_destination: SipColdReferTransferConfigInputTransferDestination) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers: None, transfer_destination, uui: None, play_dialtone: None }
    }

    pub fn sip_cold_refer_transfer_with_custom_sip_headers(custom_sip_headers: Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>, transfer_destination: SipColdReferTransferConfigInputTransferDestination, uui: Option<UuiTransferConfig>, play_dialtone: Option<bool>) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers: Some(custom_sip_headers), transfer_destination, uui, play_dialtone }
    }

    pub fn sip_cold_refer_transfer_with_uui(custom_sip_headers: Option<Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>>, transfer_destination: SipColdReferTransferConfigInputTransferDestination, uui: UuiTransferConfig, play_dialtone: Option<bool>) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers, transfer_destination, uui: Some(uui), play_dialtone }
    }

    pub fn sip_cold_refer_transfer_with_play_dialtone(custom_sip_headers: Option<Vec<SipColdReferTransferConfigInputCustomSipHeadersItem>>, transfer_destination: SipColdReferTransferConfigInputTransferDestination, uui: Option<UuiTransferConfig>, play_dialtone: bool) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers, transfer_destination, uui, play_dialtone: Some(play_dialtone) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
