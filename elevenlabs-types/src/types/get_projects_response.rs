pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetProjectsResponse {
    /// A list of projects with their metadata.
    #[serde(default)]
    pub projects: Vec<ProjectResponse>,
}

impl GetProjectsResponse {
    pub fn builder() -> GetProjectsResponseBuilder {
        <GetProjectsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProjectsResponseBuilder {
    projects: Option<Vec<ProjectResponse>>,
}

impl GetProjectsResponseBuilder {
    pub fn projects(mut self, value: Vec<ProjectResponse>) -> Self {
        self.projects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProjectsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`projects`](GetProjectsResponseBuilder::projects)
    pub fn build(self) -> Result<GetProjectsResponse, BuildError> {
        Ok(GetProjectsResponse {
            projects: self.projects.ok_or_else(|| BuildError::missing_field("projects"))?,
        })
    }
}
