pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectSnapshotExtendedResponseModel {
    /// The ID of the project snapshot.
    #[serde(default)]
    pub project_snapshot_id: String,
    /// The ID of the project.
    #[serde(default)]
    pub project_id: String,
    /// The creation date of the project snapshot.
    #[serde(default)]
    pub created_at_unix: i64,
    /// The name of the project snapshot.
    #[serde(default)]
    pub name: String,
    /// (Deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_upload: Option<HashMap<String, serde_json::Value>>,
    /// (Deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_upload: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub character_alignments: Vec<CharacterAlignmentModel>,
    /// The total duration of the audio in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub audio_duration_secs: f64,
}

impl ProjectSnapshotExtendedResponseModel {
    pub fn builder() -> ProjectSnapshotExtendedResponseModelBuilder {
        <ProjectSnapshotExtendedResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectSnapshotExtendedResponseModelBuilder {
    project_snapshot_id: Option<String>,
    project_id: Option<String>,
    created_at_unix: Option<i64>,
    name: Option<String>,
    audio_upload: Option<HashMap<String, serde_json::Value>>,
    zip_upload: Option<HashMap<String, serde_json::Value>>,
    character_alignments: Option<Vec<CharacterAlignmentModel>>,
    audio_duration_secs: Option<f64>,
}

impl ProjectSnapshotExtendedResponseModelBuilder {
    pub fn project_snapshot_id(mut self, value: impl Into<String>) -> Self {
        self.project_snapshot_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
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

    pub fn audio_upload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.audio_upload = Some(value);
        self
    }

    pub fn zip_upload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.zip_upload = Some(value);
        self
    }

    pub fn character_alignments(mut self, value: Vec<CharacterAlignmentModel>) -> Self {
        self.character_alignments = Some(value);
        self
    }

    pub fn audio_duration_secs(mut self, value: f64) -> Self {
        self.audio_duration_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectSnapshotExtendedResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_snapshot_id`](ProjectSnapshotExtendedResponseModelBuilder::project_snapshot_id)
    /// - [`project_id`](ProjectSnapshotExtendedResponseModelBuilder::project_id)
    /// - [`created_at_unix`](ProjectSnapshotExtendedResponseModelBuilder::created_at_unix)
    /// - [`name`](ProjectSnapshotExtendedResponseModelBuilder::name)
    /// - [`character_alignments`](ProjectSnapshotExtendedResponseModelBuilder::character_alignments)
    /// - [`audio_duration_secs`](ProjectSnapshotExtendedResponseModelBuilder::audio_duration_secs)
    pub fn build(self) -> Result<ProjectSnapshotExtendedResponseModel, BuildError> {
        Ok(ProjectSnapshotExtendedResponseModel {
            project_snapshot_id: self.project_snapshot_id.ok_or_else(|| BuildError::missing_field("project_snapshot_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            audio_upload: self.audio_upload,
            zip_upload: self.zip_upload,
            character_alignments: self.character_alignments.ok_or_else(|| BuildError::missing_field("character_alignments"))?,
            audio_duration_secs: self.audio_duration_secs.ok_or_else(|| BuildError::missing_field("audio_duration_secs"))?,
        })
    }
}
