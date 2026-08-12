pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RegionalProcessingSurchargeInfo {
    /// The surcharge multiplier applied to this model's pricing (e.g. 1.1 for a 10% surcharge).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub multiplier: f64,
}

impl RegionalProcessingSurchargeInfo {
    pub fn builder() -> RegionalProcessingSurchargeInfoBuilder {
        <RegionalProcessingSurchargeInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegionalProcessingSurchargeInfoBuilder {
    multiplier: Option<f64>,
}

impl RegionalProcessingSurchargeInfoBuilder {
    pub fn multiplier(mut self, value: f64) -> Self {
        self.multiplier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegionalProcessingSurchargeInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`multiplier`](RegionalProcessingSurchargeInfoBuilder::multiplier)
    pub fn build(self) -> Result<RegionalProcessingSurchargeInfo, BuildError> {
        Ok(RegionalProcessingSurchargeInfo {
            multiplier: self.multiplier.ok_or_else(|| BuildError::missing_field("multiplier"))?,
        })
    }
}
