pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RealtimeConfigSnapshotParents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_snapshot_id: Option<String>,
}

impl RealtimeConfigSnapshotParents {
    pub fn builder() -> RealtimeConfigSnapshotParentsBuilder {
        <RealtimeConfigSnapshotParentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeConfigSnapshotParentsBuilder {
    previous_snapshot_id: Option<String>,
}

impl RealtimeConfigSnapshotParentsBuilder {
    pub fn previous_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.previous_snapshot_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RealtimeConfigSnapshotParents`].
    pub fn build(self) -> Result<RealtimeConfigSnapshotParents, BuildError> {
        Ok(RealtimeConfigSnapshotParents {
            previous_snapshot_id: self.previous_snapshot_id,
        })
    }
}
