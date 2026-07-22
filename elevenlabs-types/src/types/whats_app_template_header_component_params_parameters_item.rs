pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WhatsAppTemplateHeaderComponentParamsParametersItem {
        #[serde(rename = "document")]
        #[non_exhaustive]
        Document {
            #[serde(default)]
            document: WhatsAppTemplateDocumentParamDetails,
        },

        #[serde(rename = "image")]
        #[non_exhaustive]
        Image {
            #[serde(default)]
            image: WhatsAppTemplateImageParamDetails,
        },

        #[serde(rename = "location")]
        #[non_exhaustive]
        Location {
            #[serde(default)]
            location: WhatsAppTemplateLocationParamDetails,
        },

        #[serde(rename = "text")]
        #[non_exhaustive]
        Text {
            #[serde(flatten)]
            data: WhatsAppTemplateTextParam,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WhatsAppTemplateHeaderComponentParamsParametersItem {
    pub fn document(document: WhatsAppTemplateDocumentParamDetails) -> Self {
        Self::Document { document }
    }

    pub fn image(image: WhatsAppTemplateImageParamDetails) -> Self {
        Self::Image { image }
    }

    pub fn location(location: WhatsAppTemplateLocationParamDetails) -> Self {
        Self::Location { location }
    }

    pub fn text(data: WhatsAppTemplateTextParam) -> Self {
        Self::Text { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
