pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Multipart response structure with JSON metadata and binary audio
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MultipartMusicResponse {
    /// JSON metadata about the generated audio
    pub metadata: DetailedMusicResponse,
    /// Binary audio data in the requested format
    #[serde(default)]
    pub audio: String,
}

impl MultipartMusicResponse {
    pub fn builder() -> MultipartMusicResponseBuilder {
        <MultipartMusicResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MultipartMusicResponseBuilder {
    metadata: Option<DetailedMusicResponse>,
    audio: Option<String>,
}

impl MultipartMusicResponseBuilder {
    pub fn metadata(mut self, value: DetailedMusicResponse) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MultipartMusicResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`metadata`](MultipartMusicResponseBuilder::metadata)
    /// - [`audio`](MultipartMusicResponseBuilder::audio)
    pub fn build(self) -> Result<MultipartMusicResponse, BuildError> {
        Ok(MultipartMusicResponse {
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
        })
    }
}
