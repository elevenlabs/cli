pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "branch_reason")]
#[non_exhaustive]
pub enum TransferToAgentToolResultSuccessModelOutputBranchInfo {
        #[serde(rename = "defaulting_to_main")]
        #[non_exhaustive]
        DefaultingToMain {
            #[serde(flatten)]
            data: TransferBranchInfoDefaultingToMain,
        },

        #[serde(rename = "traffic_split")]
        #[non_exhaustive]
        TrafficSplit {
            #[serde(flatten)]
            data: TransferBranchInfoTrafficSplit,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TransferToAgentToolResultSuccessModelOutputBranchInfo {
    pub fn defaulting_to_main(data: TransferBranchInfoDefaultingToMain) -> Self {
        Self::DefaultingToMain { data }
    }

    pub fn traffic_split(data: TransferBranchInfoTrafficSplit) -> Self {
        Self::TrafficSplit { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
