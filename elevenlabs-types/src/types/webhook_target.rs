pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WebhookTarget {
        #[serde(rename = "all")]
        #[non_exhaustive]
        All {},

        #[serde(rename = "ids")]
        #[non_exhaustive]
        Ids {
            #[serde(default)]
            ids: Vec<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WebhookTarget {
    pub fn all() -> Self {
        Self::All {}
    }

    pub fn ids(ids: Vec<String>) -> Self {
        Self::Ids { ids }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
