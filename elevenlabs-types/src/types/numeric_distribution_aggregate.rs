pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NumericDistributionAggregate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max: Option<f64>,
}

impl NumericDistributionAggregate {
    pub fn builder() -> NumericDistributionAggregateBuilder {
        <NumericDistributionAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NumericDistributionAggregateBuilder {
    count: Option<i64>,
    sum: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
}

impl NumericDistributionAggregateBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn sum(mut self, value: f64) -> Self {
        self.sum = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NumericDistributionAggregate`].
    pub fn build(self) -> Result<NumericDistributionAggregate, BuildError> {
        Ok(NumericDistributionAggregate {
            count: self.count,
            sum: self.sum,
            min: self.min,
            max: self.max,
        })
    }
}
