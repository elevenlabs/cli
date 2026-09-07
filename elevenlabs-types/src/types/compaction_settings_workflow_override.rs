pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompactionSettingsWorkflowOverride {
    /// Whether context compaction is enabled for this agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Context window fraction at which compaction is triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub soft_trigger_fraction: Option<f64>,
    /// Number of recent messages kept verbatim after compaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_size: Option<i64>,
    /// Minimum number of tokens that compaction must reclaim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_reclaimable_tokens: Option<i64>,
}

impl CompactionSettingsWorkflowOverride {
    pub fn builder() -> CompactionSettingsWorkflowOverrideBuilder {
        <CompactionSettingsWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompactionSettingsWorkflowOverrideBuilder {
    enabled: Option<bool>,
    soft_trigger_fraction: Option<f64>,
    tail_size: Option<i64>,
    min_reclaimable_tokens: Option<i64>,
}

impl CompactionSettingsWorkflowOverrideBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn soft_trigger_fraction(mut self, value: f64) -> Self {
        self.soft_trigger_fraction = Some(value);
        self
    }

    pub fn tail_size(mut self, value: i64) -> Self {
        self.tail_size = Some(value);
        self
    }

    pub fn min_reclaimable_tokens(mut self, value: i64) -> Self {
        self.min_reclaimable_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompactionSettingsWorkflowOverride`].
    pub fn build(self) -> Result<CompactionSettingsWorkflowOverride, BuildError> {
        Ok(CompactionSettingsWorkflowOverride {
            enabled: self.enabled,
            soft_trigger_fraction: self.soft_trigger_fraction,
            tail_size: self.tail_size,
            min_reclaimable_tokens: self.min_reclaimable_tokens,
        })
    }
}
