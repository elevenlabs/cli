pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LlmTokensCategoryUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
}

impl LlmTokensCategoryUsage {
    pub fn builder() -> LlmTokensCategoryUsageBuilder {
        <LlmTokensCategoryUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LlmTokensCategoryUsageBuilder {
    tokens: Option<i64>,
    price: Option<f64>,
}

impl LlmTokensCategoryUsageBuilder {
    pub fn tokens(mut self, value: i64) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LlmTokensCategoryUsage`].
    pub fn build(self) -> Result<LlmTokensCategoryUsage, BuildError> {
        Ok(LlmTokensCategoryUsage {
            tokens: self.tokens,
            price: self.price,
        })
    }
}
