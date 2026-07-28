pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LlmSchemaOverride {
    /// Prompt override for the LLM. If not provided, the original schema description is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

impl LlmSchemaOverride {
    pub fn builder() -> LlmSchemaOverrideBuilder {
        <LlmSchemaOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmSchemaOverrideBuilder {
    prompt: Option<String>,
}

impl LlmSchemaOverrideBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LlmSchemaOverride`].
    pub fn build(self) -> Result<LlmSchemaOverride, BuildError> {
        Ok(LlmSchemaOverride {
            prompt: self.prompt,
        })
    }
}
