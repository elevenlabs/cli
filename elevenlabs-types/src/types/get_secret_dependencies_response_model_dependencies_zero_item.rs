pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetSecretDependenciesResponseModelDependenciesZeroItem {
        #[serde(rename = "available")]
        #[non_exhaustive]
        Available {
            #[serde(flatten)]
            data: DependentAvailableToolIdentifier,
        },

        #[serde(rename = "unknown")]
        #[non_exhaustive]
        Unknown {
            #[serde(flatten)]
            data: DependentUnknownToolIdentifier,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl GetSecretDependenciesResponseModelDependenciesZeroItem {
    pub fn available(data: DependentAvailableToolIdentifier) -> Self {
        Self::Available { data }
    }

    pub fn unknown(data: DependentUnknownToolIdentifier) -> Self {
        Self::Unknown { data }
    }
}
