pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "mode")]
#[non_exhaustive]
pub enum UserEvaluationAnalysisItemScoring {
        #[serde(rename = "binary")]
        #[non_exhaustive]
        Binary {},

        #[serde(rename = "numeric_uniform")]
        #[non_exhaustive]
        NumericUniform {
            #[serde(default)]
            max_score: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            instructions: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl UserEvaluationAnalysisItemScoring {
    pub fn binary() -> Self {
        Self::Binary {}
    }

    pub fn numeric_uniform(max_score: i64) -> Self {
        Self::NumericUniform { max_score, instructions: None }
    }

    pub fn numeric_uniform_with_instructions(max_score: i64, instructions: String) -> Self {
        Self::NumericUniform { max_score, instructions: Some(instructions) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
