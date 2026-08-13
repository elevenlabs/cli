pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnConfigOverrideConfig {
    /// Configures overrides for nested fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_config: Option<SoftTimeoutConfigOverrideConfig>,
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
}

impl TurnConfigOverrideConfigBuilder {
    pub fn soft_timeout_config(mut self, value: SoftTimeoutConfigOverrideConfig) -> Self {
        self.soft_timeout_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurnConfigOverrideConfig`].
    pub fn build(self) -> Result<TurnConfigOverrideConfig, BuildError> {
        Ok(TurnConfigOverrideConfig {
            soft_timeout_config: self.soft_timeout_config,
        })
    }
}
