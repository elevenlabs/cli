pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for rate limited errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScribeRateLimitedErrorPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Rate limited error details.
    #[serde(default)]
    pub error: String,
}

impl ScribeRateLimitedErrorPayload {
    pub fn builder() -> ScribeRateLimitedErrorPayloadBuilder {
        <ScribeRateLimitedErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeRateLimitedErrorPayloadBuilder {
    message_type: Option<String>,
    error: Option<String>,
}

impl ScribeRateLimitedErrorPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeRateLimitedErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](ScribeRateLimitedErrorPayloadBuilder::message_type)
    /// - [`error`](ScribeRateLimitedErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ScribeRateLimitedErrorPayload, BuildError> {
        Ok(ScribeRateLimitedErrorPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
