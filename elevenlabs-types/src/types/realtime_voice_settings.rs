pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RealtimeVoiceSettings {
    /// Defines the stability for voice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// Defines the similarity boost for voice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
    /// Defines the style for voice settings. This parameter is available on V2+ models.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub style: Option<f64>,
    /// Defines the use speaker boost for voice settings. This parameter is available on V2+ models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
    /// Controls the speed of the generated speech. Values range from 0.7 to 1.2, with 1.0 being the default speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
}

impl RealtimeVoiceSettings {
    pub fn builder() -> RealtimeVoiceSettingsBuilder {
        <RealtimeVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RealtimeVoiceSettingsBuilder {
    stability: Option<f64>,
    similarity_boost: Option<f64>,
    style: Option<f64>,
    use_speaker_boost: Option<bool>,
    speed: Option<f64>,
}

impl RealtimeVoiceSettingsBuilder {
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

    pub fn use_speaker_boost(mut self, value: bool) -> Self {
        self.use_speaker_boost = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RealtimeVoiceSettings`].
    pub fn build(self) -> Result<RealtimeVoiceSettings, BuildError> {
        Ok(RealtimeVoiceSettings {
            stability: self.stability,
            similarity_boost: self.similarity_boost,
            style: self.style,
            use_speaker_boost: self.use_speaker_boost,
            speed: self.speed,
        })
    }
}
