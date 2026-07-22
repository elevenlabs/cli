pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "preference")]
#[non_exhaustive]
pub enum PromptAgentApiModelOutputBackupLlmConfig {
        #[serde(rename = "default")]
        #[non_exhaustive]
        Default {
            #[serde(flatten)]
            data: BackupLlmDefault,
        },

        #[serde(rename = "disabled")]
        #[non_exhaustive]
        Disabled {
            #[serde(flatten)]
            data: BackupLlmDisabled,
        },

        #[serde(rename = "override")]
        #[non_exhaustive]
        Override {
            #[serde(flatten)]
            data: BackupLlmOverride,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl PromptAgentApiModelOutputBackupLlmConfig {
    pub fn default(data: BackupLlmDefault) -> Self {
        Self::Default { data }
    }

    pub fn disabled(data: BackupLlmDisabled) -> Self {
        Self::Disabled { data }
    }

    pub fn r#override(data: BackupLlmOverride) -> Self {
        Self::Override { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
