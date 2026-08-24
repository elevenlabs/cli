pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MemoryEntrySearchResultSource {
        ConversationSource(ConversationSource),

        ManualSource(ManualSource),
}

impl MemoryEntrySearchResultSource {
    pub fn is_conversation_source(&self) -> bool {
        matches!(self, Self::ConversationSource(_))
    }

    pub fn is_manual_source(&self) -> bool {
        matches!(self, Self::ManualSource(_))
    }


    pub fn as_conversation_source(&self) -> Option<&ConversationSource> {
        match self {
                    Self::ConversationSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversation_source(self) -> Option<ConversationSource> {
        match self {
                    Self::ConversationSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_manual_source(&self) -> Option<&ManualSource> {
        match self {
                    Self::ManualSource(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_manual_source(self) -> Option<ManualSource> {
        match self {
                    Self::ManualSource(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for MemoryEntrySearchResultSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationSource(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ManualSource(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
