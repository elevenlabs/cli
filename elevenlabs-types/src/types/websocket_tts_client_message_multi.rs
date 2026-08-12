pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Message sent from the client to the multi-context TTS WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebsocketTtsClientMessageMulti {
    /// Text to be synthesized.
    /// For the first message establishing a new context (identified by `context_id`, or a default context if `context_id` is absent), this should be a single space character (' ').
    /// For subsequent messages to an active context, this is the text to synthesize.
    /// This field can be null or an empty string if the message is primarily for control (e.g., using `flush`, `close_context`, or `close_socket`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Voice settings. Can only be provided in the first message for a given context_id (or first message overall if context_id is not used/default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<RealtimeVoiceSettings>,
    /// Generation config. Can only be provided in the first message for a given context_id (or first message overall if context_id is not used/default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
    /// Your ElevenLabs API key. Can only be provided in the first message for a given context_id if not present in the header.
    #[serde(rename = "xi-api-key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    /// Your authorization bearer token. Can only be provided in the first message for a given context_id if not present in the header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    /// If true, flushes the audio buffer and returns the remaining audio for the specified `context_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
    /// Optional list of pronunciation dictionary locators. Can only be provided in the first message for a given context_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    /// An identifier for the text-to-speech context. Allows managing multiple independent audio generation streams over a single WebSocket connection. If omitted, a default context is used.
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// If true, closes the specified `contextId`. No further audio will be generated for this context. The `text` field is ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_context: Option<bool>,
    /// If true, flushes all contexts and closes the entire WebSocket connection. The `text` and `contextId` fields are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_socket: Option<bool>,
}

impl WebsocketTtsClientMessageMulti {
    pub fn builder() -> WebsocketTtsClientMessageMultiBuilder {
        <WebsocketTtsClientMessageMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebsocketTtsClientMessageMultiBuilder {
    text: Option<String>,
    voice_settings: Option<RealtimeVoiceSettings>,
    generation_config: Option<GenerationConfig>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    flush: Option<bool>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
    context_id: Option<String>,
    close_context: Option<bool>,
    close_socket: Option<bool>,
}

impl WebsocketTtsClientMessageMultiBuilder {
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

    pub fn xi_api_key(mut self, value: impl Into<String>) -> Self {
        self.xi_api_key = Some(value.into());
        self
    }

    pub fn authorization(mut self, value: impl Into<String>) -> Self {
        self.authorization = Some(value.into());
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn close_context(mut self, value: bool) -> Self {
        self.close_context = Some(value);
        self
    }

    pub fn close_socket(mut self, value: bool) -> Self {
        self.close_socket = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebsocketTtsClientMessageMulti`].
    pub fn build(self) -> Result<WebsocketTtsClientMessageMulti, BuildError> {
        Ok(WebsocketTtsClientMessageMulti {
            text: self.text,
            voice_settings: self.voice_settings,
            generation_config: self.generation_config,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
            flush: self.flush,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
            context_id: self.context_id,
            close_context: self.close_context,
            close_socket: self.close_socket,
        })
    }
}
