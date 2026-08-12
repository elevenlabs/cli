pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Base64-encoded audio of the translated speech.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranslateAudioPayload {
    /// The message type identifier.
    pub message_type: String,
    /// Base64-encoded audio data.
    #[serde(default)]
    pub data: String,
    /// Sample rate of the audio in Hz.
    #[serde(default)]
    pub sample_rate: i64,
}

impl TranslateAudioPayload {
    pub fn builder() -> TranslateAudioPayloadBuilder {
        <TranslateAudioPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranslateAudioPayloadBuilder {
    message_type: Option<String>,
    data: Option<String>,
    sample_rate: Option<i64>,
}

impl TranslateAudioPayloadBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn sample_rate(mut self, value: i64) -> Self {
        self.sample_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranslateAudioPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](TranslateAudioPayloadBuilder::message_type)
    /// - [`data`](TranslateAudioPayloadBuilder::data)
    /// - [`sample_rate`](TranslateAudioPayloadBuilder::sample_rate)
    pub fn build(self) -> Result<TranslateAudioPayload, BuildError> {
        Ok(TranslateAudioPayload {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            sample_rate: self.sample_rate.ok_or_else(|| BuildError::missing_field("sample_rate"))?,
        })
    }
}
