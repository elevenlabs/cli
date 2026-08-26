pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SoftTimeoutConfigOverride {
    /// Message to show when the first soft timeout is reached while waiting for LLM response. Supports dynamic variables (e.g., {{system__time}}, {{custom_variable}}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Extra static filler messages for subsequent soft timeouts in the same LLM generation. The first timeout uses `message`. If fewer messages are configured than `max_soft_timeouts_per_generation`, the last configured message is repeated; otherwise a built-in filler is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_soft_timeout_messages: Option<Vec<String>>,
}

impl SoftTimeoutConfigOverride {
    pub fn builder() -> SoftTimeoutConfigOverrideBuilder {
        <SoftTimeoutConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SoftTimeoutConfigOverrideBuilder {
    message: Option<String>,
    additional_soft_timeout_messages: Option<Vec<String>>,
}

impl SoftTimeoutConfigOverrideBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn additional_soft_timeout_messages(mut self, value: Vec<String>) -> Self {
        self.additional_soft_timeout_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SoftTimeoutConfigOverride`].
    pub fn build(self) -> Result<SoftTimeoutConfigOverride, BuildError> {
        Ok(SoftTimeoutConfigOverride {
            message: self.message,
            additional_soft_timeout_messages: self.additional_soft_timeout_messages,
        })
    }
}
