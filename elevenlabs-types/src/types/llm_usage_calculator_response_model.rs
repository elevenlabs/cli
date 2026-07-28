pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmUsageCalculatorResponseModel {
    #[serde(default)]
    pub llm_prices: Vec<LlmUsageCalculatorLlmResponseModel>,
}

impl LlmUsageCalculatorResponseModel {
    pub fn builder() -> LlmUsageCalculatorResponseModelBuilder {
        <LlmUsageCalculatorResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageCalculatorResponseModelBuilder {
    llm_prices: Option<Vec<LlmUsageCalculatorLlmResponseModel>>,
}

impl LlmUsageCalculatorResponseModelBuilder {
    pub fn llm_prices(mut self, value: Vec<LlmUsageCalculatorLlmResponseModel>) -> Self {
        self.llm_prices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmUsageCalculatorResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm_prices`](LlmUsageCalculatorResponseModelBuilder::llm_prices)
    pub fn build(self) -> Result<LlmUsageCalculatorResponseModel, BuildError> {
        Ok(LlmUsageCalculatorResponseModel {
            llm_prices: self.llm_prices.ok_or_else(|| BuildError::missing_field("llm_prices"))?,
        })
    }
}
