pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelRatesResponseModel {
    /// The cost multiplier for characters.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub character_cost_multiplier: f64,
    /// Discount multiplier applied to cost estimates. Defaults to 1.0 (no discount).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_discount_multiplier: Option<f64>,
}

impl ModelRatesResponseModel {
    pub fn builder() -> ModelRatesResponseModelBuilder {
        <ModelRatesResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelRatesResponseModelBuilder {
    character_cost_multiplier: Option<f64>,
    cost_discount_multiplier: Option<f64>,
}

impl ModelRatesResponseModelBuilder {
    pub fn character_cost_multiplier(mut self, value: f64) -> Self {
        self.character_cost_multiplier = Some(value);
        self
    }

    pub fn cost_discount_multiplier(mut self, value: f64) -> Self {
        self.cost_discount_multiplier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelRatesResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`character_cost_multiplier`](ModelRatesResponseModelBuilder::character_cost_multiplier)
    pub fn build(self) -> Result<ModelRatesResponseModel, BuildError> {
        Ok(ModelRatesResponseModel {
            character_cost_multiplier: self.character_cost_multiplier.ok_or_else(|| BuildError::missing_field("character_cost_multiplier"))?,
            cost_discount_multiplier: self.cost_discount_multiplier,
        })
    }
}
