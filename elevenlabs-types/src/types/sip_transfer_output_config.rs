pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum SipTransferOutputConfig {
        #[serde(rename = "sip_cold_conference_transfer")]
        #[non_exhaustive]
        SipColdConferenceTransfer {
            transfer_destination: SipColdConferenceTransferConfigOutputTransferDestination,
        },

        #[serde(rename = "sip_cold_refer_transfer")]
        #[non_exhaustive]
        SipColdReferTransfer {
            #[serde(default)]
            custom_sip_headers: Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>,
            transfer_destination: SipColdReferTransferConfigOutputTransferDestination,
            #[serde(skip_serializing_if = "Option::is_none")]
            uui: Option<UuiTransferConfig>,
            #[serde(default)]
            play_dialtone: bool,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SipTransferOutputConfig {
    pub fn sip_cold_conference_transfer(transfer_destination: SipColdConferenceTransferConfigOutputTransferDestination) -> Self {
        Self::SipColdConferenceTransfer { transfer_destination }
    }

    pub fn sip_cold_refer_transfer(custom_sip_headers: Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>, transfer_destination: SipColdReferTransferConfigOutputTransferDestination, play_dialtone: bool) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers, transfer_destination, uui: None, play_dialtone }
    }

    pub fn sip_cold_refer_transfer_with_uui(custom_sip_headers: Vec<SipColdReferTransferConfigOutputCustomSipHeadersItem>, transfer_destination: SipColdReferTransferConfigOutputTransferDestination, uui: UuiTransferConfig, play_dialtone: bool) -> Self {
        Self::SipColdReferTransfer { custom_sip_headers, transfer_destination, uui: Some(uui), play_dialtone }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
