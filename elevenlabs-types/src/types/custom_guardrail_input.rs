pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Container for custom guardrails, matching ModerationGuardrail pattern
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomGuardrailInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CustomGuardrailsConfigInput>,
}

impl CustomGuardrailInput {
    pub fn builder() -> CustomGuardrailInputBuilder {
        <CustomGuardrailInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomGuardrailInputBuilder {
    config: Option<CustomGuardrailsConfigInput>,
}

impl CustomGuardrailInputBuilder {
    pub fn config(mut self, value: CustomGuardrailsConfigInput) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomGuardrailInput`].
    pub fn build(self) -> Result<CustomGuardrailInput, BuildError> {
        Ok(CustomGuardrailInput {
            config: self.config,
        })
    }
}
