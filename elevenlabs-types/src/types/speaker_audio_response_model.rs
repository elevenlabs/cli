pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeakerAudioResponseModel {
    /// The base64 encoded audio.
    #[serde(rename = "audio_base_64")]
    #[serde(default)]
    pub audio_base64: String,
    /// The media type of the audio.
    #[serde(default)]
    pub media_type: String,
    /// The duration of the audio in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_secs: f64,
}

impl SpeakerAudioResponseModel {
    pub fn builder() -> SpeakerAudioResponseModelBuilder {
        <SpeakerAudioResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeakerAudioResponseModelBuilder {
    audio_base64: Option<String>,
    media_type: Option<String>,
    duration_secs: Option<f64>,
}

impl SpeakerAudioResponseModelBuilder {
    pub fn audio_base64(mut self, value: impl Into<String>) -> Self {
        self.audio_base64 = Some(value.into());
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

    /// Consumes the builder and constructs a [`SpeakerAudioResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base64`](SpeakerAudioResponseModelBuilder::audio_base64)
    /// - [`media_type`](SpeakerAudioResponseModelBuilder::media_type)
    /// - [`duration_secs`](SpeakerAudioResponseModelBuilder::duration_secs)
    pub fn build(self) -> Result<SpeakerAudioResponseModel, BuildError> {
        Ok(SpeakerAudioResponseModel {
            audio_base64: self.audio_base64.ok_or_else(|| BuildError::missing_field("audio_base64"))?,
            media_type: self.media_type.ok_or_else(|| BuildError::missing_field("media_type"))?,
            duration_secs: self.duration_secs.ok_or_else(|| BuildError::missing_field("duration_secs"))?,
        })
    }
}
