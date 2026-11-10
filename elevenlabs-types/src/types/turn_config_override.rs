pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnConfigOverride {
    /// Configuration for soft timeout functionality. Provides immediate feedback during longer LLM responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_config: Option<SoftTimeoutConfigOverride>,
    /// What the agent says when it detects a noisy environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noisy_environment_message: Option<String>,
}

impl TurnConfigOverride {
    pub fn builder() -> TurnConfigOverrideBuilder {
        <TurnConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnConfigOverrideBuilder {
    soft_timeout_config: Option<SoftTimeoutConfigOverride>,
    noisy_environment_message: Option<String>,
}

impl TurnConfigOverrideBuilder {
    pub fn soft_timeout_config(mut self, value: SoftTimeoutConfigOverride) -> Self {
        self.soft_timeout_config = Some(value);
        self
    }

    pub fn noisy_environment_message(mut self, value: impl Into<String>) -> Self {
        self.noisy_environment_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TurnConfigOverride`].
    pub fn build(self) -> Result<TurnConfigOverride, BuildError> {
        Ok(TurnConfigOverride {
            soft_timeout_config: self.soft_timeout_config,
            noisy_environment_message: self.noisy_environment_message,
        })
    }
}
