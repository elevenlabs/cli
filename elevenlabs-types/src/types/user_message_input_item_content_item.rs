pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum UserMessageInputItemContentItem {
        #[serde(rename = "file")]
        #[non_exhaustive]
        File {
            source: FileContentPartSource,
        },

        #[serde(rename = "text")]
        #[non_exhaustive]
        Text {
            #[serde(default)]
            text: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl UserMessageInputItemContentItem {
    pub fn file(source: FileContentPartSource) -> Self {
        Self::File { source }
    }

    pub fn text(text: String) -> Self {
        Self::Text { text }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
