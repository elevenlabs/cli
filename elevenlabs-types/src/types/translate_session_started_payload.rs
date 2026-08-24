pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Confirms the session has been initialized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateSessionStartedPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Unique identifier for this session.
    #[serde(default)]
    pub session_id: String,
    /// Client-defined session identifier, if provided during connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
}

impl TranslateSessionStartedPayload {
    pub fn builder() -> TranslateSessionStartedPayloadBuilder {
        <TranslateSessionStartedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateSessionStartedPayloadBuilder {
    message_type: Option<String>,
    session_id: Option<String>,
    client_session_id: Option<String>,
}

impl TranslateSessionStartedPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn client_session_id(mut self, value: impl Into<String>) -> Self {
        self.client_session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateSessionStartedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateSessionStartedPayloadBuilder::message_type)
    /// - [`session_id`](TranslateSessionStartedPayloadBuilder::session_id)
    pub fn build(self) -> Result<TranslateSessionStartedPayload, BuildError> {
        Ok(TranslateSessionStartedPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            client_session_id: self.client_session_id,
        })
    }
}
