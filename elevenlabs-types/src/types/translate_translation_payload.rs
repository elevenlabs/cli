pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Translated text in the target language.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateTranslationPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Translated text.
    #[serde(default)]
    pub text: String,
}

impl TranslateTranslationPayload {
    pub fn builder() -> TranslateTranslationPayloadBuilder {
        <TranslateTranslationPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateTranslationPayloadBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl TranslateTranslationPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateTranslationPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateTranslationPayloadBuilder::message_type)
    /// - [`text`](TranslateTranslationPayloadBuilder::text)
    pub fn build(self) -> Result<TranslateTranslationPayload, BuildError> {
        Ok(TranslateTranslationPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
