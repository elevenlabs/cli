pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload sent when the transcription session is successfully started.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionStartedPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Unique identifier for the session.
    #[serde(default)]
    pub session_id: String,
    /// Configuration for the transcription session.
    #[serde(default)]
    pub config: SessionStartedPayloadConfig,
}

impl SessionStartedPayload {
    pub fn builder() -> SessionStartedPayloadBuilder {
        <SessionStartedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionStartedPayloadBuilder {
    message_type: Option<String>,
    session_id: Option<String>,
    config: Option<SessionStartedPayloadConfig>,
}

impl SessionStartedPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: SessionStartedPayloadConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionStartedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SessionStartedPayloadBuilder::message_type)
    /// - [`session_id`](SessionStartedPayloadBuilder::session_id)
    /// - [`config`](SessionStartedPayloadBuilder::config)
    pub fn build(self) -> Result<SessionStartedPayload, BuildError> {
        Ok(SessionStartedPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
