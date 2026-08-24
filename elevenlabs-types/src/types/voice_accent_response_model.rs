pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoiceAccentResponseModel {
    /// The accent value used for filtering shared voices via the `accent` query parameter on `GET /v1/shared-voices`.
    #[serde(default)]
    pub accent: String,
    /// The language code this accent belongs to, e.g. `en`.
    #[serde(default)]
    pub language: String,
    /// The full accent code, e.g. `en-american`.
    #[serde(default)]
    pub code: String,
    /// The human-readable accent name, e.g. `American`.
    #[serde(default)]
    pub name: String,
}

impl VoiceAccentResponseModel {
    pub fn builder() -> VoiceAccentResponseModelBuilder {
        <VoiceAccentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAccentResponseModelBuilder {
    accent: Option<String>,
    language: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl VoiceAccentResponseModelBuilder {
    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceAccentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accent`](VoiceAccentResponseModelBuilder::accent)
    /// - [`language`](VoiceAccentResponseModelBuilder::language)
    /// - [`code`](VoiceAccentResponseModelBuilder::code)
    /// - [`name`](VoiceAccentResponseModelBuilder::name)
    pub fn build(self) -> Result<VoiceAccentResponseModel, BuildError> {
        Ok(VoiceAccentResponseModel {
            accent: self.accent.ok_or_else(|| BuildError::missing_field("accent"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
