pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Cumulative LLM cost of running post-call analysis on this conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisRunningTotal {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_feature: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_per_feature: Option<HashMap<String, i64>>,
}

impl AnalysisRunningTotal {
    pub fn builder() -> AnalysisRunningTotalBuilder {
        <AnalysisRunningTotalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisRunningTotalBuilder {
    price: Option<f64>,
    charge: Option<i64>,
    runs: Option<i64>,
    price_per_feature: Option<HashMap<String, f64>>,
    charge_per_feature: Option<HashMap<String, i64>>,
}

impl AnalysisRunningTotalBuilder {
    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn charge(mut self, value: i64) -> Self {
        self.charge = Some(value);
        self
    }

    pub fn runs(mut self, value: i64) -> Self {
        self.runs = Some(value);
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

    /// Consumes the builder and constructs a [`AnalysisRunningTotal`].
    pub fn build(self) -> Result<AnalysisRunningTotal, BuildError> {
        Ok(AnalysisRunningTotal {
            price: self.price,
            charge: self.charge,
            runs: self.runs,
            price_per_feature: self.price_per_feature,
            charge_per_feature: self.charge_per_feature,
        })
    }
}
