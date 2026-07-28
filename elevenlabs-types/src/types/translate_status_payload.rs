pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Session lifecycle update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateStatusPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Current session status (`started` or `stopped`).
    #[serde(default)]
    pub status: String,
}

impl TranslateStatusPayload {
    pub fn builder() -> TranslateStatusPayloadBuilder {
        <TranslateStatusPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateStatusPayloadBuilder {
    message_type: Option<String>,
    status: Option<String>,
}

impl TranslateStatusPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateStatusPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateStatusPayloadBuilder::message_type)
    /// - [`status`](TranslateStatusPayloadBuilder::status)
    pub fn build(self) -> Result<TranslateStatusPayload, BuildError> {
        Ok(TranslateStatusPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
