pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmInputOutputTokensUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<LlmTokensCategoryUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cache_read: Option<LlmTokensCategoryUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cache_write: Option<LlmTokensCategoryUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_total: Option<LlmTokensCategoryUsage>,
}

impl LlmInputOutputTokensUsage {
    pub fn builder() -> LlmInputOutputTokensUsageBuilder {
        <LlmInputOutputTokensUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmInputOutputTokensUsageBuilder {
    input: Option<LlmTokensCategoryUsage>,
    input_cache_read: Option<LlmTokensCategoryUsage>,
    input_cache_write: Option<LlmTokensCategoryUsage>,
    output_total: Option<LlmTokensCategoryUsage>,
}

impl LlmInputOutputTokensUsageBuilder {
    pub fn input(mut self, value: LlmTokensCategoryUsage) -> Self {
        self.input = Some(value);
        self
    }

    pub fn input_cache_read(mut self, value: LlmTokensCategoryUsage) -> Self {
        self.input_cache_read = Some(value);
        self
    }

    pub fn input_cache_write(mut self, value: LlmTokensCategoryUsage) -> Self {
        self.input_cache_write = Some(value);
        self
    }

    pub fn output_total(mut self, value: LlmTokensCategoryUsage) -> Self {
        self.output_total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmInputOutputTokensUsage`].
    pub fn build(self) -> Result<LlmInputOutputTokensUsage, BuildError> {
        Ok(LlmInputOutputTokensUsage {
            input: self.input,
            input_cache_read: self.input_cache_read,
            input_cache_write: self.input_cache_write,
            output_total: self.output_total,
        })
    }
}
