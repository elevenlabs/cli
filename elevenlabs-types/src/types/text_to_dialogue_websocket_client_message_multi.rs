pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// All fields are optional unless noted for a context's **first** message.
/// 
/// **Every message**
/// - `context_id` is required, except on a message containing only `close_socket`.
/// 
/// **First message for a context**
/// - `voices`: non-empty array of voice IDs (maximum 10 per context for `eleven_v3`; exactly 1 for `eleven_v3_conversational`).
/// - Credentials if not supplied via `xi-api-key` / `Authorization` headers or `single_use_token` query parameter (accepted on the first message of the connection only).
/// 
/// **Subsequent messages for a context**
/// - Do not resend `voices`, `voice_settings`, `pronunciation_dictionary_locators`, or credential fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueWebsocketClientMessageMulti {
    /// Identifier for an independent dialogue stream within the socket. The first message with a new `context_id` creates that context. Required on every message except one containing only `close_socket`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    /// Dialogue lines to append to this context for synthesis. Each `voice_id` must be registered for this context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<TextToDialogueWebsocketVoiceInput>>,
    /// Force generation of this context's buffered text without closing it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush: Option<bool>,
    /// Flush this context's remaining audio, emit its `is_final` message, and close it. Other contexts stay open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_context: Option<bool>,
    /// Flush all contexts, emit their remaining audio and `is_final` messages, and close the WebSocket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_socket: Option<bool>,
    /// Resets this context's 20s inactivity timer; performs no synthesis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<bool>,
    /// API key for the first message of the connection if not provided via the `xi-api-key` header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xi_api_key: Option<String>,
    /// Bearer token for the first message of the connection if not provided via the `Authorization` header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    /// Single-use token for the first message of the connection if not provided via the `single_use_token` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use_token: Option<String>,
    /// Voice IDs to load for this context (first message for the context only, required on that message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voices: Option<Vec<String>>,
    /// Optional voice settings for this context (first message for the context only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<TextToDialogueWebsocketVoiceSettings>,
    /// Optional pronunciation dictionaries for this context (first message for the context only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
}

impl TextToDialogueWebsocketClientMessageMulti {
    pub fn builder() -> TextToDialogueWebsocketClientMessageMultiBuilder {
        <TextToDialogueWebsocketClientMessageMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketClientMessageMultiBuilder {
    context_id: Option<String>,
    inputs: Option<Vec<TextToDialogueWebsocketVoiceInput>>,
    flush: Option<bool>,
    close_context: Option<bool>,
    close_socket: Option<bool>,
    keep_alive: Option<bool>,
    xi_api_key: Option<String>,
    authorization: Option<String>,
    single_use_token: Option<String>,
    voices: Option<Vec<String>>,
    voice_settings: Option<TextToDialogueWebsocketVoiceSettings>,
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,
}

impl TextToDialogueWebsocketClientMessageMultiBuilder {
    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    pub fn inputs(mut self, value: Vec<TextToDialogueWebsocketVoiceInput>) -> Self {
        self.inputs = Some(value);
        self
    }

    pub fn flush(mut self, value: bool) -> Self {
        self.flush = Some(value);
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

    pub fn keep_alive(mut self, value: bool) -> Self {
        self.keep_alive = Some(value);
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

    pub fn single_use_token(mut self, value: impl Into<String>) -> Self {
        self.single_use_token = Some(value.into());
        self
    }

    pub fn voices(mut self, value: Vec<String>) -> Self {
        self.voices = Some(value);
        self
    }

    pub fn voice_settings(mut self, value: TextToDialogueWebsocketVoiceSettings) -> Self {
        self.voice_settings = Some(value);
        self
    }

    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketClientMessageMulti`].
    pub fn build(self) -> Result<TextToDialogueWebsocketClientMessageMulti, BuildError> {
        Ok(TextToDialogueWebsocketClientMessageMulti {
            context_id: self.context_id,
            inputs: self.inputs,
            flush: self.flush,
            close_context: self.close_context,
            close_socket: self.close_socket,
            keep_alive: self.keep_alive,
            xi_api_key: self.xi_api_key,
            authorization: self.authorization,
            single_use_token: self.single_use_token,
            voices: self.voices,
            voice_settings: self.voice_settings,
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators,
        })
    }
}
