pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerifiedVoiceLanguageResponseModel {
    /// The language of the voice.
    #[serde(default)]
    pub language: String,
    /// The voice's model ID.
    #[serde(default)]
    pub model_id: String,
    /// The voice's accent, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// The voice's locale, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// The voice's preview URL, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

impl VerifiedVoiceLanguageResponseModel {
    pub fn builder() -> VerifiedVoiceLanguageResponseModelBuilder {
        <VerifiedVoiceLanguageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifiedVoiceLanguageResponseModelBuilder {
    language: Option<String>,
    model_id: Option<String>,
    accent: Option<String>,
    locale: Option<String>,
    preview_url: Option<String>,
}

impl VerifiedVoiceLanguageResponseModelBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn preview_url(mut self, value: impl Into<String>) -> Self {
        self.preview_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerifiedVoiceLanguageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](VerifiedVoiceLanguageResponseModelBuilder::language)
    /// - [`model_id`](VerifiedVoiceLanguageResponseModelBuilder::model_id)
    pub fn build(self) -> Result<VerifiedVoiceLanguageResponseModel, BuildError> {
        Ok(VerifiedVoiceLanguageResponseModel {
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            model_id: self.model_id.ok_or_else(|| BuildError::missing_field("model_id"))?,
            accent: self.accent,
            locale: self.locale,
            preview_url: self.preview_url,
        })
    }
}
