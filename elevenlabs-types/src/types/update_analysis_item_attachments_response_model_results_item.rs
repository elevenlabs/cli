pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum UpdateAnalysisItemAttachmentsResponseModelResultsItem {
        #[serde(rename = "failure")]
        #[non_exhaustive]
        Failure {
            #[serde(flatten)]
            data: BatchFailureResponseModel,
        },

        #[serde(rename = "success")]
        #[non_exhaustive]
        Success {
            data: AnalysisItemAttachmentResultData,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl UpdateAnalysisItemAttachmentsResponseModelResultsItem {
    pub fn failure(data: BatchFailureResponseModel) -> Self {
        Self::Failure { data }
    }

    pub fn success(data: AnalysisItemAttachmentResultData) -> Self {
        Self::Success { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
