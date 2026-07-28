pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmCategoryUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irreversible_generation: Option<LlmUsageOutput>,
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
