pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectMutedTracksResponseModel {
    /// List of chapter IDs that have muted tracks.
    #[serde(default)]
    pub chapter_ids: Vec<String>,
}

impl ProjectMutedTracksResponseModel {
    pub fn builder() -> ProjectMutedTracksResponseModelBuilder {
        <ProjectMutedTracksResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectMutedTracksResponseModelBuilder {
    chapter_ids: Option<Vec<String>>,
}

impl ProjectMutedTracksResponseModelBuilder {
    pub fn chapter_ids(mut self, value: Vec<String>) -> Self {
        self.chapter_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectMutedTracksResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_ids`](ProjectMutedTracksResponseModelBuilder::chapter_ids)
    pub fn build(self) -> Result<ProjectMutedTracksResponseModel, BuildError> {
        Ok(ProjectMutedTracksResponseModel {
            chapter_ids: self.chapter_ids.ok_or_else(|| BuildError::missing_field("chapter_ids"))?,
        })
    }
}
