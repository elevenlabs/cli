pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Alerting configuration used at both per-agent and per-workspace level.
/// 
/// All fields are optional overrides; the cascade resolver fills in defaults
/// when they are unset. Notifiers stack and dedupe (by webhook_id) across the
/// workspace and agent layers rather than overriding each other.
/// 
/// Cascade order for per-monitor threshold and auto-resolve: agent → workspace →
/// system default.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AlertingSettings {
    /// Alerting configuration keyed by monitor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_configs: Option<HashMap<String, AlertingMonitorConfig>>,
    /// How many minutes an alert can stay inactive before it is auto-resolved. Unset values fall through to the next layer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_resolve_after_inactive_minutes: Option<i64>,
    /// Delivery channels for alert lifecycle notifications. Stacked and deduped by webhook_id with other layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifiers: Option<Vec<AlertingWebhookNotifier>>,
}

impl AlertingSettings {
    pub fn builder() -> AlertingSettingsBuilder {
        <AlertingSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AlertingSettingsBuilder {
    monitor_configs: Option<HashMap<String, AlertingMonitorConfig>>,
    auto_resolve_after_inactive_minutes: Option<i64>,
    notifiers: Option<Vec<AlertingWebhookNotifier>>,
}

impl AlertingSettingsBuilder {
    pub fn monitor_configs(mut self, value: HashMap<String, AlertingMonitorConfig>) -> Self {
        self.monitor_configs = Some(value);
        self
    }

    pub fn auto_resolve_after_inactive_minutes(mut self, value: i64) -> Self {
        self.auto_resolve_after_inactive_minutes = Some(value);
        self
    }

    pub fn notifiers(mut self, value: Vec<AlertingWebhookNotifier>) -> Self {
        self.notifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AlertingSettings`].
    pub fn build(self) -> Result<AlertingSettings, BuildError> {
        Ok(AlertingSettings {
            monitor_configs: self.monitor_configs,
            auto_resolve_after_inactive_minutes: self.auto_resolve_after_inactive_minutes,
            notifiers: self.notifiers,
        })
    }
}
