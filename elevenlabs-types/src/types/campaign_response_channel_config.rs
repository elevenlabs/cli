pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "channel")]
#[non_exhaustive]
pub enum CampaignResponseChannelConfig {
        #[serde(rename = "phone")]
        #[non_exhaustive]
        Phone {
            #[serde(default)]
            agent_phone_number_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            mode_config: Option<PhoneVoiceConfigOutput>,
        },

        #[serde(rename = "whatsapp")]
        #[non_exhaustive]
        Whatsapp {
            #[serde(default)]
            whatsapp_phone_number_id: String,
            mode_config: WhatsAppChannelConfigModeConfig,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl CampaignResponseChannelConfig {
    pub fn phone(agent_phone_number_id: String) -> Self {
        Self::Phone { agent_phone_number_id, mode_config: None }
    }

    pub fn whatsapp(whatsapp_phone_number_id: String, mode_config: WhatsAppChannelConfigModeConfig) -> Self {
        Self::Whatsapp { whatsapp_phone_number_id, mode_config }
    }

    pub fn phone_with_mode_config(agent_phone_number_id: String, mode_config: PhoneVoiceConfigOutput) -> Self {
        Self::Phone { agent_phone_number_id, mode_config: Some(mode_config) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
