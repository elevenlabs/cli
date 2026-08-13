pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for session time limit exceeded errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeSessionTimeLimitExceededErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Session time limit exceeded error details.
    #[serde(default)]
    pub error: String,
}

impl ScribeSessionTimeLimitExceededErrorPayload {
    pub fn builder() -> ScribeSessionTimeLimitExceededErrorPayloadBuilder {
        <ScribeSessionTimeLimitExceededErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeSessionTimeLimitExceededErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeSessionTimeLimitExceededErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeSessionTimeLimitExceededErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeSessionTimeLimitExceededErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeSessionTimeLimitExceededErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeSessionTimeLimitExceededErrorPayload, BuildError> {
        Ok(ScribeSessionTimeLimitExceededErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
