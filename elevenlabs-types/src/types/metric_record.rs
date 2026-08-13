pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MetricRecord {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub elapsed_time: f64,
}

impl MetricRecord {
    pub fn builder() -> MetricRecordBuilder {
        <MetricRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricRecordBuilder {
    elapsed_time: Option<f64>,
}

impl MetricRecordBuilder {
    pub fn elapsed_time(mut self, value: f64) -> Self {
        self.elapsed_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`elapsed_time`](MetricRecordBuilder::elapsed_time)
    pub fn build(self) -> Result<MetricRecord, BuildError> {
        Ok(MetricRecord {
            elapsed_time: self.elapsed_time.ok_or_else(|| BuildError::missing_field("elapsed_time"))?,
        })
    }
}
