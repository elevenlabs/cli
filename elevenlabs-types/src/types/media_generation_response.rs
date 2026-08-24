pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum MediaGenerationResponse {
        #[serde(rename = "completed")]
        #[non_exhaustive]
        Completed {
            #[serde(default)]
            id: String,
            #[serde(default)]
            content_url: String,
            #[serde(default)]
            content_mime_type: String,
        },

        #[serde(rename = "failed")]
        #[non_exhaustive]
        Failed {
            #[serde(default)]
            id: String,
            failure_reason: MediaGenerationFailedResponseFailureReason,
            #[serde(default)]
            error_message: String,
        },

        #[serde(rename = "generating")]
        #[non_exhaustive]
        Generating {
            #[serde(flatten)]
            data: MediaGenerationInProgressResponse,
        },

        #[serde(rename = "pending")]
        #[non_exhaustive]
        Pending {
            #[serde(flatten)]
            data: MediaGenerationInProgressResponse,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl MediaGenerationResponse {
    pub fn completed(id: String, content_url: String, content_mime_type: String) -> Self {
        Self::Completed { id, content_url, content_mime_type }
    }

    pub fn failed(id: String, failure_reason: MediaGenerationFailedResponseFailureReason, error_message: String) -> Self {
        Self::Failed { id, failure_reason, error_message }
    }

    pub fn generating(data: MediaGenerationInProgressResponse) -> Self {
        Self::Generating { data }
    }

    pub fn pending(data: MediaGenerationInProgressResponse) -> Self {
        Self::Pending { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
