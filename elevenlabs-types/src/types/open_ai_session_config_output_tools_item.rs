pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OpenAiSessionConfigOutputToolsItem {
        #[serde(rename = "function")]
        #[non_exhaustive]
        Function {
            #[serde(flatten)]
            data: OpenAiFunctionTool,
        },

        #[serde(rename = "mcp")]
        #[non_exhaustive]
        Mcp {
            #[serde(flatten)]
            data: OpenAimcpTool,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl OpenAiSessionConfigOutputToolsItem {
    pub fn function(data: OpenAiFunctionTool) -> Self {
        Self::Function { data }
    }

    pub fn mcp(data: OpenAimcpTool) -> Self {
        Self::Mcp { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
