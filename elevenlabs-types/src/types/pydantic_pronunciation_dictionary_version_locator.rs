pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A locator for other documents to be able to reference a specific dictionary and it's version.
/// This is a pydantic version of PronunciationDictionaryVersionLocatorDBModel.
/// Required to ensure compat with the rest of the agent data models.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PydanticPronunciationDictionaryVersionLocator {
    /// The ID of the pronunciation dictionary
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    /// The ID of the version of the pronunciation dictionary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PydanticPronunciationDictionaryVersionLocator {
    pub fn builder() -> PydanticPronunciationDictionaryVersionLocatorBuilder {
        <PydanticPronunciationDictionaryVersionLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PydanticPronunciationDictionaryVersionLocatorBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl PydanticPronunciationDictionaryVersionLocatorBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PydanticPronunciationDictionaryVersionLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](PydanticPronunciationDictionaryVersionLocatorBuilder::pronunciation_dictionary_id)
    pub fn build(self) -> Result<PydanticPronunciationDictionaryVersionLocator, BuildError> {
        Ok(PydanticPronunciationDictionaryVersionLocator {
            pronunciation_dictionary_id: self.pronunciation_dictionary_id.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self.version_id,
        })
    }
}
