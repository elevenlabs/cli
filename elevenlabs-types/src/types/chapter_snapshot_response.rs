pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChapterSnapshotResponse {
    /// The ID of the chapter snapshot.
    #[serde(default)]
    pub chapter_snapshot_id: String,
    /// The ID of the project.
    #[serde(default)]
    pub project_id: String,
    /// The ID of the chapter.
    #[serde(default)]
    pub chapter_id: String,
    /// The creation date of the chapter snapshot.
    #[serde(default)]
    pub created_at_unix: i64,
    /// The name of the chapter snapshot.
    #[serde(default)]
    pub name: String,
}

impl ChapterSnapshotResponse {
    pub fn builder() -> ChapterSnapshotResponseBuilder {
        <ChapterSnapshotResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterSnapshotResponseBuilder {
    chapter_snapshot_id: Option<String>,
    project_id: Option<String>,
    chapter_id: Option<String>,
    created_at_unix: Option<i64>,
    name: Option<String>,
}

impl ChapterSnapshotResponseBuilder {
    pub fn chapter_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_snapshot_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChapterSnapshotResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_snapshot_id`](ChapterSnapshotResponseBuilder::chapter_snapshot_id)
    /// - [`project_id`](ChapterSnapshotResponseBuilder::project_id)
    /// - [`chapter_id`](ChapterSnapshotResponseBuilder::chapter_id)
    /// - [`created_at_unix`](ChapterSnapshotResponseBuilder::created_at_unix)
    /// - [`name`](ChapterSnapshotResponseBuilder::name)
    pub fn build(self) -> Result<ChapterSnapshotResponse, BuildError> {
        Ok(ChapterSnapshotResponse {
            chapter_snapshot_id: self.chapter_snapshot_id.ok_or_else(|| BuildError::missing_field("chapter_snapshot_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            chapter_id: self.chapter_id.ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
