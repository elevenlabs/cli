pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmUsageOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_usage: Option<HashMap<String, LlmInputOutputTokensUsage>>,
}

impl LlmUsageOutput {
    pub fn builder() -> LlmUsageOutputBuilder {
        <LlmUsageOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageOutputBuilder {
    model_usage: Option<HashMap<String, LlmInputOutputTokensUsage>>,
}

impl LlmUsageOutputBuilder {
    pub fn model_usage(mut self, value: HashMap<String, LlmInputOutputTokensUsage>) -> Self {
        self.model_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmUsageOutput`].
    pub fn build(self) -> Result<LlmUsageOutput, BuildError> {
        Ok(LlmUsageOutput {
            model_usage: self.model_usage,
        })
    }
}
