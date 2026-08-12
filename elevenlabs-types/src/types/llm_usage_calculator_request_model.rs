pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LlmUsageCalculatorRequestModel {
    /// Length of the prompt in characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_length: Option<i64>,
    /// Pages of content in pdf documents OR urls in agent's Knowledge Base.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_pages: Option<i64>,
    /// Whether RAG is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_enabled: Option<bool>,
}

impl LlmUsageCalculatorRequestModel {
    pub fn builder() -> LlmUsageCalculatorRequestModelBuilder {
        <LlmUsageCalculatorRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageCalculatorRequestModelBuilder {
    prompt_length: Option<i64>,
    number_of_pages: Option<i64>,
    rag_enabled: Option<bool>,
}

impl LlmUsageCalculatorRequestModelBuilder {
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

    /// Consumes the builder and constructs a [`LlmUsageCalculatorRequestModel`].
    pub fn build(self) -> Result<LlmUsageCalculatorRequestModel, BuildError> {
        Ok(LlmUsageCalculatorRequestModel {
            prompt_length: self.prompt_length,
            number_of_pages: self.number_of_pages,
            rag_enabled: self.rag_enabled,
        })
    }
}

