pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for throttled errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeThrottledErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Throttled error details.
    #[serde(default)]
    pub error: String,
}

impl ScribeThrottledErrorPayload {
    pub fn builder() -> ScribeThrottledErrorPayloadBuilder {
        <ScribeThrottledErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeThrottledErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeThrottledErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeThrottledErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeThrottledErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeThrottledErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeThrottledErrorPayload, BuildError> {
        Ok(ScribeThrottledErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
