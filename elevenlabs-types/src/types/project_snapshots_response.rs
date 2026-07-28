pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectSnapshotsResponse {
    /// List of project snapshots.
    #[serde(default)]
    pub snapshots: Vec<ProjectSnapshotResponse>,
}

impl ProjectSnapshotsResponse {
    pub fn builder() -> ProjectSnapshotsResponseBuilder {
        <ProjectSnapshotsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectSnapshotsResponseBuilder {
    snapshots: Option<Vec<ProjectSnapshotResponse>>,
}

impl ProjectSnapshotsResponseBuilder {
    pub fn snapshots(mut self, value: Vec<ProjectSnapshotResponse>) -> Self {
        self.snapshots = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectSnapshotsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`snapshots`](ProjectSnapshotsResponseBuilder::snapshots)
    pub fn build(self) -> Result<ProjectSnapshotsResponse, BuildError> {
        Ok(ProjectSnapshotsResponse {
            snapshots: self.snapshots.ok_or_else(|| BuildError::missing_field("snapshots"))?,
        })
    }
}
