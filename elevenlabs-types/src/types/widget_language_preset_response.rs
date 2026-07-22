pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WidgetLanguagePresetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message: Option<String>,
    /// The text contents for the selected language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_contents: Option<WidgetTextContents>,
    /// The text to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_text: Option<String>,
    /// The HTML to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_html: Option<String>,
    /// The key to display for terms and conditions in this language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_key: Option<String>,
}

impl WidgetLanguagePresetResponse {
    pub fn builder() -> WidgetLanguagePresetResponseBuilder {
        <WidgetLanguagePresetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WidgetLanguagePresetResponseBuilder {
    first_message: Option<String>,
    text_contents: Option<WidgetTextContents>,
    terms_text: Option<String>,
    terms_html: Option<String>,
    terms_key: Option<String>,
}

impl WidgetLanguagePresetResponseBuilder {
    pub fn first_message(mut self, value: impl Into<String>) -> Self {
        self.first_message = Some(value.into());
        self
    }

    pub fn text_contents(mut self, value: WidgetTextContents) -> Self {
        self.text_contents = Some(value);
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

    /// Consumes the builder and constructs a [`WidgetLanguagePresetResponse`].
    pub fn build(self) -> Result<WidgetLanguagePresetResponse, BuildError> {
        Ok(WidgetLanguagePresetResponse {
            first_message: self.first_message,
            text_contents: self.text_contents,
            terms_text: self.terms_text,
            terms_html: self.terms_html,
            terms_key: self.terms_key,
        })
    }
}
