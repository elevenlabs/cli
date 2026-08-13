pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContentConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sexual: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violence: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harassment: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_harm: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub religion_or_politics: Option<ContentThresholdGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_and_legal_information: Option<ContentThresholdGuardrail>,
}

impl ContentConfig {
    pub fn builder() -> ContentConfigBuilder {
        <ContentConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentConfigBuilder {
    sexual: Option<ContentThresholdGuardrail>,
    violence: Option<ContentThresholdGuardrail>,
    harassment: Option<ContentThresholdGuardrail>,
    self_harm: Option<ContentThresholdGuardrail>,
    profanity: Option<ContentThresholdGuardrail>,
    religion_or_politics: Option<ContentThresholdGuardrail>,
    medical_and_legal_information: Option<ContentThresholdGuardrail>,
}

impl ContentConfigBuilder {
    pub fn sexual(mut self, value: ContentThresholdGuardrail) -> Self {
        self.sexual = Some(value);
        self
    }

    pub fn violence(mut self, value: ContentThresholdGuardrail) -> Self {
        self.violence = Some(value);
        self
    }

    pub fn harassment(mut self, value: ContentThresholdGuardrail) -> Self {
        self.harassment = Some(value);
        self
    }

    pub fn self_harm(mut self, value: ContentThresholdGuardrail) -> Self {
        self.self_harm = Some(value);
        self
    }

    pub fn profanity(mut self, value: ContentThresholdGuardrail) -> Self {
        self.profanity = Some(value);
        self
    }

    pub fn religion_or_politics(mut self, value: ContentThresholdGuardrail) -> Self {
        self.religion_or_politics = Some(value);
        self
    }

    pub fn medical_and_legal_information(mut self, value: ContentThresholdGuardrail) -> Self {
        self.medical_and_legal_information = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentConfig`].
    pub fn build(self) -> Result<ContentConfig, BuildError> {
        Ok(ContentConfig {
            sexual: self.sexual,
            violence: self.violence,
            harassment: self.harassment,
            self_harm: self.self_harm,
            profanity: self.profanity,
            religion_or_politics: self.religion_or_politics,
            medical_and_legal_information: self.medical_and_legal_information,
        })
    }
}
