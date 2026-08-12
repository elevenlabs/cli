pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguagePresetTranslation {
    #[serde(default)]
    pub source_hash: String,
    #[serde(default)]
    pub text: String,
}

impl LanguagePresetTranslation {
    pub fn builder() -> LanguagePresetTranslationBuilder {
        <LanguagePresetTranslationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguagePresetTranslationBuilder {
    source_hash: Option<String>,
    text: Option<String>,
}

impl LanguagePresetTranslationBuilder {
    pub fn source_hash(mut self, value: impl Into<String>) -> Self {
        self.source_hash = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguagePresetTranslation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source_hash`](LanguagePresetTranslationBuilder::source_hash)
    /// - [`text`](LanguagePresetTranslationBuilder::text)
    pub fn build(self) -> Result<LanguagePresetTranslation, BuildError> {
        Ok(LanguagePresetTranslation {
            source_hash: self.source_hash.ok_or_else(|| BuildError::missing_field("source_hash"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
