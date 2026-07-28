pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload to initialize a new context in a multi-stream WebSocket connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeConnectionMulti {
    /// Must be a single space character to initiate the context.
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<RealtimeVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Optional pronunciation dictionaries for this context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    /// Your ElevenLabs API key (if not in header). For this context's first message only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    /// Your authorization bearer token (if not in header). For this context's first message only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    /// A unique identifier for the first context created in the websocket. If not provided, a default context will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl InitializeConnectionMulti {
    pub fn builder() -> InitializeConnectionMultiBuilder {
        <InitializeConnectionMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitializeConnectionMultiBuilder {
    text: Option<String>,
    voice_settings: Option<RealtimeVoiceSettings>,
    generation_config: Option<GenerationConfig>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    context_id: Option<String>,
}

impl InitializeConnectionMultiBuilder {
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

    /// Consumes the builder and constructs a [`InitializeConnectionMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](InitializeConnectionMultiBuilder::text)
    pub fn build(self) -> Result<InitializeConnectionMulti, BuildError> {
        Ok(InitializeConnectionMulti {
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
