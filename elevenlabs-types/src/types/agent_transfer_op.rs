pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AgentTransferOp {
        #[serde(rename = "pop")]
        #[non_exhaustive]
        Pop {},

        #[serde(rename = "push")]
        #[non_exhaustive]
        Push {
            #[serde(skip_serializing_if = "Option::is_none")]
            return_node_id: Option<String>,
        },

        #[serde(rename = "replace")]
        #[non_exhaustive]
        Replace {},

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentTransferOp {
    pub fn pop() -> Self {
        Self::Pop {}
    }

    pub fn push() -> Self {
        Self::Push { return_node_id: None }
    }

    pub fn replace() -> Self {
        Self::Replace {}
    }

    pub fn push_with_return_node_id(return_node_id: String) -> Self {
        Self::Push { return_node_id: Some(return_node_id) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
