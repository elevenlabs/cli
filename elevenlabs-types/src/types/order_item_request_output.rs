pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum OrderItemRequestOutput {
        #[serde(rename = "dub")]
        #[non_exhaustive]
        Dub {
            #[serde(flatten)]
            data: DubOrderItemRequest,
        },

        #[serde(rename = "subtitles")]
        #[non_exhaustive]
        Subtitles {
            #[serde(flatten)]
            data: SubtitleOrderItemRequest,
        },

        #[serde(rename = "transcription")]
        #[non_exhaustive]
        Transcription {
            #[serde(flatten)]
            data: TranscriptionOrderItemRequest,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl OrderItemRequestOutput {
    pub fn dub(data: DubOrderItemRequest) -> Self {
        Self::Dub { data }
    }

    pub fn subtitles(data: SubtitleOrderItemRequest) -> Self {
        Self::Subtitles { data }
    }

    pub fn transcription(data: TranscriptionOrderItemRequest) -> Self {
        Self::Transcription { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
