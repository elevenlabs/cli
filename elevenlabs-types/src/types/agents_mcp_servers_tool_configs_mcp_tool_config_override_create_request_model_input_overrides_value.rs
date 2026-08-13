pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source")]
#[non_exhaustive]
pub enum McpToolConfigOverrideCreateRequestModelInputOverridesValue {
        #[serde(rename = "constant")]
        #[non_exhaustive]
        Constant {
            #[serde(flatten)]
            data: ConstantSchemaOverride,
        },

        #[serde(rename = "dynamic_variable")]
        #[non_exhaustive]
        DynamicVariable {
            #[serde(flatten)]
            data: DynamicVariableSchemaOverride,
        },

        #[serde(rename = "llm")]
        #[non_exhaustive]
        Llm {
            #[serde(flatten)]
            data: LlmSchemaOverride,
        },

        #[serde(rename = "omit")]
        #[non_exhaustive]
        Omit {
            #[serde(flatten)]
            data: OmitSchemaOverride,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl McpToolConfigOverrideCreateRequestModelInputOverridesValue {
    pub fn constant(data: ConstantSchemaOverride) -> Self {
        Self::Constant { data }
    }

    pub fn dynamic_variable(data: DynamicVariableSchemaOverride) -> Self {
        Self::DynamicVariable { data }
    }

    pub fn llm(data: LlmSchemaOverride) -> Self {
        Self::Llm { data }
    }

    pub fn omit(data: OmitSchemaOverride) -> Self {
        Self::Omit { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
