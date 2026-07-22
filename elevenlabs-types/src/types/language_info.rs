pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageInfo {
    /// The language code (e.g. 'en', 'fr', 'es-ES').
    #[serde(default)]
    pub code: String,
    /// The human-readable language name (e.g. 'English', 'French', 'Spanish (Spain)').
    #[serde(default)]
    pub label: String,
}

impl LanguageInfo {
    pub fn builder() -> LanguageInfoBuilder {
        <LanguageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageInfoBuilder {
    code: Option<String>,
    label: Option<String>,
}

impl LanguageInfoBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](LanguageInfoBuilder::code)
    /// - [`label`](LanguageInfoBuilder::label)
    pub fn build(self) -> Result<LanguageInfo, BuildError> {
        Ok(LanguageInfo {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
