pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromptInjectionGuardrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

impl PromptInjectionGuardrail {
    pub fn builder() -> PromptInjectionGuardrailBuilder {
        <PromptInjectionGuardrailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptInjectionGuardrailBuilder {
    is_enabled: Option<bool>,
}

impl PromptInjectionGuardrailBuilder {
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptInjectionGuardrail`].
    pub fn build(self) -> Result<PromptInjectionGuardrail, BuildError> {
        Ok(PromptInjectionGuardrail {
            is_enabled: self.is_enabled,
        })
    }
}
