pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum FileContentPartSource {
        #[serde(rename = "base64")]
        #[non_exhaustive]
        Base64 {
            #[serde(default)]
            filename: String,
            media_type: FileInputMimeType,
            #[serde(default)]
            data: String,
        },

        #[serde(rename = "file_id")]
        #[non_exhaustive]
        FileId {
            #[serde(default)]
            file_id: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl FileContentPartSource {
    pub fn base64(filename: String, media_type: FileInputMimeType, data: String) -> Self {
        Self::Base64 { filename, media_type, data }
    }

    pub fn file_id(file_id: String) -> Self {
        Self::FileId { file_id }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
