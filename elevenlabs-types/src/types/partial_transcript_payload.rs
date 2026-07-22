pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for partial transcription results that may change.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartialTranscriptPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Partial transcription text.
    #[serde(default)]
    pub text: String,
}

impl PartialTranscriptPayload {
    pub fn builder() -> PartialTranscriptPayloadBuilder {
        <PartialTranscriptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartialTranscriptPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl PartialTranscriptPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartialTranscriptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](PartialTranscriptPayloadBuilder::message_type)
    /// - [`text`](PartialTranscriptPayloadBuilder::text)
    pub fn build(self) -> Result<PartialTranscriptPayload, BuildError> {
        Ok(PartialTranscriptPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
