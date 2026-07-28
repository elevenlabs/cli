pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Message sent from the server to the client for the multi-context TTS WebSocket.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebsocketTtsServerMessageMulti {
    /// A generated partial audio chunk, encoded using the selected output_format (e.g., MP3 as a base64 string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// If true, indicates that this is the final message for the specified contextId. This is sent when a context is closed. `audio` will be null or empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final: Option<bool>,
    #[serde(rename = "normalizedAlignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<NormalizedAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<Alignment>,
    /// The context identifier to which this message pertains.
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl WebsocketTtsServerMessageMulti {
    pub fn builder() -> WebsocketTtsServerMessageMultiBuilder {
        <WebsocketTtsServerMessageMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebsocketTtsServerMessageMultiBuilder {
    audio: Option<String>,
    is_final: Option<bool>,
    normalized_alignment: Option<NormalizedAlignment>,
    alignment: Option<Alignment>,
    context_id: Option<String>,
}

impl WebsocketTtsServerMessageMultiBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: NormalizedAlignment) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    pub fn alignment(mut self, value: Alignment) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebsocketTtsServerMessageMulti`].
    pub fn build(self) -> Result<WebsocketTtsServerMessageMulti, BuildError> {
        Ok(WebsocketTtsServerMessageMulti {
            audio: self.audio,
            is_final: self.is_final,
            normalized_alignment: self.normalized_alignment,
            alignment: self.alignment,
            context_id: self.context_id,
        })
    }
}
