pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AlertingMonitorConfig {
    /// Failure rate threshold at which this monitor can notify.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threshold: Option<f64>,
    /// Relative increase over the trailing baseline at which this monitor can notify (0.2 = 20% above baseline, 0 = any failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub relative_increase_threshold: Option<f64>,
    /// Minimum failures in the window before this monitor can fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_failure_count: Option<i64>,
    /// Minimum trailing buckets with traffic before spike detection can fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_history_bucket_count: Option<i64>,
    /// Minimum samples in the window before this monitor can fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_sample_count: Option<i64>,
    /// How many suspect buckets within the lookback window are required to promote a suspect to an alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspect_trigger_threshold: Option<i64>,
    /// How many minutes an alert can stay inactive before it is auto-resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_resolve_after_inactive_minutes: Option<i64>,
}

impl AlertingMonitorConfig {
    pub fn builder() -> AlertingMonitorConfigBuilder {
        <AlertingMonitorConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingMonitorConfigBuilder {
    threshold: Option<f64>,
    relative_increase_threshold: Option<f64>,
    min_failure_count: Option<i64>,
    min_history_bucket_count: Option<i64>,
    min_sample_count: Option<i64>,
    suspect_trigger_threshold: Option<i64>,
    auto_resolve_after_inactive_minutes: Option<i64>,
}

impl AlertingMonitorConfigBuilder {
    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
        self
    }

    pub fn relative_increase_threshold(mut self, value: f64) -> Self {
        self.relative_increase_threshold = Some(value);
        self
    }

    pub fn min_failure_count(mut self, value: i64) -> Self {
        self.min_failure_count = Some(value);
        self
    }

    pub fn min_history_bucket_count(mut self, value: i64) -> Self {
        self.min_history_bucket_count = Some(value);
        self
    }

    pub fn min_sample_count(mut self, value: i64) -> Self {
        self.min_sample_count = Some(value);
        self
    }

    pub fn suspect_trigger_threshold(mut self, value: i64) -> Self {
        self.suspect_trigger_threshold = Some(value);
        self
    }

    pub fn auto_resolve_after_inactive_minutes(mut self, value: i64) -> Self {
        self.auto_resolve_after_inactive_minutes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlertingMonitorConfig`].
    pub fn build(self) -> Result<AlertingMonitorConfig, BuildError> {
        Ok(AlertingMonitorConfig {
            threshold: self.threshold,
            relative_increase_threshold: self.relative_increase_threshold,
            min_failure_count: self.min_failure_count,
            min_history_bucket_count: self.min_history_bucket_count,
            min_sample_count: self.min_sample_count,
            suspect_trigger_threshold: self.suspect_trigger_threshold,
            auto_resolve_after_inactive_minutes: self.auto_resolve_after_inactive_minutes,
        })
    }
}
