pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violence: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violence_graphic: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harassment: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harassment_threatening: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hate: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hate_threatening: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_harm_instructions: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_harm: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_harm_intent: Option<ThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual_minors: Option<ThresholdGuardrail>,
}

impl ModerationConfig {
    pub fn builder() -> ModerationConfigBuilder {
        <ModerationConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModerationConfigBuilder {
    sexual: Option<ThresholdGuardrail>,
    violence: Option<ThresholdGuardrail>,
    violence_graphic: Option<ThresholdGuardrail>,
    harassment: Option<ThresholdGuardrail>,
    harassment_threatening: Option<ThresholdGuardrail>,
    hate: Option<ThresholdGuardrail>,
    hate_threatening: Option<ThresholdGuardrail>,
    self_harm_instructions: Option<ThresholdGuardrail>,
    self_harm: Option<ThresholdGuardrail>,
    self_harm_intent: Option<ThresholdGuardrail>,
    sexual_minors: Option<ThresholdGuardrail>,
}

impl ModerationConfigBuilder {
    pub fn sexual(mut self, value: ThresholdGuardrail) -> Self {
        self.sexual = Some(value);
        self
    }

    pub fn violence(mut self, value: ThresholdGuardrail) -> Self {
        self.violence = Some(value);
        self
    }

    pub fn violence_graphic(mut self, value: ThresholdGuardrail) -> Self {
        self.violence_graphic = Some(value);
        self
    }

    pub fn harassment(mut self, value: ThresholdGuardrail) -> Self {
        self.harassment = Some(value);
        self
    }

    pub fn harassment_threatening(mut self, value: ThresholdGuardrail) -> Self {
        self.harassment_threatening = Some(value);
        self
    }

    pub fn hate(mut self, value: ThresholdGuardrail) -> Self {
        self.hate = Some(value);
        self
    }

    pub fn hate_threatening(mut self, value: ThresholdGuardrail) -> Self {
        self.hate_threatening = Some(value);
        self
    }

    pub fn self_harm_instructions(mut self, value: ThresholdGuardrail) -> Self {
        self.self_harm_instructions = Some(value);
        self
    }

    pub fn self_harm(mut self, value: ThresholdGuardrail) -> Self {
        self.self_harm = Some(value);
        self
    }

    pub fn self_harm_intent(mut self, value: ThresholdGuardrail) -> Self {
        self.self_harm_intent = Some(value);
        self
    }

    pub fn sexual_minors(mut self, value: ThresholdGuardrail) -> Self {
        self.sexual_minors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModerationConfig`].
    pub fn build(self) -> Result<ModerationConfig, BuildError> {
        Ok(ModerationConfig {
            sexual: self.sexual,
            violence: self.violence,
            violence_graphic: self.violence_graphic,
            harassment: self.harassment,
            harassment_threatening: self.harassment_threatening,
            hate: self.hate,
            hate_threatening: self.hate_threatening,
            self_harm_instructions: self.self_harm_instructions,
            self_harm: self.self_harm,
            self_harm_intent: self.self_harm_intent,
            sexual_minors: self.sexual_minors,
        })
    }
}
