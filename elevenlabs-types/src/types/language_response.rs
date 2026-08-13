pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LanguageResponse {
    /// The unique identifier of the language.
    #[serde(default)]
    pub language_id: String,
    /// The name of the language.
    #[serde(default)]
    pub name: String,
}

impl LanguageResponse {
    pub fn builder() -> LanguageResponseBuilder {
        <LanguageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguageResponseBuilder {
    language_id: Option<String>,
    name: Option<String>,
}

impl LanguageResponseBuilder {
    pub fn language_id(mut self, value: impl Into<String>) -> Self {
        self.language_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LanguageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_id`](LanguageResponseBuilder::language_id)
    /// - [`name`](LanguageResponseBuilder::name)
    pub fn build(self) -> Result<LanguageResponse, BuildError> {
        Ok(LanguageResponse {
            language_id: self.language_id.ok_or_else(|| BuildError::missing_field("language_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
