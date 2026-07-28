pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryLocatorResponseModel {
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PronunciationDictionaryLocatorResponseModel {
    pub fn builder() -> PronunciationDictionaryLocatorResponseModelBuilder {
        <PronunciationDictionaryLocatorResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryLocatorResponseModelBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl PronunciationDictionaryLocatorResponseModelBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryLocatorResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](PronunciationDictionaryLocatorResponseModelBuilder::pronunciation_dictionary_id)
    pub fn build(self) -> Result<PronunciationDictionaryLocatorResponseModel, BuildError> {
        Ok(PronunciationDictionaryLocatorResponseModel {
            pronunciation_dictionary_id: self.pronunciation_dictionary_id.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self.version_id,
        })
    }
}
