pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteProjectResponseModel {
    /// The status of the studio project deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteProjectResponseModel {
    pub fn builder() -> DeleteProjectResponseModelBuilder {
        <DeleteProjectResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteProjectResponseModelBuilder {
    status: Option<String>,
}

impl DeleteProjectResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteProjectResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteProjectResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteProjectResponseModel, BuildError> {
        Ok(DeleteProjectResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
