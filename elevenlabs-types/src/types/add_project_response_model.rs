pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddProjectResponseModel {
    pub project: ProjectResponse,
}

impl AddProjectResponseModel {
    pub fn builder() -> AddProjectResponseModelBuilder {
        <AddProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddProjectResponseModelBuilder {
    project: Option<ProjectResponse>,
}

impl AddProjectResponseModelBuilder {
    pub fn project(mut self, value: ProjectResponse) -> Self {
        self.project = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project`](AddProjectResponseModelBuilder::project)
    pub fn build(self) -> Result<AddProjectResponseModel, BuildError> {
        Ok(AddProjectResponseModel {
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
        })
    }
}
