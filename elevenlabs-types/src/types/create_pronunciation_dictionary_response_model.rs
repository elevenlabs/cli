pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePronunciationDictionaryResponseModel {
    /// The status of the create pronunciation dictionary request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl CreatePronunciationDictionaryResponseModel {
    pub fn builder() -> CreatePronunciationDictionaryResponseModelBuilder {
        <CreatePronunciationDictionaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePronunciationDictionaryResponseModelBuilder {
    status: Option<String>,
}

impl CreatePronunciationDictionaryResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePronunciationDictionaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CreatePronunciationDictionaryResponseModelBuilder::status)
    pub fn build(self) -> Result<CreatePronunciationDictionaryResponseModel, BuildError> {
        Ok(CreatePronunciationDictionaryResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
