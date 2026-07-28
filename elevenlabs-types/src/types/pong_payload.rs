pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Reply to a `ping` message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PongPayload {
    /// The message type identifier.
    pub r#type: String,
}

impl PongPayload {
    pub fn builder() -> PongPayloadBuilder {
        <PongPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PongPayloadBuilder {
    r#type: Option<String>,
}

impl PongPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PongPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PongPayloadBuilder::r#type)
    pub fn build(self) -> Result<PongPayload, BuildError> {
        Ok(PongPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
