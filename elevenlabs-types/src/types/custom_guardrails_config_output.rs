pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Config container for custom guardrails list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomGuardrailsConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<CustomGuardrailConfig>>,
}

impl CustomGuardrailsConfigOutput {
    pub fn builder() -> CustomGuardrailsConfigOutputBuilder {
        <CustomGuardrailsConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomGuardrailsConfigOutputBuilder {
    configs: Option<Vec<CustomGuardrailConfig>>,
}

impl CustomGuardrailsConfigOutputBuilder {
    pub fn configs(mut self, value: Vec<CustomGuardrailConfig>) -> Self {
        self.configs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomGuardrailsConfigOutput`].
    pub fn build(self) -> Result<CustomGuardrailsConfigOutput, BuildError> {
        Ok(CustomGuardrailsConfigOutput {
            configs: self.configs,
        })
    }
}
