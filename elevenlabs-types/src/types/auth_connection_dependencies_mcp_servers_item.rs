pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AuthConnectionDependenciesMcpServersItem {
        #[serde(rename = "available")]
        #[non_exhaustive]
        Available {
            #[serde(flatten)]
            data: DependentAvailableMcpServerIdentifier,
        },

        #[serde(rename = "unknown")]
        #[non_exhaustive]
        Unknown {
            #[serde(flatten)]
            data: DependentUnknownMcpServerIdentifier,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AuthConnectionDependenciesMcpServersItem {
    pub fn available(data: DependentAvailableMcpServerIdentifier) -> Self {
        Self::Available { data }
    }

    pub fn unknown(data: DependentUnknownMcpServerIdentifier) -> Self {
        Self::Unknown { data }
    }
}
