pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for protocol-level errors sent by ElevenLabs before closing the connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ErrorPayload {
    /// The message type identifier.
    pub r#type: String,
    /// Human-readable description of the error.
    #[serde(default)]
    pub message: String,
}

impl ErrorPayload {
    pub fn builder() -> ErrorPayloadBuilder {
        <ErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorPayloadBuilder {
    r#type: Option<String>,
    message: Option<String>,
}

impl ErrorPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ErrorPayloadBuilder::r#type)
    /// - [`message`](ErrorPayloadBuilder::message)
    pub fn build(self) -> Result<ErrorPayload, BuildError> {
        Ok(ErrorPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
