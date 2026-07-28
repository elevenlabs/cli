pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectCreationMetaResponseModel {
    /// The progress of the project creation.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub creation_progress: f64,
    /// The status of the project creation action.
    pub status: ProjectCreationMetaResponseModelStatus,
    /// The type of the project creation action.
    pub r#type: ProjectCreationMetaType,
}

impl ProjectCreationMetaResponseModel {
    pub fn builder() -> ProjectCreationMetaResponseModelBuilder {
        <ProjectCreationMetaResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectCreationMetaResponseModelBuilder {
    creation_progress: Option<f64>,
    status: Option<ProjectCreationMetaResponseModelStatus>,
    r#type: Option<ProjectCreationMetaType>,
}

impl ProjectCreationMetaResponseModelBuilder {
    pub fn creation_progress(mut self, value: f64) -> Self {
        self.creation_progress = Some(value);
        self
    }

    pub fn status(mut self, value: ProjectCreationMetaResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ProjectCreationMetaType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectCreationMetaResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`creation_progress`](ProjectCreationMetaResponseModelBuilder::creation_progress)
    /// - [`status`](ProjectCreationMetaResponseModelBuilder::status)
    /// - [`r#type`](ProjectCreationMetaResponseModelBuilder::r#type)
    pub fn build(self) -> Result<ProjectCreationMetaResponseModel, BuildError> {
        Ok(ProjectCreationMetaResponseModel {
            creation_progress: self.creation_progress.ok_or_else(|| BuildError::missing_field("creation_progress"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
