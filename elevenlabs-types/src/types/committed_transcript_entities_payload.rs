pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for detected entities on a committed transcript.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommittedTranscriptEntitiesPayload {
    /// The message type identifier.
    pub message_type: String,
    /// The committed transcript text the entities were detected in.
    #[serde(default)]
    pub text: String,
    /// Detected entities. Empty if none were found.
    #[serde(default)]
    pub entities: Vec<DetectedEntity>,
}

impl CommittedTranscriptEntitiesPayload {
    pub fn builder() -> CommittedTranscriptEntitiesPayloadBuilder {
        <CommittedTranscriptEntitiesPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommittedTranscriptEntitiesPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
    entities: Option<Vec<DetectedEntity>>,
}

impl CommittedTranscriptEntitiesPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn entities(mut self, value: Vec<DetectedEntity>) -> Self {
        self.entities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommittedTranscriptEntitiesPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](CommittedTranscriptEntitiesPayloadBuilder::message_type)
    /// - [`text`](CommittedTranscriptEntitiesPayloadBuilder::text)
    /// - [`entities`](CommittedTranscriptEntitiesPayloadBuilder::entities)
    pub fn build(self) -> Result<CommittedTranscriptEntitiesPayload, BuildError> {
        Ok(CommittedTranscriptEntitiesPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            entities: self.entities.ok_or_else(|| BuildError::missing_field("entities"))?,
        })
    }
}
