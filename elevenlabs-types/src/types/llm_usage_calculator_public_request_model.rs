pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LlmUsageCalculatorPublicRequestModel {
    /// Length of the prompt in characters.
    #[serde(default)]
    pub prompt_length: i64,
    /// Pages of content in PDF documents or URLs in the agent's knowledge base.
    #[serde(default)]
    pub number_of_pages: i64,
    /// Whether RAG is enabled.
    #[serde(default)]
    pub rag_enabled: bool,
}

impl LlmUsageCalculatorPublicRequestModel {
    pub fn builder() -> LlmUsageCalculatorPublicRequestModelBuilder {
        <LlmUsageCalculatorPublicRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageCalculatorPublicRequestModelBuilder {
    prompt_length: Option<i64>,
    number_of_pages: Option<i64>,
    rag_enabled: Option<bool>,
}

impl LlmUsageCalculatorPublicRequestModelBuilder {
    pub fn prompt_length(mut self, value: i64) -> Self {
        self.prompt_length = Some(value);
        self
    }

    pub fn number_of_pages(mut self, value: i64) -> Self {
        self.number_of_pages = Some(value);
        self
    }

    pub fn rag_enabled(mut self, value: bool) -> Self {
        self.rag_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmUsageCalculatorPublicRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_length`](LlmUsageCalculatorPublicRequestModelBuilder::prompt_length)
    /// - [`number_of_pages`](LlmUsageCalculatorPublicRequestModelBuilder::number_of_pages)
    /// - [`rag_enabled`](LlmUsageCalculatorPublicRequestModelBuilder::rag_enabled)
    pub fn build(self) -> Result<LlmUsageCalculatorPublicRequestModel, BuildError> {
        Ok(LlmUsageCalculatorPublicRequestModel {
            prompt_length: self.prompt_length.ok_or_else(|| BuildError::missing_field("prompt_length"))?,
            number_of_pages: self.number_of_pages.ok_or_else(|| BuildError::missing_field("number_of_pages"))?,
            rag_enabled: self.rag_enabled.ok_or_else(|| BuildError::missing_field("rag_enabled"))?,
        })
    }
}

