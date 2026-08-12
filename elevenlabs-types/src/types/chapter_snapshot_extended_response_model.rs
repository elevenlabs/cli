pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChapterSnapshotExtendedResponseModel {
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
    #[serde(default)]
    pub character_alignments: Vec<CharacterAlignmentModel>,
}

impl ChapterSnapshotExtendedResponseModel {
    pub fn builder() -> ChapterSnapshotExtendedResponseModelBuilder {
        <ChapterSnapshotExtendedResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterSnapshotExtendedResponseModelBuilder {
    chapter_snapshot_id: Option<String>,
    project_id: Option<String>,
    chapter_id: Option<String>,
    created_at_unix: Option<i64>,
    name: Option<String>,
    character_alignments: Option<Vec<CharacterAlignmentModel>>,
}

impl ChapterSnapshotExtendedResponseModelBuilder {
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

    pub fn character_alignments(mut self, value: Vec<CharacterAlignmentModel>) -> Self {
        self.character_alignments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterSnapshotExtendedResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_snapshot_id`](ChapterSnapshotExtendedResponseModelBuilder::chapter_snapshot_id)
    /// - [`project_id`](ChapterSnapshotExtendedResponseModelBuilder::project_id)
    /// - [`chapter_id`](ChapterSnapshotExtendedResponseModelBuilder::chapter_id)
    /// - [`created_at_unix`](ChapterSnapshotExtendedResponseModelBuilder::created_at_unix)
    /// - [`name`](ChapterSnapshotExtendedResponseModelBuilder::name)
    /// - [`character_alignments`](ChapterSnapshotExtendedResponseModelBuilder::character_alignments)
    pub fn build(self) -> Result<ChapterSnapshotExtendedResponseModel, BuildError> {
        Ok(ChapterSnapshotExtendedResponseModel {
            chapter_snapshot_id: self.chapter_snapshot_id.ok_or_else(|| BuildError::missing_field("chapter_snapshot_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            chapter_id: self.chapter_id.ok_or_else(|| BuildError::missing_field("chapter_id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            character_alignments: self.character_alignments.ok_or_else(|| BuildError::missing_field("character_alignments"))?,
        })
    }
}
