pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditProjectResponseModel {
    pub project: ProjectResponse,
}

impl EditProjectResponseModel {
    pub fn builder() -> EditProjectResponseModelBuilder {
        <EditProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditProjectResponseModelBuilder {
    project: Option<ProjectResponse>,
}

impl EditProjectResponseModelBuilder {
    pub fn project(mut self, value: ProjectResponse) -> Self {
        self.project = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project`](EditProjectResponseModelBuilder::project)
    pub fn build(self) -> Result<EditProjectResponseModel, BuildError> {
        Ok(EditProjectResponseModel {
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
        })
    }
}
