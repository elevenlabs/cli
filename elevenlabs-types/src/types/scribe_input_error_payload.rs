pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for input errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeInputErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Input error details.
    #[serde(default)]
    pub error: String,
}

impl ScribeInputErrorPayload {
    pub fn builder() -> ScribeInputErrorPayloadBuilder {
        <ScribeInputErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeInputErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeInputErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeInputErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeInputErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeInputErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeInputErrorPayload, BuildError> {
        Ok(ScribeInputErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
