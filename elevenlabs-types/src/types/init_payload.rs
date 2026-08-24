pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for the session initialisation message sent by ElevenLabs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitPayload {
    /// The message type identifier.
    pub r#type: String,
    /// Unique identifier for this conversation session.
    #[serde(default)]
    pub conversation_id: String,
}

impl InitPayload {
    pub fn builder() -> InitPayloadBuilder {
        <InitPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitPayloadBuilder {
    r#type: Option<String>,
    conversation_id: Option<String>,
}

impl InitPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InitPayloadBuilder::r#type)
    /// - [`conversation_id`](InitPayloadBuilder::conversation_id)
    pub fn build(self) -> Result<InitPayload, BuildError> {
        Ok(InitPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
