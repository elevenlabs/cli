pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A chunk of audio to be translated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateInputAudioChunkPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Base64-encoded audio data. Recommended chunk size is around 100 ms of audio.
    #[serde(rename = "audio_base_64")]
    #[serde(default)]
    pub audio_base64: String,
}

impl TranslateInputAudioChunkPayload {
    pub fn builder() -> TranslateInputAudioChunkPayloadBuilder {
        <TranslateInputAudioChunkPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateInputAudioChunkPayloadBuilder {
    message_type: Option<String>,
    audio_base64: Option<String>,
}

impl TranslateInputAudioChunkPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranslateInputAudioChunkPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateInputAudioChunkPayloadBuilder::message_type)
    /// - [`audio_base64`](TranslateInputAudioChunkPayloadBuilder::audio_base64)
    pub fn build(self) -> Result<TranslateInputAudioChunkPayload, BuildError> {
        Ok(TranslateInputAudioChunkPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
        })
    }
}
