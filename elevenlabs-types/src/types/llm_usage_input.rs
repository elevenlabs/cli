pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmUsageInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_usage: Option<HashMap<String, LlmInputOutputTokensUsage>>,
}

impl LlmUsageInput {
    pub fn builder() -> LlmUsageInputBuilder {
        <LlmUsageInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageInputBuilder {
    model_usage: Option<HashMap<String, LlmInputOutputTokensUsage>>,
}

impl LlmUsageInputBuilder {
    pub fn model_usage(mut self, value: HashMap<String, LlmInputOutputTokensUsage>) -> Self {
        self.model_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmUsageInput`].
    pub fn build(self) -> Result<LlmUsageInput, BuildError> {
        Ok(LlmUsageInput {
            model_usage: self.model_usage,
        })
    }
}
