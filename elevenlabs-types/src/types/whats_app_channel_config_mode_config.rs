pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode")]
#[non_exhaustive]
pub enum WhatsAppChannelConfigModeConfig {
        #[serde(rename = "message")]
        #[non_exhaustive]
        Message {
            #[serde(default)]
            template: WhatsAppTemplate,
        },

        #[serde(rename = "voice")]
        #[non_exhaustive]
        Voice {
            #[serde(flatten)]
            data: WhatsAppVoiceConfig,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WhatsAppChannelConfigModeConfig {
    pub fn message(template: WhatsAppTemplate) -> Self {
        Self::Message { template }
    }

    pub fn voice(data: WhatsAppVoiceConfig) -> Self {
        Self::Voice { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
