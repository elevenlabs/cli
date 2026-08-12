pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Keep-alive ping sent periodically by ElevenLabs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PingPayload {
    /// The message type identifier.
    pub r#type: String,
}

impl PingPayload {
    pub fn builder() -> PingPayloadBuilder {
        <PingPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PingPayloadBuilder {
    r#type: Option<String>,
}

impl PingPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PingPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PingPayloadBuilder::r#type)
    pub fn build(self) -> Result<PingPayload, BuildError> {
        Ok(PingPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
