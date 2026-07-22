pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferBranchInfoTrafficSplit {
    #[serde(default)]
    pub branch_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub traffic_percentage: f64,
}

impl TransferBranchInfoTrafficSplit {
    pub fn builder() -> TransferBranchInfoTrafficSplitBuilder {
        <TransferBranchInfoTrafficSplitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBranchInfoTrafficSplitBuilder {
    branch_id: Option<String>,
    traffic_percentage: Option<f64>,
}

impl TransferBranchInfoTrafficSplitBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn traffic_percentage(mut self, value: f64) -> Self {
        self.traffic_percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferBranchInfoTrafficSplit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](TransferBranchInfoTrafficSplitBuilder::branch_id)
    /// - [`traffic_percentage`](TransferBranchInfoTrafficSplitBuilder::traffic_percentage)
    pub fn build(self) -> Result<TransferBranchInfoTrafficSplit, BuildError> {
        Ok(TransferBranchInfoTrafficSplit {
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
            traffic_percentage: self.traffic_percentage.ok_or_else(|| BuildError::missing_field("traffic_percentage"))?,
        })
    }
}
