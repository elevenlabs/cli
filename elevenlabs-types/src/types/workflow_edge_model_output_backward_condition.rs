pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowEdgeModelOutputBackwardCondition {
        #[serde(rename = "expression")]
        #[non_exhaustive]
        Expression {
            #[serde(flatten)]
            data: WorkflowExpressionConditionModelOutput,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(flatten)]
            data: WorkflowLlmConditionModelOutput,
        },

        #[serde(rename = "result")]
        #[non_exhaustive]
        r#Result {
            #[serde(flatten)]
            data: WorkflowResultConditionModelOutput,
        },

        #[serde(rename = "unconditional")]
        #[non_exhaustive]
        Unconditional {
            #[serde(flatten)]
            data: WorkflowUnconditionalModelOutput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowEdgeModelOutputBackwardCondition {
    pub fn expression(data: WorkflowExpressionConditionModelOutput) -> Self {
        Self::Expression { data }
    }

    pub fn llm(data: WorkflowLlmConditionModelOutput) -> Self {
        Self::Llm { data }
    }

    pub fn result(data: WorkflowResultConditionModelOutput) -> Self {
        Self::r#Result { data }
    }

    pub fn unconditional(data: WorkflowUnconditionalModelOutput) -> Self {
        Self::Unconditional { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
