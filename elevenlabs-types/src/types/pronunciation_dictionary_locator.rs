pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Identifies a specific pronunciation dictionary to use
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PronunciationDictionaryLocator {
    /// The unique identifier of the pronunciation dictionary
    #[serde(default)]
    pub pronunciation_dictionary_id: String,
    /// The version identifier of the pronunciation dictionary
    #[serde(default)]
    pub version_id: String,
}

impl PronunciationDictionaryLocator {
    pub fn builder() -> PronunciationDictionaryLocatorBuilder {
        <PronunciationDictionaryLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PronunciationDictionaryLocatorBuilder {
    pronunciation_dictionary_id: Option<String>,
    version_id: Option<String>,
}

impl PronunciationDictionaryLocatorBuilder {
    pub fn pronunciation_dictionary_id(mut self, value: impl Into<String>) -> Self {
        self.pronunciation_dictionary_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PronunciationDictionaryLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_id`](PronunciationDictionaryLocatorBuilder::pronunciation_dictionary_id)
    /// - [`version_id`](PronunciationDictionaryLocatorBuilder::version_id)
    pub fn build(self) -> Result<PronunciationDictionaryLocator, BuildError> {
        Ok(PronunciationDictionaryLocator {
            pronunciation_dictionary_id: self.pronunciation_dictionary_id.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_id"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
