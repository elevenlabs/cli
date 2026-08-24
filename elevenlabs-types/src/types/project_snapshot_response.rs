pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectSnapshotResponse {
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
}

impl ProjectSnapshotResponse {
    pub fn builder() -> ProjectSnapshotResponseBuilder {
        <ProjectSnapshotResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectSnapshotResponseBuilder {
    project_snapshot_id: Option<String>,
    project_id: Option<String>,
    created_at_unix: Option<i64>,
    name: Option<String>,
    audio_upload: Option<HashMap<String, serde_json::Value>>,
    zip_upload: Option<HashMap<String, serde_json::Value>>,
}

impl ProjectSnapshotResponseBuilder {
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

    /// Consumes the builder and constructs a [`ProjectSnapshotResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_snapshot_id`](ProjectSnapshotResponseBuilder::project_snapshot_id)
    /// - [`project_id`](ProjectSnapshotResponseBuilder::project_id)
    /// - [`created_at_unix`](ProjectSnapshotResponseBuilder::created_at_unix)
    /// - [`name`](ProjectSnapshotResponseBuilder::name)
    pub fn build(self) -> Result<ProjectSnapshotResponse, BuildError> {
        Ok(ProjectSnapshotResponse {
            project_snapshot_id: self.project_snapshot_id.ok_or_else(|| BuildError::missing_field("project_snapshot_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            created_at_unix: self.created_at_unix.ok_or_else(|| BuildError::missing_field("created_at_unix"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            audio_upload: self.audio_upload,
            zip_upload: self.zip_upload,
        })
    }
}
