pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to initialize or re-initialize a TTS context with specific settings and initial text for multi-stream connections.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InitialiseContext {
    /// The initial text to synthesize. Should end with a single space.
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<RealtimeVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Optional list of pronunciation dictionary locators to be used for this context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    /// Your ElevenLabs API key. Required if not provided in the WebSocket connection's header or query parameters. This applies to the (re)initialization of this specific context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    /// Your authorization bearer token. Required if not provided in the WebSocket connection's header or query parameters. This applies to the (re)initialization of this specific context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    /// An identifier for the text-to-speech context. If omitted, a default context ID may be assigned by the server. If provided, this message will create a new context with this ID or re-initialize an existing one with the new settings and text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl InitialiseContext {
    pub fn builder() -> InitialiseContextBuilder {
        <InitialiseContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialiseContextBuilder {
    text: Option<String>,
    voice_settings: Option<RealtimeVoiceSettings>,
    generation_config: Option<GenerationConfig>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    context_id: Option<String>,
}

impl InitialiseContextBuilder {
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

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitialiseContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](InitialiseContextBuilder::text)
    pub fn build(self) -> Result<InitialiseContext, BuildError> {
        Ok(InitialiseContext {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
            context_id: self.context_id,
        })
    }
}
