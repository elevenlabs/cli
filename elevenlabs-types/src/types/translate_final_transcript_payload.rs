pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Final transcription of a completed segment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateFinalTranscriptPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Final transcription text.
    #[serde(default)]
    pub text: String,
}

impl TranslateFinalTranscriptPayload {
    pub fn builder() -> TranslateFinalTranscriptPayloadBuilder {
        <TranslateFinalTranscriptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateFinalTranscriptPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl TranslateFinalTranscriptPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateFinalTranscriptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateFinalTranscriptPayloadBuilder::message_type)
    /// - [`text`](TranslateFinalTranscriptPayloadBuilder::text)
    pub fn build(self) -> Result<TranslateFinalTranscriptPayload, BuildError> {
        Ok(TranslateFinalTranscriptPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
