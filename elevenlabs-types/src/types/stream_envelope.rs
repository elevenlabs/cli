pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamEnvelope {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub turn_id: String,
    #[serde(default)]
    pub created_at_unix_ms: i64,
    pub event: StreamEnvelopeEvent,
}

impl StreamEnvelope {
    pub fn builder() -> StreamEnvelopeBuilder {
        <StreamEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEnvelopeBuilder {
    conversation_id: Option<String>,
    turn_id: Option<String>,
    created_at_unix_ms: Option<i64>,
    event: Option<StreamEnvelopeEvent>,
}

impl StreamEnvelopeBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn turn_id(mut self, value: impl Into<String>) -> Self {
        self.turn_id = Some(value.into());
        self
    }

    pub fn created_at_unix_ms(mut self, value: i64) -> Self {
        self.created_at_unix_ms = Some(value);
        self
    }

    pub fn event(mut self, value: StreamEnvelopeEvent) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](StreamEnvelopeBuilder::conversation_id)
    /// - [`turn_id`](StreamEnvelopeBuilder::turn_id)
    /// - [`created_at_unix_ms`](StreamEnvelopeBuilder::created_at_unix_ms)
    /// - [`event`](StreamEnvelopeBuilder::event)
    pub fn build(self) -> Result<StreamEnvelope, BuildError> {
        Ok(StreamEnvelope {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            turn_id: self.turn_id.ok_or_else(|| BuildError::missing_field("turn_id"))?,
            created_at_unix_ms: self.created_at_unix_ms.ok_or_else(|| BuildError::missing_field("created_at_unix_ms"))?,
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
