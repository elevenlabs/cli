pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewAudioDbModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    pub audio_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_manifest_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_manifest_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_generated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at_unix: Option<i64>,
}

impl PreviewAudioDbModel {
    pub fn builder() -> PreviewAudioDbModelBuilder {
        <PreviewAudioDbModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewAudioDbModelBuilder {
    voice_id: Option<String>,
    text: Option<String>,
    audio_url: Option<String>,
    hls_manifest_url: Option<String>,
    dash_manifest_url: Option<String>,
    is_auto_generated: Option<bool>,
    generated_at_unix: Option<i64>,
}

impl PreviewAudioDbModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    pub fn hls_manifest_url(mut self, value: impl Into<String>) -> Self {
        self.hls_manifest_url = Some(value.into());
        self
    }

    pub fn dash_manifest_url(mut self, value: impl Into<String>) -> Self {
        self.dash_manifest_url = Some(value.into());
        self
    }

    pub fn is_auto_generated(mut self, value: bool) -> Self {
        self.is_auto_generated = Some(value);
        self
    }

    pub fn generated_at_unix(mut self, value: i64) -> Self {
        self.generated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewAudioDbModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_url`](PreviewAudioDbModelBuilder::audio_url)
    pub fn build(self) -> Result<PreviewAudioDbModel, BuildError> {
        Ok(PreviewAudioDbModel {
            voice_id: self.voice_id,
            text: self.text,
            audio_url: self.audio_url.ok_or_else(|| BuildError::missing_field("audio_url"))?,
            hls_manifest_url: self.hls_manifest_url,
            dash_manifest_url: self.dash_manifest_url,
            is_auto_generated: self.is_auto_generated,
            generated_at_unix: self.generated_at_unix,
        })
    }
}
