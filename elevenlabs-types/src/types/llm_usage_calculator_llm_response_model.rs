pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LlmUsageCalculatorLlmResponseModel {
    pub llm: Llm,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub price_per_minute: f64,
}

impl LlmUsageCalculatorLlmResponseModel {
    pub fn builder() -> LlmUsageCalculatorLlmResponseModelBuilder {
        <LlmUsageCalculatorLlmResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmUsageCalculatorLlmResponseModelBuilder {
    llm: Option<Llm>,
    price_per_minute: Option<f64>,
}

impl LlmUsageCalculatorLlmResponseModelBuilder {
    pub fn llm(mut self, value: Llm) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn price_per_minute(mut self, value: f64) -> Self {
        self.price_per_minute = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmUsageCalculatorLlmResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm`](LlmUsageCalculatorLlmResponseModelBuilder::llm)
    /// - [`price_per_minute`](LlmUsageCalculatorLlmResponseModelBuilder::price_per_minute)
    pub fn build(self) -> Result<LlmUsageCalculatorLlmResponseModel, BuildError> {
        Ok(LlmUsageCalculatorLlmResponseModel {
            llm: self.llm.ok_or_else(|| BuildError::missing_field("llm"))?,
            price_per_minute: self.price_per_minute.ok_or_else(|| BuildError::missing_field("price_per_minute"))?,
        })
    }
}
