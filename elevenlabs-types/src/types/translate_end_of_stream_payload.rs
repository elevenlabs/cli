pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Signal that the client has finished sending audio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct TranslateEndOfStreamPayload {
    /// The message type identifier.
    pub message_type: String,
}

impl TranslateEndOfStreamPayload {
    pub fn builder() -> TranslateEndOfStreamPayloadBuilder {
        <TranslateEndOfStreamPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateEndOfStreamPayloadBuilder {
    message_type: Option<String>,
}

impl TranslateEndOfStreamPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateEndOfStreamPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateEndOfStreamPayloadBuilder::message_type)
    pub fn build(self) -> Result<TranslateEndOfStreamPayload, BuildError> {
        Ok(TranslateEndOfStreamPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
        })
    }
}
