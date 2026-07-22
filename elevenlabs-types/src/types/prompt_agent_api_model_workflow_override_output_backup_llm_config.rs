pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PromptAgentApiModelWorkflowOverrideOutputBackupLlmConfig {
        BackupLlmDefault(BackupLlmDefault),

        BackupLlmDisabled(BackupLlmDisabled),

        BackupLlmOverride(BackupLlmOverride),
}

impl PromptAgentApiModelWorkflowOverrideOutputBackupLlmConfig {
    pub fn is_backup_llm_default(&self) -> bool {
        matches!(self, Self::BackupLlmDefault(_))
    }

    pub fn is_backup_llm_disabled(&self) -> bool {
        matches!(self, Self::BackupLlmDisabled(_))
    }

    pub fn is_backup_llm_override(&self) -> bool {
        matches!(self, Self::BackupLlmOverride(_))
    }


    pub fn as_backup_llm_default(&self) -> Option<&BackupLlmDefault> {
        match self {
                    Self::BackupLlmDefault(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_backup_llm_default(self) -> Option<BackupLlmDefault> {
        match self {
                    Self::BackupLlmDefault(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_backup_llm_disabled(&self) -> Option<&BackupLlmDisabled> {
        match self {
                    Self::BackupLlmDisabled(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_backup_llm_disabled(self) -> Option<BackupLlmDisabled> {
        match self {
                    Self::BackupLlmDisabled(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_backup_llm_override(&self) -> Option<&BackupLlmOverride> {
        match self {
                    Self::BackupLlmOverride(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_backup_llm_override(self) -> Option<BackupLlmOverride> {
        match self {
                    Self::BackupLlmOverride(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for PromptAgentApiModelWorkflowOverrideOutputBackupLlmConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BackupLlmDefault(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::BackupLlmDisabled(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::BackupLlmOverride(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
