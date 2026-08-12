pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Single custom guardrail configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomGuardrailConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// User-facing name for this guardrail
    #[serde(default)]
    pub name: String,
    /// Instruction describing what to block, e.g. 'don't talk about politics'
    #[serde(default)]
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<GuardrailExecutionMode>,
    /// LLM model to use for custom guardrail evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<CustomGuardrailConfigModel>,
    /// How much recent history the guardrail sees before the reply it evaluates, counted in user messages (the agent replies between them are included too). The guardrail always gets a single <conversation_history> transcript ending in the evaluated reply, marked 'AGENT [current reply]:'. 0 (default) adds no prior history (just that line); 1 adds the latest user message onward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_message_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_action: Option<CustomGuardrailConfigTriggerAction>,
    /// Evaluate once against the complete non-TTS response instead of cumulative partials. Requires blocking mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_full_response_only: Option<bool>,
}

impl CustomGuardrailConfig {
    pub fn builder() -> CustomGuardrailConfigBuilder {
        <CustomGuardrailConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomGuardrailConfigBuilder {
    is_enabled: Option<bool>,
    name: Option<String>,
    prompt: Option<String>,
    execution_mode: Option<GuardrailExecutionMode>,
    model: Option<CustomGuardrailConfigModel>,
    history_message_count: Option<i64>,
    trigger_action: Option<CustomGuardrailConfigTriggerAction>,
    evaluate_full_response_only: Option<bool>,
}

impl CustomGuardrailConfigBuilder {
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn execution_mode(mut self, value: GuardrailExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn model(mut self, value: CustomGuardrailConfigModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn history_message_count(mut self, value: i64) -> Self {
        self.history_message_count = Some(value);
        self
    }

    pub fn trigger_action(mut self, value: CustomGuardrailConfigTriggerAction) -> Self {
        self.trigger_action = Some(value);
        self
    }

    pub fn evaluate_full_response_only(mut self, value: bool) -> Self {
        self.evaluate_full_response_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomGuardrailConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CustomGuardrailConfigBuilder::name)
    /// - [`prompt`](CustomGuardrailConfigBuilder::prompt)
    pub fn build(self) -> Result<CustomGuardrailConfig, BuildError> {
        Ok(CustomGuardrailConfig {
            is_enabled: self.is_enabled,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            execution_mode: self.execution_mode,
            model: self.model,
            history_message_count: self.history_message_count,
            trigger_action: self.trigger_action,
            evaluate_full_response_only: self.evaluate_full_response_only,
        })
    }
}
