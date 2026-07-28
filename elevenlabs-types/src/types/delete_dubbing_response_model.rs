pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteDubbingResponseModel {
    /// The status of the dubbing project. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteDubbingResponseModel {
    pub fn builder() -> DeleteDubbingResponseModelBuilder {
        <DeleteDubbingResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteDubbingResponseModelBuilder {
    status: Option<String>,
}

impl DeleteDubbingResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteDubbingResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteDubbingResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteDubbingResponseModel, BuildError> {
        Ok(DeleteDubbingResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
