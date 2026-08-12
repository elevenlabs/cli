pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioWithTimestampsResponse {
    /// Base64 encoded audio data
    #[serde(default)]
    pub audio_base64: String,
    /// Timestamp information for each character in the original text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<CharacterAlignmentResponseModel>,
    /// Timestamp information for each character in the normalized text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_alignment: Option<CharacterAlignmentResponseModel>,
}

impl AudioWithTimestampsResponse {
    pub fn builder() -> AudioWithTimestampsResponseBuilder {
        <AudioWithTimestampsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioWithTimestampsResponseBuilder {
    audio_base64: Option<String>,
    alignment: Option<CharacterAlignmentResponseModel>,
    normalized_alignment: Option<CharacterAlignmentResponseModel>,
}

impl AudioWithTimestampsResponseBuilder {
    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
        self
    }

    pub fn alignment(mut self, value: CharacterAlignmentResponseModel) -> Self {
        self.alignment = Some(value);
        self
    }

    pub fn normalized_alignment(mut self, value: CharacterAlignmentResponseModel) -> Self {
        self.normalized_alignment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioWithTimestampsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base64`](AudioWithTimestampsResponseBuilder::audio_base64)
    pub fn build(self) -> Result<AudioWithTimestampsResponse, BuildError> {
        Ok(AudioWithTimestampsResponse {
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
            alignment: self.alignment,
            normalized_alignment: self.normalized_alignment,
        })
    }
}
