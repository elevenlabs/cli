pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem {
        #[serde(rename = "dynamic")]
        #[non_exhaustive]
        Dynamic {
            #[serde(flatten)]
            data: CustomSipHeaderWithDynamicVariable,
        },

        #[serde(rename = "static")]
        #[non_exhaustive]
        Static {
            #[serde(flatten)]
            data: CustomSipHeader,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowPhoneNumberNodeModelInputCustomSipHeadersItem {
    pub fn dynamic(data: CustomSipHeaderWithDynamicVariable) -> Self {
        Self::Dynamic { data }
    }

    pub fn r#static(data: CustomSipHeader) -> Self {
        Self::Static { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
