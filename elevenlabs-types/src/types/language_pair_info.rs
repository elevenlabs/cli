pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguagePairInfo {
    /// The source language.
    #[serde(default)]
    pub source_language: LanguageInfo,
    /// The available destination languages for this source language.
    #[serde(default)]
    pub destination_languages: Vec<LanguageInfo>,
}

impl LanguagePairInfo {
    pub fn builder() -> LanguagePairInfoBuilder {
        <LanguagePairInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguagePairInfoBuilder {
    source_language: Option<LanguageInfo>,
    destination_languages: Option<Vec<LanguageInfo>>,
}

impl LanguagePairInfoBuilder {
    pub fn source_language(mut self, value: LanguageInfo) -> Self {
        self.source_language = Some(value);
        self
    }

    pub fn destination_languages(mut self, value: Vec<LanguageInfo>) -> Self {
        self.destination_languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LanguagePairInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_language`](LanguagePairInfoBuilder::source_language)
    /// - [`destination_languages`](LanguagePairInfoBuilder::destination_languages)
    pub fn build(self) -> Result<LanguagePairInfo, BuildError> {
        Ok(LanguagePairInfo {
            source_language: self.source_language.ok_or_else(|| BuildError::missing_field("source_language"))?,
            destination_languages: self.destination_languages.ok_or_else(|| BuildError::missing_field("destination_languages"))?,
        })
    }
}
