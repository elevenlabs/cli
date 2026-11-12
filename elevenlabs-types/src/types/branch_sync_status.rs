pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BranchSyncStatus {
    /// Latest automatic attempt to pull the synced branch's new versions in, run after each of them. Null when none has run since sync was turned on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt: Option<AutoRebaseAttempt>,
}

impl BranchSyncStatus {
    pub fn builder() -> BranchSyncStatusBuilder {
        <BranchSyncStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BranchSyncStatusBuilder {
    last_attempt: Option<AutoRebaseAttempt>,
}

impl BranchSyncStatusBuilder {
    pub fn last_attempt(mut self, value: AutoRebaseAttempt) -> Self {
        self.last_attempt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BranchSyncStatus`].
    pub fn build(self) -> Result<BranchSyncStatus, BuildError> {
        Ok(BranchSyncStatus {
            last_attempt: self.last_attempt,
        })
    }
}
