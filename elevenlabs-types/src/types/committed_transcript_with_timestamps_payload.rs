pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for committed transcription results with word-level timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommittedTranscriptWithTimestampsPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Committed transcription text.
    #[serde(default)]
    pub text: String,
    /// Detected or specified language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Word-level information with timestamps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<TranscriptionWord>>,
}

impl CommittedTranscriptWithTimestampsPayload {
    pub fn builder() -> CommittedTranscriptWithTimestampsPayloadBuilder {
        <CommittedTranscriptWithTimestampsPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptWithTimestampsPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
    language_code: Option<String>,
    words: Option<Vec<TranscriptionWord>>,
}

impl CommittedTranscriptWithTimestampsPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<TranscriptionWord>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscriptWithTimestampsPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](CommittedTranscriptWithTimestampsPayloadBuilder::message_type)
    /// - [`text`](CommittedTranscriptWithTimestampsPayloadBuilder::text)
    pub fn build(self) -> Result<CommittedTranscriptWithTimestampsPayload, BuildError> {
        Ok(CommittedTranscriptWithTimestampsPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            language_code: self.language_code,
            words: self.words,
        })
    }
}
