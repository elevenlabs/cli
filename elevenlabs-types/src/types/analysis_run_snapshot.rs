pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// LLM cost of the most recent post-call analysis pass on this conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisRunSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_feature: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_per_feature: Option<HashMap<String, i64>>,
}

impl AnalysisRunSnapshot {
    pub fn builder() -> AnalysisRunSnapshotBuilder {
        <AnalysisRunSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisRunSnapshotBuilder {
    price: Option<f64>,
    charge: Option<i64>,
    price_per_feature: Option<HashMap<String, f64>>,
    charge_per_feature: Option<HashMap<String, i64>>,
}

impl AnalysisRunSnapshotBuilder {
    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn charge(mut self, value: i64) -> Self {
        self.charge = Some(value);
        self
    }

    pub fn price_per_feature(mut self, value: HashMap<String, f64>) -> Self {
        self.price_per_feature = Some(value);
        self
    }

    pub fn charge_per_feature(mut self, value: HashMap<String, i64>) -> Self {
        self.charge_per_feature = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisRunSnapshot`].
    pub fn build(self) -> Result<AnalysisRunSnapshot, BuildError> {
        Ok(AnalysisRunSnapshot {
            price: self.price,
            charge: self.charge,
            price_per_feature: self.price_per_feature,
            charge_per_feature: self.charge_per_feature,
        })
    }
}
