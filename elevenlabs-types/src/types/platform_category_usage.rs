pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Accumulated charge for a single :class:`PlatformCategory`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PlatformCategoryUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub quantity: Option<f64>,
}

impl PlatformCategoryUsage {
    pub fn builder() -> PlatformCategoryUsageBuilder {
        <PlatformCategoryUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformCategoryUsageBuilder {
    credits: Option<i64>,
    price: Option<f64>,
    quantity: Option<f64>,
}

impl PlatformCategoryUsageBuilder {
    pub fn credits(mut self, value: i64) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformCategoryUsage`].
    pub fn build(self) -> Result<PlatformCategoryUsage, BuildError> {
        Ok(PlatformCategoryUsage {
            credits: self.credits,
            price: self.price,
            quantity: self.quantity,
        })
    }
}
