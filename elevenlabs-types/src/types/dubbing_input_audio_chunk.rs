pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingInputAudioChunk {
    pub message_type: String,
    #[serde(rename = "audio_base_64")]
    #[serde(default)]
    pub audio_base64: String,
}

impl DubbingInputAudioChunk {
    pub fn builder() -> DubbingInputAudioChunkBuilder {
        <DubbingInputAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingInputAudioChunkBuilder {
    message_type: Option<String>,
    audio_base64: Option<String>,
}

impl DubbingInputAudioChunkBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingInputAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingInputAudioChunkBuilder::message_type)
    /// - [`audio_base64`](DubbingInputAudioChunkBuilder::audio_base64)
    pub fn build(self) -> Result<DubbingInputAudioChunk, BuildError> {
        Ok(DubbingInputAudioChunk {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
        })
    }
}
