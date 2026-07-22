pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationGuardrailInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<GuardrailExecutionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ModerationConfig>,
}

impl ModerationGuardrailInput {
    pub fn builder() -> ModerationGuardrailInputBuilder {
        <ModerationGuardrailInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationGuardrailInputBuilder {
    execution_mode: Option<GuardrailExecutionMode>,
    config: Option<ModerationConfig>,
}

impl ModerationGuardrailInputBuilder {
    pub fn execution_mode(mut self, value: GuardrailExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn config(mut self, value: ModerationConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationGuardrailInput`].
    pub fn build(self) -> Result<ModerationGuardrailInput, BuildError> {
        Ok(ModerationGuardrailInput {
            execution_mode: self.execution_mode,
            config: self.config,
        })
    }
}
