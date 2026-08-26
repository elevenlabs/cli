pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SoftTimeoutConfigOverrideConfig {
    /// Whether to allow overriding the message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<bool>,
    /// Whether to allow overriding the additional_soft_timeout_messages field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_soft_timeout_messages: Option<bool>,
}

impl SoftTimeoutConfigOverrideConfig {
    pub fn builder() -> SoftTimeoutConfigOverrideConfigBuilder {
        <SoftTimeoutConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SoftTimeoutConfigOverrideConfigBuilder {
    message: Option<bool>,
    additional_soft_timeout_messages: Option<bool>,
}

impl SoftTimeoutConfigOverrideConfigBuilder {
    pub fn message(mut self, value: bool) -> Self {
        self.message = Some(value);
        self
    }

    pub fn additional_soft_timeout_messages(mut self, value: bool) -> Self {
        self.additional_soft_timeout_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SoftTimeoutConfigOverrideConfig`].
    pub fn build(self) -> Result<SoftTimeoutConfigOverrideConfig, BuildError> {
        Ok(SoftTimeoutConfigOverrideConfig {
            message: self.message,
            additional_soft_timeout_messages: self.additional_soft_timeout_messages,
        })
    }
}
