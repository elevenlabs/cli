pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Interim transcription of the source audio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslatePartialTranscriptPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Partial transcription text.
    #[serde(default)]
    pub text: String,
}

impl TranslatePartialTranscriptPayload {
    pub fn builder() -> TranslatePartialTranscriptPayloadBuilder {
        <TranslatePartialTranscriptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslatePartialTranscriptPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl TranslatePartialTranscriptPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslatePartialTranscriptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslatePartialTranscriptPayloadBuilder::message_type)
    /// - [`text`](TranslatePartialTranscriptPayloadBuilder::text)
    pub fn build(self) -> Result<TranslatePartialTranscriptPayload, BuildError> {
        Ok(TranslatePartialTranscriptPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
