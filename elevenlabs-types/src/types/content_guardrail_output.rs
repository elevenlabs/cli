pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContentGuardrailOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<GuardrailExecutionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ContentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_action: Option<ContentGuardrailOutputTriggerAction>,
}

impl ContentGuardrailOutput {
    pub fn builder() -> ContentGuardrailOutputBuilder {
        <ContentGuardrailOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentGuardrailOutputBuilder {
    execution_mode: Option<GuardrailExecutionMode>,
    config: Option<ContentConfig>,
    trigger_action: Option<ContentGuardrailOutputTriggerAction>,
}

impl ContentGuardrailOutputBuilder {
    pub fn execution_mode(mut self, value: GuardrailExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn config(mut self, value: ContentConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn trigger_action(mut self, value: ContentGuardrailOutputTriggerAction) -> Self {
        self.trigger_action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentGuardrailOutput`].
    pub fn build(self) -> Result<ContentGuardrailOutput, BuildError> {
        Ok(ContentGuardrailOutput {
            execution_mode: self.execution_mode,
            config: self.config,
            trigger_action: self.trigger_action,
        })
    }
}
