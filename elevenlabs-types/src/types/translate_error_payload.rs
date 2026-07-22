pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Error encountered during translation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Error message.
    #[serde(default)]
    pub error: String,
}

impl TranslateErrorPayload {
    pub fn builder() -> TranslateErrorPayloadBuilder {
        <TranslateErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl TranslateErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateErrorPayloadBuilder::message_type)
    /// - [`error`](TranslateErrorPayloadBuilder::error)
    pub fn build(self) -> Result<TranslateErrorPayload, BuildError> {
        Ok(TranslateErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
