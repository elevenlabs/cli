pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferBranchInfoConfigured {
    #[serde(default)]
    pub branch_id: String,
}

impl TransferBranchInfoConfigured {
    pub fn builder() -> TransferBranchInfoConfiguredBuilder {
        <TransferBranchInfoConfiguredBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBranchInfoConfiguredBuilder {
    branch_id: Option<String>,
}

impl TransferBranchInfoConfiguredBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferBranchInfoConfigured`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](TransferBranchInfoConfiguredBuilder::branch_id)
    pub fn build(self) -> Result<TransferBranchInfoConfigured, BuildError> {
        Ok(TransferBranchInfoConfigured {
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}
