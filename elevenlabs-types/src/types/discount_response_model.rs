pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DiscountResponseModel {
    /// The discount applied to the invoice. E.g. [20.0f] for 20% off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount_percent_off: Option<f64>,
    /// The discount applied to the invoice. E.g. [20.0f] for 20 cents off.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount_amount_off: Option<f64>,
}

impl DiscountResponseModel {
    pub fn builder() -> DiscountResponseModelBuilder {
        <DiscountResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiscountResponseModelBuilder {
    discount_percent_off: Option<f64>,
    discount_amount_off: Option<f64>,
}

impl DiscountResponseModelBuilder {
    pub fn discount_percent_off(mut self, value: f64) -> Self {
        self.discount_percent_off = Some(value);
        self
    }

    pub fn discount_amount_off(mut self, value: f64) -> Self {
        self.discount_amount_off = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DiscountResponseModel`].
    pub fn build(self) -> Result<DiscountResponseModel, BuildError> {
        Ok(DiscountResponseModel {
            discount_percent_off: self.discount_percent_off,
            discount_amount_off: self.discount_amount_off,
        })
    }
}
