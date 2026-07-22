pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSamplePreviewResponseModel {
    /// The base64 encoded audio.
    #[serde(rename = "audio_base_64")]
    #[serde(default)]
    pub audio_base64: String,
    /// The ID of the voice.
    #[serde(default)]
    pub voice_id: String,
    /// The ID of the sample.
    #[serde(default)]
    pub sample_id: String,
    /// The media type of the audio.
    #[serde(default)]
    pub media_type: String,
    /// The duration of the audio in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_secs: Option<f64>,
}

impl VoiceSamplePreviewResponseModel {
    pub fn builder() -> VoiceSamplePreviewResponseModelBuilder {
        <VoiceSamplePreviewResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSamplePreviewResponseModelBuilder {
    audio_base64: Option<String>,
    voice_id: Option<String>,
    sample_id: Option<String>,
    media_type: Option<String>,
    duration_secs: Option<f64>,
}

impl VoiceSamplePreviewResponseModelBuilder {
    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn sample_id(mut self, value: impl Into<String>) -> Self {
        self.sample_id = Some(value.into());
        self
    }

    pub fn media_type(mut self, value: impl Into<String>) -> Self {
        self.media_type = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSamplePreviewResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base64`](VoiceSamplePreviewResponseModelBuilder::audio_base64)
    /// - [`voice_id`](VoiceSamplePreviewResponseModelBuilder::voice_id)
    /// - [`sample_id`](VoiceSamplePreviewResponseModelBuilder::sample_id)
    /// - [`media_type`](VoiceSamplePreviewResponseModelBuilder::media_type)
    pub fn build(self) -> Result<VoiceSamplePreviewResponseModel, BuildError> {
        Ok(VoiceSamplePreviewResponseModel {
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            sample_id: self.sample_id.ok_or_else(|| BuildError::missing_field("sample_id"))?,
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            duration_secs: self.duration_secs,
        })
    }
}
