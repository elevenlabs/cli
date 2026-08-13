pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LanguagePresetInput {
    /// The overrides for the language preset
    #[serde(default)]
    pub overrides: ConversationConfigClientOverrideInput,
    /// The translation of the first message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_message_translation: Option<LanguagePresetTranslation>,
    /// The translation of the soft timeout message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_timeout_translation: Option<LanguagePresetTranslation>,
}

impl LanguagePresetInput {
    pub fn builder() -> LanguagePresetInputBuilder {
        <LanguagePresetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LanguagePresetInputBuilder {
    overrides: Option<ConversationConfigClientOverrideInput>,
    first_message_translation: Option<LanguagePresetTranslation>,
    soft_timeout_translation: Option<LanguagePresetTranslation>,
}

impl LanguagePresetInputBuilder {
    pub fn overrides(mut self, value: ConversationConfigClientOverrideInput) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn first_message_translation(mut self, value: LanguagePresetTranslation) -> Self {
        self.first_message_translation = Some(value);
        self
    }

    pub fn soft_timeout_translation(mut self, value: LanguagePresetTranslation) -> Self {
        self.soft_timeout_translation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LanguagePresetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`overrides`](LanguagePresetInputBuilder::overrides)
    pub fn build(self) -> Result<LanguagePresetInput, BuildError> {
        Ok(LanguagePresetInput {
            overrides: self.overrides.ok_or_else(|| BuildError::missing_field("overrides"))?,
            first_message_translation: self.first_message_translation,
            soft_timeout_translation: self.soft_timeout_translation,
        })
    }
}
