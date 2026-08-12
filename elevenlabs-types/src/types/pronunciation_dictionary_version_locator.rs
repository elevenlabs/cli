pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryVersionLocator {
    /// The ID of the pronunciation dictionary.
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    /// The ID of the version of the pronunciation dictionary. If not provided, the latest version will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PronunciationDictionaryVersionLocator {
    pub fn builder() -> PronunciationDictionaryVersionLocatorBuilder {
        <PronunciationDictionaryVersionLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryVersionLocatorBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl PronunciationDictionaryVersionLocatorBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryVersionLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](PronunciationDictionaryVersionLocatorBuilder::pronunciation_dictionary_id)
    pub fn build(self) -> Result<PronunciationDictionaryVersionLocator, BuildError> {
        Ok(PronunciationDictionaryVersionLocator {
            pronunciation_dictionary_id: self.pronunciation_dictionary_id.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self.version_id,
        })
    }
}
