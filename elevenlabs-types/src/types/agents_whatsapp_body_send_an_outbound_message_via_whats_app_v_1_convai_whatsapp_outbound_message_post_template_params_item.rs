pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem {
        #[serde(rename = "body")]
        #[non_exhaustive]
        Body {
            #[serde(default)]
            parameters: Vec<WhatsAppTemplateTextParam>,
        },

        #[serde(rename = "button")]
        #[non_exhaustive]
        Button {
            #[serde(default)]
            parameters: Vec<WhatsAppTemplateTextParam>,
            #[serde(default)]
            index: i64,
            #[serde(default)]
            sub_type: String,
        },

        #[serde(rename = "header")]
        #[non_exhaustive]
        Header {
            #[serde(default)]
            parameters: Vec<WhatsAppTemplateHeaderComponentParamsParametersItem>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl BodySendAnOutboundMessageViaWhatsAppV1ConvaiWhatsappOutboundMessagePostTemplateParamsItem {
    pub fn body(parameters: Vec<WhatsAppTemplateTextParam>) -> Self {
        Self::Body { parameters }
    }

    pub fn button(parameters: Vec<WhatsAppTemplateTextParam>, index: i64, sub_type: String) -> Self {
        Self::Button { parameters, index, sub_type }
    }

    pub fn header(parameters: Vec<WhatsAppTemplateHeaderComponentParamsParametersItem>) -> Self {
        Self::Header { parameters }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
