pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConvAiFileUploadResponseModel {
    #[serde(default)]
    pub file_id: String,
}

impl ConvAiFileUploadResponseModel {
    pub fn builder() -> ConvAiFileUploadResponseModelBuilder {
        <ConvAiFileUploadResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvAiFileUploadResponseModelBuilder {
    file_id: Option<String>,
}

impl ConvAiFileUploadResponseModelBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConvAiFileUploadResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](ConvAiFileUploadResponseModelBuilder::file_id)
    pub fn build(self) -> Result<ConvAiFileUploadResponseModel, BuildError> {
        Ok(ConvAiFileUploadResponseModel {
            file_id: self.file_id.ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
