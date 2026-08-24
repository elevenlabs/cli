pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferBranchInfoDefaultingToMain {
    #[serde(default)]
    pub branch_id: String,
}

impl TransferBranchInfoDefaultingToMain {
    pub fn builder() -> TransferBranchInfoDefaultingToMainBuilder {
        <TransferBranchInfoDefaultingToMainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBranchInfoDefaultingToMainBuilder {
    branch_id: Option<String>,
}

impl TransferBranchInfoDefaultingToMainBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferBranchInfoDefaultingToMain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](TransferBranchInfoDefaultingToMainBuilder::branch_id)
    pub fn build(self) -> Result<TransferBranchInfoDefaultingToMain, BuildError> {
        Ok(TransferBranchInfoDefaultingToMain {
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}
