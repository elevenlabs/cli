pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload indicating a clean end-of-conversation signal from ElevenLabs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ClosePayload {
    /// The message type identifier.
    pub r#type: String,
}

impl ClosePayload {
    pub fn builder() -> ClosePayloadBuilder {
        <ClosePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClosePayloadBuilder {
    r#type: Option<String>,
}

impl ClosePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClosePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ClosePayloadBuilder::r#type)
    pub fn build(self) -> Result<ClosePayload, BuildError> {
        Ok(ClosePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
