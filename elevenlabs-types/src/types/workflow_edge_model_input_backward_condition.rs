pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowEdgeModelInputBackwardCondition {
        #[serde(rename = "expression")]
        #[non_exhaustive]
        Expression {
            #[serde(flatten)]
            data: WorkflowExpressionConditionModelInput,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(flatten)]
            data: WorkflowLlmConditionModelInput,
        },

        #[serde(rename = "result")]
        #[non_exhaustive]
        r#Result {
            #[serde(flatten)]
            data: WorkflowResultConditionModelInput,
        },

        #[serde(rename = "unconditional")]
        #[non_exhaustive]
        Unconditional {
            #[serde(flatten)]
            data: WorkflowUnconditionalModelInput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowEdgeModelInputBackwardCondition {
    pub fn expression(data: WorkflowExpressionConditionModelInput) -> Self {
        Self::Expression { data }
    }

    pub fn llm(data: WorkflowLlmConditionModelInput) -> Self {
        Self::Llm { data }
    }

    pub fn result(data: WorkflowResultConditionModelInput) -> Self {
        Self::r#Result { data }
    }

    pub fn unconditional(data: WorkflowUnconditionalModelInput) -> Self {
        Self::Unconditional { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
