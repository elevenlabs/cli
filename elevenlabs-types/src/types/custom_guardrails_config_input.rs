pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Config container for custom guardrails list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomGuardrailsConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<CustomGuardrailConfig>>,
}

impl CustomGuardrailsConfigInput {
    pub fn builder() -> CustomGuardrailsConfigInputBuilder {
        <CustomGuardrailsConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomGuardrailsConfigInputBuilder {
    configs: Option<Vec<CustomGuardrailConfig>>,
}

impl CustomGuardrailsConfigInputBuilder {
    pub fn configs(mut self, value: Vec<CustomGuardrailConfig>) -> Self {
        self.configs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomGuardrailsConfigInput`].
    pub fn build(self) -> Result<CustomGuardrailsConfigInput, BuildError> {
        Ok(CustomGuardrailsConfigInput {
            configs: self.configs,
        })
    }
}
