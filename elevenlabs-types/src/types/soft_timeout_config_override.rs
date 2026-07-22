pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SoftTimeoutConfigOverride {
    /// Message to show when the first soft timeout is reached while waiting for LLM response. Supports dynamic variables (e.g., {{system__time}}, {{custom_variable}}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
}

impl SoftTimeoutConfigOverrideBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SoftTimeoutConfigOverride`].
    pub fn build(self) -> Result<SoftTimeoutConfigOverride, BuildError> {
        Ok(SoftTimeoutConfigOverride {
            message: self.message,
        })
    }
}
