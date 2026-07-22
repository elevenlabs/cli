pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteChapterResponseModel {
    /// The status of the studio chapter deletion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl DeleteChapterResponseModel {
    pub fn builder() -> DeleteChapterResponseModelBuilder {
        <DeleteChapterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteChapterResponseModelBuilder {
    status: Option<String>,
}

impl DeleteChapterResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteChapterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DeleteChapterResponseModelBuilder::status)
    pub fn build(self) -> Result<DeleteChapterResponseModel, BuildError> {
        Ok(DeleteChapterResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
