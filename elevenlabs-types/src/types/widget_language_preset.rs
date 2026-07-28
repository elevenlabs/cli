pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WidgetLanguagePreset {
    /// The text contents for the selected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_contents: Option<WidgetTextContents>,
    /// The translation cache for the text contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_contents_translation: Option<WidgetTextContentsTranslation>,
    /// The text to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_text: Option<String>,
    /// The HTML to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_html: Option<String>,
    /// The key to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_key: Option<String>,
    /// The translation cache for the terms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_translation: Option<WidgetTermsTranslation>,
}

impl WidgetLanguagePreset {
    pub fn builder() -> WidgetLanguagePresetBuilder {
        <WidgetLanguagePresetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetLanguagePresetBuilder {
    text_contents: Option<WidgetTextContents>,
    text_contents_translation: Option<WidgetTextContentsTranslation>,
    terms_text: Option<String>,
    terms_html: Option<String>,
    terms_key: Option<String>,
    terms_translation: Option<WidgetTermsTranslation>,
}

impl WidgetLanguagePresetBuilder {
    pub fn text_contents(mut self, value: WidgetTextContents) -> Self {
        self.text_contents = Some(value);
        self
    }

    pub fn text_contents_translation(mut self, value: WidgetTextContentsTranslation) -> Self {
        self.text_contents_translation = Some(value);
        self
    }

    pub fn terms_text(mut self, value: impl Into<String>) -> Self {
        self.terms_text = Some(value.into());
        self
    }

    pub fn terms_html(mut self, value: impl Into<String>) -> Self {
        self.terms_html = Some(value.into());
        self
    }

    pub fn terms_key(mut self, value: impl Into<String>) -> Self {
        self.terms_key = Some(value.into());
        self
    }

    pub fn terms_translation(mut self, value: WidgetTermsTranslation) -> Self {
        self.terms_translation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WidgetLanguagePreset`].
    pub fn build(self) -> Result<WidgetLanguagePreset, BuildError> {
        Ok(WidgetLanguagePreset {
            text_contents: self.text_contents,
            text_contents_translation: self.text_contents_translation,
            terms_text: self.terms_text,
            terms_html: self.terms_html,
            terms_key: self.terms_key,
            terms_translation: self.terms_translation,
        })
    }
}
