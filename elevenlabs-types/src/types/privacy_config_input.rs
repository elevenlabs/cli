pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrivacyConfigInput {
    /// Whether to record the conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_voice: Option<bool>,
    /// The number of days to retain the conversation. -1 indicates there is no retention limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
    /// Whether to delete the transcript and PII
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_transcript_and_pii: Option<bool>,
    /// Whether to delete the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_audio: Option<bool>,
    /// Whether to apply the privacy settings to existing conversations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to_existing_conversations: Option<bool>,
    /// Whether to enable zero retention mode - no PII data is stored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_retention_mode: Option<bool>,
    /// Config for PII redaction in the conversation history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_history_redaction: Option<ConversationHistoryRedactionConfig>,
}

impl PrivacyConfigInput {
    pub fn builder() -> PrivacyConfigInputBuilder {
        <PrivacyConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrivacyConfigInputBuilder {
    record_voice: Option<bool>,
    retention_days: Option<i64>,
    delete_transcript_and_pii: Option<bool>,
    delete_audio: Option<bool>,
    apply_to_existing_conversations: Option<bool>,
    zero_retention_mode: Option<bool>,
    conversation_history_redaction: Option<ConversationHistoryRedactionConfig>,
}

impl PrivacyConfigInputBuilder {
    pub fn record_voice(mut self, value: bool) -> Self {
        self.record_voice = Some(value);
        self
    }

    pub fn retention_days(mut self, value: i64) -> Self {
        self.retention_days = Some(value);
        self
    }

    pub fn delete_transcript_and_pii(mut self, value: bool) -> Self {
        self.delete_transcript_and_pii = Some(value);
        self
    }

    pub fn delete_audio(mut self, value: bool) -> Self {
        self.delete_audio = Some(value);
        self
    }

    pub fn apply_to_existing_conversations(mut self, value: bool) -> Self {
        self.apply_to_existing_conversations = Some(value);
        self
    }

    pub fn zero_retention_mode(mut self, value: bool) -> Self {
        self.zero_retention_mode = Some(value);
        self
    }

    pub fn conversation_history_redaction(mut self, value: ConversationHistoryRedactionConfig) -> Self {
        self.conversation_history_redaction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrivacyConfigInput`].
    pub fn build(self) -> Result<PrivacyConfigInput, BuildError> {
        Ok(PrivacyConfigInput {
            record_voice: self.record_voice,
            retention_days: self.retention_days,
            delete_transcript_and_pii: self.delete_transcript_and_pii,
            delete_audio: self.delete_audio,
            apply_to_existing_conversations: self.apply_to_existing_conversations,
            zero_retention_mode: self.zero_retention_mode,
            conversation_history_redaction: self.conversation_history_redaction,
        })
    }
}
