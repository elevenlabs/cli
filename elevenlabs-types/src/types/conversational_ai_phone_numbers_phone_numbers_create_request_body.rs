pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "provider")]
#[non_exhaustive]
pub enum PhoneNumbersCreateRequestBody {
        #[serde(rename = "twilio")]
        #[non_exhaustive]
        Twilio {
            #[serde(default)]
            phone_number: String,
            #[serde(default)]
            label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_inbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_outbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            agent_id: Option<String>,
            #[serde(default)]
            sid: String,
            #[serde(default)]
            token: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            region_config: Option<RegionConfigRequest>,
            #[serde(skip_serializing_if = "Option::is_none")]
            enable_sms: Option<bool>,
        },

        #[serde(rename = "exotel")]
        #[non_exhaustive]
        Exotel {
            #[serde(default)]
            phone_number: String,
            #[serde(default)]
            label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_inbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_outbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            agent_id: Option<String>,
            #[serde(default)]
            account_sid: String,
            #[serde(default)]
            api_key: String,
            #[serde(default)]
            api_token: String,
            api_subdomain: ExotelApiSubdomain,
            #[serde(default)]
            app_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            applet_url: Option<String>,
        },

        #[serde(rename = "sip_trunk")]
        #[non_exhaustive]
        SipTrunk {
            #[serde(default)]
            phone_number: String,
            #[serde(default)]
            label: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_inbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            supports_outbound: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            agent_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>,
            #[serde(skip_serializing_if = "Option::is_none")]
            outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl PhoneNumbersCreateRequestBody {
    pub fn twilio(phone_number: String, label: String, sid: String, token: String) -> Self {
        Self::Twilio { phone_number, label, supports_inbound: None, supports_outbound: None, agent_id: None, sid, token, region_config: None, enable_sms: None }
    }

    pub fn exotel(phone_number: String, label: String, account_sid: String, api_key: String, api_token: String, api_subdomain: ExotelApiSubdomain, app_id: String) -> Self {
        Self::Exotel { phone_number, label, supports_inbound: None, supports_outbound: None, agent_id: None, account_sid, api_key, api_token, api_subdomain, app_id, applet_url: None }
    }

    pub fn sip_trunk(phone_number: String, label: String) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound: None, supports_outbound: None, agent_id: None, inbound_trunk_config: None, outbound_trunk_config: None }
    }

    pub fn twilio_with_supports_inbound(phone_number: String, label: String, supports_inbound: bool, supports_outbound: Option<bool>, agent_id: Option<String>, sid: String, token: String, region_config: Option<RegionConfigRequest>, enable_sms: Option<bool>) -> Self {
        Self::Twilio { phone_number, label, supports_inbound: Some(supports_inbound), supports_outbound, agent_id, sid, token, region_config, enable_sms }
    }

    pub fn twilio_with_supports_outbound(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: bool, agent_id: Option<String>, sid: String, token: String, region_config: Option<RegionConfigRequest>, enable_sms: Option<bool>) -> Self {
        Self::Twilio { phone_number, label, supports_inbound, supports_outbound: Some(supports_outbound), agent_id, sid, token, region_config, enable_sms }
    }

    pub fn twilio_with_agent_id(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: String, sid: String, token: String, region_config: Option<RegionConfigRequest>, enable_sms: Option<bool>) -> Self {
        Self::Twilio { phone_number, label, supports_inbound, supports_outbound, agent_id: Some(agent_id), sid, token, region_config, enable_sms }
    }

    pub fn twilio_with_region_config(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: Option<String>, sid: String, token: String, region_config: RegionConfigRequest, enable_sms: Option<bool>) -> Self {
        Self::Twilio { phone_number, label, supports_inbound, supports_outbound, agent_id, sid, token, region_config: Some(region_config), enable_sms }
    }

    pub fn twilio_with_enable_sms(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: Option<String>, sid: String, token: String, region_config: Option<RegionConfigRequest>, enable_sms: bool) -> Self {
        Self::Twilio { phone_number, label, supports_inbound, supports_outbound, agent_id, sid, token, region_config, enable_sms: Some(enable_sms) }
    }

    pub fn exotel_with_supports_inbound(phone_number: String, label: String, supports_inbound: bool, supports_outbound: Option<bool>, agent_id: Option<String>, account_sid: String, api_key: String, api_token: String, api_subdomain: ExotelApiSubdomain, app_id: String, applet_url: Option<String>) -> Self {
        Self::Exotel { phone_number, label, supports_inbound: Some(supports_inbound), supports_outbound, agent_id, account_sid, api_key, api_token, api_subdomain, app_id, applet_url }
    }

    pub fn exotel_with_supports_outbound(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: bool, agent_id: Option<String>, account_sid: String, api_key: String, api_token: String, api_subdomain: ExotelApiSubdomain, app_id: String, applet_url: Option<String>) -> Self {
        Self::Exotel { phone_number, label, supports_inbound, supports_outbound: Some(supports_outbound), agent_id, account_sid, api_key, api_token, api_subdomain, app_id, applet_url }
    }

    pub fn exotel_with_agent_id(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: String, account_sid: String, api_key: String, api_token: String, api_subdomain: ExotelApiSubdomain, app_id: String, applet_url: Option<String>) -> Self {
        Self::Exotel { phone_number, label, supports_inbound, supports_outbound, agent_id: Some(agent_id), account_sid, api_key, api_token, api_subdomain, app_id, applet_url }
    }

    pub fn exotel_with_applet_url(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: Option<String>, account_sid: String, api_key: String, api_token: String, api_subdomain: ExotelApiSubdomain, app_id: String, applet_url: String) -> Self {
        Self::Exotel { phone_number, label, supports_inbound, supports_outbound, agent_id, account_sid, api_key, api_token, api_subdomain, app_id, applet_url: Some(applet_url) }
    }

    pub fn sip_trunk_with_supports_inbound(phone_number: String, label: String, supports_inbound: bool, supports_outbound: Option<bool>, agent_id: Option<String>, inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>, outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound: Some(supports_inbound), supports_outbound, agent_id, inbound_trunk_config, outbound_trunk_config }
    }

    pub fn sip_trunk_with_supports_outbound(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: bool, agent_id: Option<String>, inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>, outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound, supports_outbound: Some(supports_outbound), agent_id, inbound_trunk_config, outbound_trunk_config }
    }

    pub fn sip_trunk_with_agent_id(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: String, inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>, outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound, supports_outbound, agent_id: Some(agent_id), inbound_trunk_config, outbound_trunk_config }
    }

    pub fn sip_trunk_with_inbound_trunk_config(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: Option<String>, inbound_trunk_config: InboundSipTrunkConfigRequestModel, outbound_trunk_config: Option<OutboundSipTrunkConfigRequestModel>) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound, supports_outbound, agent_id, inbound_trunk_config: Some(inbound_trunk_config), outbound_trunk_config }
    }

    pub fn sip_trunk_with_outbound_trunk_config(phone_number: String, label: String, supports_inbound: Option<bool>, supports_outbound: Option<bool>, agent_id: Option<String>, inbound_trunk_config: Option<InboundSipTrunkConfigRequestModel>, outbound_trunk_config: OutboundSipTrunkConfigRequestModel) -> Self {
        Self::SipTrunk { phone_number, label, supports_inbound, supports_outbound, agent_id, inbound_trunk_config, outbound_trunk_config: Some(outbound_trunk_config) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
