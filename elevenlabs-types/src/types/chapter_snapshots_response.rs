pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChapterSnapshotsResponse {
    /// List of chapter snapshots.
    #[serde(default)]
    pub snapshots: Vec<ChapterSnapshotResponse>,
}

impl ChapterSnapshotsResponse {
    pub fn builder() -> ChapterSnapshotsResponseBuilder {
        <ChapterSnapshotsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterSnapshotsResponseBuilder {
    snapshots: Option<Vec<ChapterSnapshotResponse>>,
}

impl ChapterSnapshotsResponseBuilder {
    pub fn snapshots(mut self, value: Vec<ChapterSnapshotResponse>) -> Self {
        self.snapshots = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterSnapshotsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`snapshots`](ChapterSnapshotsResponseBuilder::snapshots)
    pub fn build(self) -> Result<ChapterSnapshotsResponse, BuildError> {
        Ok(ChapterSnapshotsResponse {
            snapshots: self.snapshots.ok_or_else(|| BuildError::missing_field("snapshots"))?,
        })
    }
}
