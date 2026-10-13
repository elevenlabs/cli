pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
#[non_exhaustive]
pub enum AgentMergeProposalResponseOutcome {
        #[serde(rename = "closed")]
        #[non_exhaustive]
        Closed {
            reason: MergeProposalCloseReason,
            #[serde(default)]
            at: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            by_user_id: Option<String>,
        },

        #[serde(rename = "merged")]
        #[non_exhaustive]
        Merged {
            #[serde(default)]
            version_id: String,
            #[serde(default)]
            at: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            by_user_id: Option<String>,
        },

        #[serde(rename = "open")]
        #[non_exhaustive]
        Open {},

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AgentMergeProposalResponseOutcome {
    pub fn closed(reason: MergeProposalCloseReason, at: i64) -> Self {
        Self::Closed { reason, at, by_user_id: None }
    }

    pub fn merged(version_id: String, at: i64) -> Self {
        Self::Merged { version_id, at, by_user_id: None }
    }

    pub fn open() -> Self {
        Self::Open {}
    }

    pub fn closed_with_by_user_id(reason: MergeProposalCloseReason, at: i64, by_user_id: String) -> Self {
        Self::Closed { reason, at, by_user_id: Some(by_user_id) }
    }

    pub fn merged_with_by_user_id(version_id: String, at: i64, by_user_id: String) -> Self {
        Self::Merged { version_id, at, by_user_id: Some(by_user_id) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
