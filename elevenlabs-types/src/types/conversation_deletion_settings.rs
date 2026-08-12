pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationDeletionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_logs_at_time_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_audio_at_time_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_transcript_at_time_unix_secs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_transcript_and_pii: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_audio: Option<bool>,
}

impl ConversationDeletionSettings {
    pub fn builder() -> ConversationDeletionSettingsBuilder {
        <ConversationDeletionSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationDeletionSettingsBuilder {
    deletion_time_unix_secs: Option<i64>,
    deleted_logs_at_time_unix_secs: Option<i64>,
    deleted_audio_at_time_unix_secs: Option<i64>,
    deleted_transcript_at_time_unix_secs: Option<i64>,
    delete_transcript_and_pii: Option<bool>,
    delete_audio: Option<bool>,
}

impl ConversationDeletionSettingsBuilder {
    pub fn deletion_time_unix_secs(mut self, value: i64) -> Self {
        self.deletion_time_unix_secs = Some(value);
        self
    }

    pub fn deleted_logs_at_time_unix_secs(mut self, value: i64) -> Self {
        self.deleted_logs_at_time_unix_secs = Some(value);
        self
    }

    pub fn deleted_audio_at_time_unix_secs(mut self, value: i64) -> Self {
        self.deleted_audio_at_time_unix_secs = Some(value);
        self
    }

    pub fn deleted_transcript_at_time_unix_secs(mut self, value: i64) -> Self {
        self.deleted_transcript_at_time_unix_secs = Some(value);
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

    /// Consumes the builder and constructs a [`ConversationDeletionSettings`].
    pub fn build(self) -> Result<ConversationDeletionSettings, BuildError> {
        Ok(ConversationDeletionSettings {
            deletion_time_unix_secs: self.deletion_time_unix_secs,
            deleted_logs_at_time_unix_secs: self.deleted_logs_at_time_unix_secs,
            deleted_audio_at_time_unix_secs: self.deleted_audio_at_time_unix_secs,
            deleted_transcript_at_time_unix_secs: self.deleted_transcript_at_time_unix_secs,
            delete_transcript_and_pii: self.delete_transcript_and_pii,
            delete_audio: self.delete_audio,
        })
    }
}
