pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GuardrailsV1Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus: Option<FocusGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_injection: Option<PromptInjectionGuardrail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentGuardrailInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomGuardrailInput>,
}

impl GuardrailsV1Input {
    pub fn builder() -> GuardrailsV1InputBuilder {
        <GuardrailsV1InputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GuardrailsV1InputBuilder {
    version: Option<String>,
    focus: Option<FocusGuardrail>,
    prompt_injection: Option<PromptInjectionGuardrail>,
    content: Option<ContentGuardrailInput>,
    custom: Option<CustomGuardrailInput>,
}

impl GuardrailsV1InputBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn focus(mut self, value: FocusGuardrail) -> Self {
        self.focus = Some(value);
        self
    }

    pub fn prompt_injection(mut self, value: PromptInjectionGuardrail) -> Self {
        self.prompt_injection = Some(value);
        self
    }

    pub fn content(mut self, value: ContentGuardrailInput) -> Self {
        self.content = Some(value);
        self
    }

    pub fn custom(mut self, value: CustomGuardrailInput) -> Self {
        self.custom = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GuardrailsV1Input`].
    pub fn build(self) -> Result<GuardrailsV1Input, BuildError> {
        Ok(GuardrailsV1Input {
            version: self.version,
            focus: self.focus,
            prompt_injection: self.prompt_injection,
            content: self.content,
            custom: self.custom,
        })
    }
}
