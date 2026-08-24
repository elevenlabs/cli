pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source")]
#[non_exhaustive]
pub enum AgentAnalysisItemsOutputDataCollectionItem {
        #[serde(rename = "system")]
        #[non_exhaustive]
        System {
            #[serde(flatten)]
            data: AttachedSystemDataCollectionRef,
        },

        #[serde(rename = "user")]
        #[non_exhaustive]
        User {
            #[serde(flatten)]
            data: AttachedUserDataCollectionRef,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentAnalysisItemsOutputDataCollectionItem {
    pub fn system(data: AttachedSystemDataCollectionRef) -> Self {
        Self::System { data }
    }

    pub fn user(data: AttachedUserDataCollectionRef) -> Self {
        Self::User { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
