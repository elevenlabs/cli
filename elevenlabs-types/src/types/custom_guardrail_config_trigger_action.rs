pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CustomGuardrailConfigTriggerAction {
        #[serde(rename = "end_call")]
        #[non_exhaustive]
        EndCall {
            #[serde(flatten)]
            data: EndCallTriggerAction,
        },

        #[serde(rename = "retry")]
        #[non_exhaustive]
        Retry {
            #[serde(flatten)]
            data: RetryTriggerAction,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl CustomGuardrailConfigTriggerAction {
    pub fn end_call(data: EndCallTriggerAction) -> Self {
        Self::EndCall { data }
    }

    pub fn retry(data: RetryTriggerAction) -> Self {
        Self::Retry { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
