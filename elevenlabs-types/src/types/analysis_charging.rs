pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Cost of running post-call analysis on this conversation.
/// Present once an analysis pass has run, billed or not.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisCharging {
    #[serde(default)]
    pub total: AnalysisRunningTotal,
    #[serde(default)]
    pub last_run: AnalysisRunSnapshot,
}

impl AnalysisCharging {
    pub fn builder() -> AnalysisChargingBuilder {
        <AnalysisChargingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisChargingBuilder {
    total: Option<AnalysisRunningTotal>,
    last_run: Option<AnalysisRunSnapshot>,
}

impl AnalysisChargingBuilder {
    pub fn total(mut self, value: AnalysisRunningTotal) -> Self {
        self.total = Some(value);
        self
    }

    pub fn last_run(mut self, value: AnalysisRunSnapshot) -> Self {
        self.last_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisCharging`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](AnalysisChargingBuilder::total)
    /// - [`last_run`](AnalysisChargingBuilder::last_run)
    pub fn build(self) -> Result<AnalysisCharging, BuildError> {
        Ok(AnalysisCharging {
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            last_run: self.last_run.ok_or_else(|| BuildError::missing_field("last_run"))?,
        })
    }
}
