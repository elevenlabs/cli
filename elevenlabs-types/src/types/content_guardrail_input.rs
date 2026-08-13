pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContentGuardrailInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<GuardrailExecutionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ContentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_action: Option<ContentGuardrailInputTriggerAction>,
}

impl ContentGuardrailInput {
    pub fn builder() -> ContentGuardrailInputBuilder {
        <ContentGuardrailInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentGuardrailInputBuilder {
    execution_mode: Option<GuardrailExecutionMode>,
    config: Option<ContentConfig>,
    trigger_action: Option<ContentGuardrailInputTriggerAction>,
}

impl ContentGuardrailInputBuilder {
    pub fn execution_mode(mut self, value: GuardrailExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn config(mut self, value: ContentConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn trigger_action(mut self, value: ContentGuardrailInputTriggerAction) -> Self {
        self.trigger_action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentGuardrailInput`].
    pub fn build(self) -> Result<ContentGuardrailInput, BuildError> {
        Ok(ContentGuardrailInput {
            execution_mode: self.execution_mode,
            config: self.config,
            trigger_action: self.trigger_action,
        })
    }
}
