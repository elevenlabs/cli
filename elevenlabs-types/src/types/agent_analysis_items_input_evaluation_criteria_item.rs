pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source")]
#[non_exhaustive]
pub enum AgentAnalysisItemsInputEvaluationCriteriaItem {
        #[serde(rename = "system")]
        #[non_exhaustive]
        System {
            #[serde(flatten)]
            data: AttachedSystemEvaluationRef,
        },

        #[serde(rename = "user")]
        #[non_exhaustive]
        User {
            #[serde(flatten)]
            data: AttachedUserEvaluationRef,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentAnalysisItemsInputEvaluationCriteriaItem {
    pub fn system(data: AttachedSystemEvaluationRef) -> Self {
        Self::System { data }
    }

    pub fn user(data: AttachedUserEvaluationRef) -> Self {
        Self::User { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
