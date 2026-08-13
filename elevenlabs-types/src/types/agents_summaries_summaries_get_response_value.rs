pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum SummariesGetResponseValue {
        #[serde(rename = "success")]
        #[non_exhaustive]
        Success {
            data: AgentSummaryResponseModel,
        },

        #[serde(rename = "failure")]
        #[non_exhaustive]
        Failure {
            #[serde(flatten)]
            data: BatchFailureResponseModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl SummariesGetResponseValue {
    pub fn success(data: AgentSummaryResponseModel) -> Self {
        Self::Success { data }
    }

    pub fn failure(data: BatchFailureResponseModel) -> Self {
        Self::Failure { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
