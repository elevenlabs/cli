pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeConnection {
    /// The initial text that must be sent is a blank space.
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<RealtimeVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Optional list of pronunciation dictionary locators. If provided, these dictionaries will be used to
    /// modify pronunciation of matching text. Must only be provided in the first message.
    ///
    /// Note: Pronunciation dictionary matches will only be respected within a provided chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    /// Your ElevenLabs API key. This can only be included in the first message and is not needed if present in the header.
    #[serde(rename = "xi-api-key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    /// Your authorization bearer token. This can only be included in the first message and is not needed if present in the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
}

impl InitializeConnection {
    pub fn builder() -> InitializeConnectionBuilder {
        <InitializeConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionBuilder {
    text: Option<String>,
    voice_settings: Option<RealtimeVoiceSettings>,
    generation_config: Option<GenerationConfig>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
}

impl InitializeConnectionBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_settings(mut self, value: RealtimeVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn generation_config(mut self, value: GenerationConfig) -> Self {
        self.generation_config = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn xi_api_key(mut self, value: impl Into<String>) -> Self {
        self.xi_api_key = Some(value.into());
        self
    }

    pub fn authorization(mut self, value: impl Into<String>) -> Self {
        self.authorization = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitializeConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](InitializeConnectionBuilder::text)
    pub fn build(self) -> Result<InitializeConnection, BuildError> {
        Ok(InitializeConnection {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
        })
    }
}
