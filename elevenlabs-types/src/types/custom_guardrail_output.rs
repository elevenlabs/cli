pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Container for custom guardrails, matching ModerationGuardrail pattern
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomGuardrailOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CustomGuardrailsConfigOutput>,
}

impl CustomGuardrailOutput {
    pub fn builder() -> CustomGuardrailOutputBuilder {
        <CustomGuardrailOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomGuardrailOutputBuilder {
    config: Option<CustomGuardrailsConfigOutput>,
}

impl CustomGuardrailOutputBuilder {
    pub fn config(mut self, value: CustomGuardrailsConfigOutput) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomGuardrailOutput`].
    pub fn build(self) -> Result<CustomGuardrailOutput, BuildError> {
        Ok(CustomGuardrailOutput {
            config: self.config,
        })
    }
}
