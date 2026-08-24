pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload containing the full conversation history sent by ElevenLabs each time
/// the user finishes speaking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserTranscriptPayload {
    /// The message type identifier.
    pub r#type: String,
    /// Full conversation history up to and including the latest user turn, ordered
    /// chronologically. Contains both `user` and `agent` turns.
    #[serde(default)]
    pub user_transcript: Vec<TranscriptMessage>,
    /// Monotonically increasing identifier for this transcript event. Pass this value
    /// back in every `agent_response` message so ElevenLabs can correlate responses and
    /// discard any that belong to an interrupted turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
}

impl UserTranscriptPayload {
    pub fn builder() -> UserTranscriptPayloadBuilder {
        <UserTranscriptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserTranscriptPayloadBuilder {
    r#type: Option<String>,
    user_transcript: Option<Vec<TranscriptMessage>>,
    event_id: Option<i64>,
}

impl UserTranscriptPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn user_transcript(mut self, value: Vec<TranscriptMessage>) -> Self {
        self.user_transcript = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserTranscriptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UserTranscriptPayloadBuilder::r#type)
    /// - [`user_transcript`](UserTranscriptPayloadBuilder::user_transcript)
    pub fn build(self) -> Result<UserTranscriptPayload, BuildError> {
        Ok(UserTranscriptPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            user_transcript: self.user_transcript.ok_or_else(|| BuildError::missing_field("user_transcript"))?,
            event_id: self.event_id,
        })
    }
}
