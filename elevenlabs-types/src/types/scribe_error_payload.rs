pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for error events during transcription.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Error message describing what went wrong.
    #[serde(default)]
    pub error: String,
}

impl ScribeErrorPayload {
    pub fn builder() -> ScribeErrorPayloadBuilder {
        <ScribeErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeErrorPayload, BuildError> {
        Ok(ScribeErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
