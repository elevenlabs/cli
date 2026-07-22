pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProjectVoiceResponseModel {
    #[serde(default)]
    pub project_voice_ref_id: String,
    #[serde(default)]
    pub voice_id: String,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub stability: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub similarity_boost: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub style: f64,
    #[serde(default)]
    pub is_pinned: bool,
    #[serde(default)]
    pub use_speaker_boost: bool,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub volume_gain: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub speed: f64,
}

impl ProjectVoiceResponseModel {
    pub fn builder() -> ProjectVoiceResponseModelBuilder {
        <ProjectVoiceResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectVoiceResponseModelBuilder {
    project_voice_ref_id: Option<String>,
    voice_id: Option<String>,
    alias: Option<String>,
    stability: Option<f64>,
    similarity_boost: Option<f64>,
    style: Option<f64>,
    is_pinned: Option<bool>,
    use_speaker_boost: Option<bool>,
    volume_gain: Option<f64>,
    speed: Option<f64>,
}

impl ProjectVoiceResponseModelBuilder {
    pub fn project_voice_ref_id(mut self, value: impl Into<String>) -> Self {
        self.project_voice_ref_id = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: f64) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    pub fn style(mut self, value: f64) -> Self {
        self.style = Some(value);
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    pub fn use_speaker_boost(mut self, value: bool) -> Self {
        self.use_speaker_boost = Some(value);
        self
    }

    pub fn volume_gain(mut self, value: f64) -> Self {
        self.volume_gain = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectVoiceResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_voice_ref_id`](ProjectVoiceResponseModelBuilder::project_voice_ref_id)
    /// - [`voice_id`](ProjectVoiceResponseModelBuilder::voice_id)
    /// - [`alias`](ProjectVoiceResponseModelBuilder::alias)
    /// - [`stability`](ProjectVoiceResponseModelBuilder::stability)
    /// - [`similarity_boost`](ProjectVoiceResponseModelBuilder::similarity_boost)
    /// - [`style`](ProjectVoiceResponseModelBuilder::style)
    /// - [`is_pinned`](ProjectVoiceResponseModelBuilder::is_pinned)
    /// - [`use_speaker_boost`](ProjectVoiceResponseModelBuilder::use_speaker_boost)
    /// - [`volume_gain`](ProjectVoiceResponseModelBuilder::volume_gain)
    /// - [`speed`](ProjectVoiceResponseModelBuilder::speed)
    pub fn build(self) -> Result<ProjectVoiceResponseModel, BuildError> {
        Ok(ProjectVoiceResponseModel {
            project_voice_ref_id: self.project_voice_ref_id.ok_or_else(|| BuildError::missing_field("project_voice_ref_id"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            alias: self.alias.ok_or_else(|| BuildError::missing_field("alias"))?,
            stability: self.stability.ok_or_else(|| BuildError::missing_field("stability"))?,
            similarity_boost: self.similarity_boost.ok_or_else(|| BuildError::missing_field("similarity_boost"))?,
            style: self.style.ok_or_else(|| BuildError::missing_field("style"))?,
            is_pinned: self.is_pinned.ok_or_else(|| BuildError::missing_field("is_pinned"))?,
            use_speaker_boost: self.use_speaker_boost.ok_or_else(|| BuildError::missing_field("use_speaker_boost"))?,
            volume_gain: self.volume_gain.ok_or_else(|| BuildError::missing_field("volume_gain"))?,
            speed: self.speed.ok_or_else(|| BuildError::missing_field("speed"))?,
        })
    }
}
