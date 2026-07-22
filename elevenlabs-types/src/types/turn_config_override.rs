pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnConfigOverride {
    /// Configuration for soft timeout functionality. Provides immediate feedback during longer LLM responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_config: Option<SoftTimeoutConfigOverride>,
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
}

impl TurnConfigOverrideBuilder {
    pub fn soft_timeout_config(mut self, value: SoftTimeoutConfigOverride) -> Self {
        self.soft_timeout_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurnConfigOverride`].
    pub fn build(self) -> Result<TurnConfigOverride, BuildError> {
        Ok(TurnConfigOverride {
            soft_timeout_config: self.soft_timeout_config,
        })
    }
}
