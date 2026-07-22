pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for committed transcription results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommittedTranscriptPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Committed transcription text.
    #[serde(default)]
    pub text: String,
}

impl CommittedTranscriptPayload {
    pub fn builder() -> CommittedTranscriptPayloadBuilder {
        <CommittedTranscriptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl CommittedTranscriptPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscriptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](CommittedTranscriptPayloadBuilder::message_type)
    /// - [`text`](CommittedTranscriptPayloadBuilder::text)
    pub fn build(self) -> Result<CommittedTranscriptPayload, BuildError> {
        Ok(CommittedTranscriptPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
