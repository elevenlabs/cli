pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum UnitTestToolCallParameterEval {
        #[serde(rename = "anything")]
        #[non_exhaustive]
        Anything {},

        #[serde(rename = "exact")]
        #[non_exhaustive]
        Exact {
            #[serde(default)]
            expected_value: String,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(default)]
            description: String,
        },

        #[serde(rename = "regex")]
        #[non_exhaustive]
        Regex {
            #[serde(default)]
            pattern: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl UnitTestToolCallParameterEval {
    pub fn anything() -> Self {
        Self::Anything {}
    }

    pub fn exact(expected_value: String) -> Self {
        Self::Exact { expected_value }
    }

    pub fn llm(description: String) -> Self {
        Self::Llm { description }
    }

    pub fn regex(pattern: String) -> Self {
        Self::Regex { pattern }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
