pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for queue overflow errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeQueueOverflowErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Queue overflow error details.
    #[serde(default)]
    pub error: String,
}

impl ScribeQueueOverflowErrorPayload {
    pub fn builder() -> ScribeQueueOverflowErrorPayloadBuilder {
        <ScribeQueueOverflowErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeQueueOverflowErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeQueueOverflowErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeQueueOverflowErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeQueueOverflowErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeQueueOverflowErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeQueueOverflowErrorPayload, BuildError> {
        Ok(ScribeQueueOverflowErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
