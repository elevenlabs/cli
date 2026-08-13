pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ImageReference {
        #[serde(rename = "asset")]
        #[non_exhaustive]
        Asset {
            #[serde(flatten)]
            data: StaticAssetReference,
        },

        #[serde(rename = "generation")]
        #[non_exhaustive]
        Generation {
            #[serde(flatten)]
            data: GenerationReference,
        },

        #[serde(rename = "inline_base64")]
        #[non_exhaustive]
        InlineBase64 {
            #[serde(default)]
            content_base64: String,
            mime_type: InlineImageReferenceMimeType,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ImageReference {
    pub fn asset(data: StaticAssetReference) -> Self {
        Self::Asset { data }
    }

    pub fn generation(data: GenerationReference) -> Self {
        Self::Generation { data }
    }

    pub fn inline_base64(content_base64: String, mime_type: InlineImageReferenceMimeType) -> Self {
        Self::InlineBase64 { content_base64, mime_type }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
