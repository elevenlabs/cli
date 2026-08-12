pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PodcastProjectResponseModel {
    /// The project associated with the created podcast.
    pub project: ProjectResponse,
}

impl PodcastProjectResponseModel {
    pub fn builder() -> PodcastProjectResponseModelBuilder {
        <PodcastProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PodcastProjectResponseModelBuilder {
    project: Option<ProjectResponse>,
}

impl PodcastProjectResponseModelBuilder {
    pub fn project(mut self, value: ProjectResponse) -> Self {
        self.project = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PodcastProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project`](PodcastProjectResponseModelBuilder::project)
    pub fn build(self) -> Result<PodcastProjectResponseModel, BuildError> {
        Ok(PodcastProjectResponseModel {
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
        })
    }
}
