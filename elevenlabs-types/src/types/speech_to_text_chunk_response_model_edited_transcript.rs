pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum SpeechToTextChunkResponseModelEditedTranscript {
        #[serde(rename = "error")]
        #[non_exhaustive]
        Error {
            error_type: String,
            #[serde(default)]
            message: String,
        },

        #[serde(rename = "transcript")]
        #[non_exhaustive]
        Transcript {
            #[serde(flatten)]
            data: EditedTranscript,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SpeechToTextChunkResponseModelEditedTranscript {
    pub fn error(error_type: String, message: String) -> Self {
        Self::Error { error_type, message }
    }

    pub fn transcript(data: EditedTranscript) -> Self {
        Self::Transcript { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
