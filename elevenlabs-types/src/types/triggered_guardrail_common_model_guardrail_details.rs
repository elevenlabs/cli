pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TriggeredGuardrailCommonModelGuardrailDetails {
        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(default)]
            rationale: String,
        },

        #[serde(rename = "classifier")]
        #[non_exhaustive]
        Classifier {
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            score: f64,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            threshold: f64,
        },

        #[serde(rename = "unknown")]
        #[non_exhaustive]
        Unknown {},

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TriggeredGuardrailCommonModelGuardrailDetails {
    pub fn llm(rationale: String) -> Self {
        Self::Llm { rationale }
    }

    pub fn classifier(score: f64, threshold: f64) -> Self {
        Self::Classifier { score, threshold }
    }

    pub fn unknown() -> Self {
        Self::Unknown {}
    }
}
