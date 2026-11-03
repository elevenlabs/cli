pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnConfigOverrideConfig {
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_config: Option<SoftTimeoutConfigOverrideConfig>,
    /// Whether to allow overriding the noisy_environment_message field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noisy_environment_message: Option<bool>,
}

impl TurnConfigOverrideConfig {
    pub fn builder() -> TurnConfigOverrideConfigBuilder {
        <TurnConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnConfigOverrideConfigBuilder {
    soft_timeout_config: Option<SoftTimeoutConfigOverrideConfig>,
    noisy_environment_message: Option<bool>,
}

impl TurnConfigOverrideConfigBuilder {
    pub fn soft_timeout_config(mut self, value: SoftTimeoutConfigOverrideConfig) -> Self {
        self.soft_timeout_config = Some(value);
        self
    }

    pub fn noisy_environment_message(mut self, value: bool) -> Self {
        self.noisy_environment_message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurnConfigOverrideConfig`].
    pub fn build(self) -> Result<TurnConfigOverrideConfig, BuildError> {
        Ok(TurnConfigOverrideConfig {
            soft_timeout_config: self.soft_timeout_config,
            noisy_environment_message: self.noisy_environment_message,
        })
    }
}
