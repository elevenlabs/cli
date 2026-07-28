pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryRedactionConfig {
    /// Whether conversation history redaction is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The entities to redact from the conversation transcript, audio and analysis. Use top-level types like 'name', 'email_address', or dot notation for specific subtypes like 'name.full_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<ConfigEntityType>>,
}

impl ConversationHistoryRedactionConfig {
    pub fn builder() -> ConversationHistoryRedactionConfigBuilder {
        <ConversationHistoryRedactionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryRedactionConfigBuilder {
    enabled: Option<bool>,
    entities: Option<Vec<ConfigEntityType>>,
}

impl ConversationHistoryRedactionConfigBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn entities(mut self, value: Vec<ConfigEntityType>) -> Self {
        self.entities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryRedactionConfig`].
    pub fn build(self) -> Result<ConversationHistoryRedactionConfig, BuildError> {
        Ok(ConversationHistoryRedactionConfig {
            enabled: self.enabled,
            entities: self.entities,
        })
    }
}
