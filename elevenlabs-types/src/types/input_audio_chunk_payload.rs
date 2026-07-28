pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Payload for sending audio chunks from client to server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InputAudioChunkPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Base64-encoded audio data.
    #[serde(rename = "audio_base_64")]
    #[serde(default)]
    pub audio_base64: String,
    /// Whether to commit the transcription after this chunk.
    #[serde(default)]
    pub commit: bool,
    /// Sample rate of the audio in Hz.
    #[serde(default)]
    pub sample_rate: i64,
    /// Send text context to the model. Can only be sent alongside the first audio chunk. If sent in a subsequent chunk, an error will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,
}

impl InputAudioChunkPayload {
    pub fn builder() -> InputAudioChunkPayloadBuilder {
        <InputAudioChunkPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InputAudioChunkPayloadBuilder {
    message_type: Option<String>,
    audio_base64: Option<String>,
    commit: Option<bool>,
    sample_rate: Option<i64>,
    previous_text: Option<String>,
}

impl InputAudioChunkPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
        self
    }

    pub fn commit(mut self, value: bool) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    pub fn previous_text(mut self, value: impl Into<String>) -> Self {
        self.previous_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InputAudioChunkPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](InputAudioChunkPayloadBuilder::message_type)
    /// - [`audio_base64`](InputAudioChunkPayloadBuilder::audio_base64)
    /// - [`commit`](InputAudioChunkPayloadBuilder::commit)
    /// - [`sample_rate`](InputAudioChunkPayloadBuilder::sample_rate)
    pub fn build(self) -> Result<InputAudioChunkPayload, BuildError> {
        Ok(InputAudioChunkPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
            commit: self.commit.ok_or_else(|| BuildError::missing_field("commit"))?,
            sample_rate: self.sample_rate.ok_or_else(|| BuildError::missing_field("sample_rate"))?,
            previous_text: self.previous_text,
        })
    }
}
