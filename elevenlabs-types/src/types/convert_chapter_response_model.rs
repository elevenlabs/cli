pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvertChapterResponseModel {
    /// The status of the studio chapter conversion request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl ConvertChapterResponseModel {
    pub fn builder() -> ConvertChapterResponseModelBuilder {
        <ConvertChapterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertChapterResponseModelBuilder {
    status: Option<String>,
}

impl ConvertChapterResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvertChapterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ConvertChapterResponseModelBuilder::status)
    pub fn build(self) -> Result<ConvertChapterResponseModel, BuildError> {
        Ok(ConvertChapterResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
