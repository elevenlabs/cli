pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutoSyncInfo {
    /// Minimum frequency (in days) at which the document is refreshed. The actual interval may be shorter, never longer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_frequency_days: Option<i64>,
    /// Whether to remove the document if the URL becomes unavailable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Number of consecutive sync failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consec_failures: Option<i64>,
    /// Unix timestamp for the next scheduled sync or None (in case of folders)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_refresh_by: Option<i64>,
}

impl AutoSyncInfo {
    pub fn builder() -> AutoSyncInfoBuilder {
        <AutoSyncInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutoSyncInfoBuilder {
    minimum_frequency_days: Option<i64>,
    auto_remove: Option<bool>,
    consec_failures: Option<i64>,
    next_refresh_by: Option<i64>,
}

impl AutoSyncInfoBuilder {
    pub fn minimum_frequency_days(mut self, value: i64) -> Self {
        self.minimum_frequency_days = Some(value);
        self
    }

    pub fn auto_remove(mut self, value: bool) -> Self {
        self.auto_remove = Some(value);
        self
    }

    pub fn consec_failures(mut self, value: i64) -> Self {
        self.consec_failures = Some(value);
        self
    }

    pub fn next_refresh_by(mut self, value: i64) -> Self {
        self.next_refresh_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutoSyncInfo`].
    pub fn build(self) -> Result<AutoSyncInfo, BuildError> {
        Ok(AutoSyncInfo {
            minimum_frequency_days: self.minimum_frequency_days,
            auto_remove: self.auto_remove,
            consec_failures: self.consec_failures,
            next_refresh_by: self.next_refresh_by,
        })
    }
}
