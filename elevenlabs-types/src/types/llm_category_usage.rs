pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// LLM usage of a conversation, split by whether the generated response reached the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmCategoryUsage {
    /// Subset of `initiated_generation`: usage of agent turns whose audio had started playing to the user, whether the turn then completed or was interrupted. A turn the user interrupted while the LLM was still generating, or that was cut off by the call ending, can be missing here even if part of it was heard. Informational; LLM charges are based on `initiated_generation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irreversible_generation: Option<LlmUsageOutput>,
    /// Usage of every LLM call made during the conversation: turns the user heard, turns the user interrupted, and speculative turns that were started and dropped before any audio was sent. LLM charges are based on this block; its prices sum to `llm_price`. Always greater than or equal to `irreversible_generation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_generation: Option<LlmUsageOutput>,
}

impl LlmCategoryUsage {
    pub fn builder() -> LlmCategoryUsageBuilder {
        <LlmCategoryUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmCategoryUsageBuilder {
    irreversible_generation: Option<LlmUsageOutput>,
    initiated_generation: Option<LlmUsageOutput>,
}

impl LlmCategoryUsageBuilder {
    pub fn irreversible_generation(mut self, value: LlmUsageOutput) -> Self {
        self.irreversible_generation = Some(value);
        self
    }

    pub fn initiated_generation(mut self, value: LlmUsageOutput) -> Self {
        self.initiated_generation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmCategoryUsage`].
    pub fn build(self) -> Result<LlmCategoryUsage, BuildError> {
        Ok(LlmCategoryUsage {
            irreversible_generation: self.irreversible_generation,
            initiated_generation: self.initiated_generation,
        })
    }
}
