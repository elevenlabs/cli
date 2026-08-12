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
    auto_resolve_after_inactive_minutes: Option<i64>,
}

impl AlertingMonitorConfigBuilder {
    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
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
            auto_resolve_after_inactive_minutes: self.auto_resolve_after_inactive_minutes,
        })
    }
}
