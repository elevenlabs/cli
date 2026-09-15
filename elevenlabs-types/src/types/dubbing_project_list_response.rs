pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingProjectListResponse {
    /// The page of dubbing projects the caller can access.
    #[serde(default)]
    pub projects: Vec<DubbingProjectResponse>,
    /// Opaque cursor to pass back as `cursor` for the next page, or null when there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl DubbingProjectListResponse {
    pub fn builder() -> DubbingProjectListResponseBuilder {
        <DubbingProjectListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingProjectListResponseBuilder {
    projects: Option<Vec<DubbingProjectResponse>>,
    next_cursor: Option<String>,
}

impl DubbingProjectListResponseBuilder {
    pub fn projects(mut self, value: Vec<DubbingProjectResponse>) -> Self {
        self.projects = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingProjectListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`projects`](DubbingProjectListResponseBuilder::projects)
    pub fn build(self) -> Result<DubbingProjectListResponse, BuildError> {
        Ok(DubbingProjectListResponse {
            projects: self.projects.ok_or_else(|| BuildError::missing_field("projects"))?,
            next_cursor: self.next_cursor,
        })
    }
}
