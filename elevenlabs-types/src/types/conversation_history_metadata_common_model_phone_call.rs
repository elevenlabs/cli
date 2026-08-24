pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ConversationHistoryMetadataCommonModelPhoneCall {
        #[serde(rename = "exotel")]
        #[non_exhaustive]
        Exotel {
            direction: TelephonyDirection,
            #[serde(default)]
            phone_number_id: String,
            #[serde(default)]
            agent_number: String,
            #[serde(default)]
            external_number: String,
            #[serde(default)]
            stream_sid: String,
            #[serde(default)]
            call_sid: String,
        },

        #[serde(rename = "sip_trunking")]
        #[non_exhaustive]
        SipTrunking {
            direction: TelephonyDirection,
            #[serde(default)]
            phone_number_id: String,
            #[serde(default)]
            agent_number: String,
            #[serde(default)]
            external_number: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            call_id: Option<String>,
            #[serde(default)]
            call_sid: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            sip_header_dynamic_variables: Option<HashMap<String, String>>,
        },

        #[serde(rename = "twilio")]
        #[non_exhaustive]
        Twilio {
            direction: TelephonyDirection,
            #[serde(default)]
            phone_number_id: String,
            #[serde(default)]
            agent_number: String,
            #[serde(default)]
            external_number: String,
            #[serde(default)]
            stream_sid: String,
            #[serde(default)]
            call_sid: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ConversationHistoryMetadataCommonModelPhoneCall {
    pub fn exotel(direction: TelephonyDirection, phone_number_id: String, agent_number: String, external_number: String, stream_sid: String, call_sid: String) -> Self {
        Self::Exotel { direction, phone_number_id, agent_number, external_number, stream_sid, call_sid }
    }

    pub fn sip_trunking(direction: TelephonyDirection, phone_number_id: String, agent_number: String, external_number: String, call_sid: String) -> Self {
        Self::SipTrunking { direction, phone_number_id, agent_number, external_number, call_id: None, call_sid, sip_header_dynamic_variables: None }
    }

    pub fn twilio(direction: TelephonyDirection, phone_number_id: String, agent_number: String, external_number: String, stream_sid: String, call_sid: String) -> Self {
        Self::Twilio { direction, phone_number_id, agent_number, external_number, stream_sid, call_sid }
    }

    pub fn sip_trunking_with_call_id(direction: TelephonyDirection, phone_number_id: String, agent_number: String, external_number: String, call_id: String, call_sid: String, sip_header_dynamic_variables: Option<HashMap<String, String>>) -> Self {
        Self::SipTrunking { direction, phone_number_id, agent_number, external_number, call_id: Some(call_id), call_sid, sip_header_dynamic_variables }
    }

    pub fn sip_trunking_with_sip_header_dynamic_variables(direction: TelephonyDirection, phone_number_id: String, agent_number: String, external_number: String, call_id: Option<String>, call_sid: String, sip_header_dynamic_variables: HashMap<String, String>) -> Self {
        Self::SipTrunking { direction, phone_number_id, agent_number, external_number, call_id, call_sid, sip_header_dynamic_variables: Some(sip_header_dynamic_variables) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
