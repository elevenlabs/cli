pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSample {
    /// The ID of the sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_id: Option<String>,
    /// The name of the sample file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The MIME type of the sample file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The size of the sample file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// The hash of the sample file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub duration_secs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_background_noise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_isolated_audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_isolated_audio_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_separation: Option<SpeakerSeparationResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_end: Option<i64>,
}

impl VoiceSample {
    pub fn builder() -> VoiceSampleBuilder {
        <VoiceSampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSampleBuilder {
    sample_id: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
    hash: Option<String>,
    duration_secs: Option<f64>,
    remove_background_noise: Option<bool>,
    has_isolated_audio: Option<bool>,
    has_isolated_audio_preview: Option<bool>,
    speaker_separation: Option<SpeakerSeparationResponseModel>,
    trim_start: Option<i64>,
    trim_end: Option<i64>,
}

impl VoiceSampleBuilder {
    pub fn sample_id(mut self, value: impl Into<String>) -> Self {
        self.sample_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    pub fn duration_secs(mut self, value: f64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn remove_background_noise(mut self, value: bool) -> Self {
        self.remove_background_noise = Some(value);
        self
    }

    pub fn has_isolated_audio(mut self, value: bool) -> Self {
        self.has_isolated_audio = Some(value);
        self
    }

    pub fn has_isolated_audio_preview(mut self, value: bool) -> Self {
        self.has_isolated_audio_preview = Some(value);
        self
    }

    pub fn speaker_separation(mut self, value: SpeakerSeparationResponseModel) -> Self {
        self.speaker_separation = Some(value);
        self
    }

    pub fn trim_start(mut self, value: i64) -> Self {
        self.trim_start = Some(value);
        self
    }

    pub fn trim_end(mut self, value: i64) -> Self {
        self.trim_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSample`].
    pub fn build(self) -> Result<VoiceSample, BuildError> {
        Ok(VoiceSample {
            sample_id: self.sample_id,
            file_name: self.file_name,
            mime_type: self.mime_type,
            size_bytes: self.size_bytes,
            hash: self.hash,
            duration_secs: self.duration_secs,
            remove_background_noise: self.remove_background_noise,
            has_isolated_audio: self.has_isolated_audio,
            has_isolated_audio_preview: self.has_isolated_audio_preview,
            speaker_separation: self.speaker_separation,
            trim_start: self.trim_start,
            trim_end: self.trim_end,
        })
    }
}
