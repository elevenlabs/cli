pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Overrides for the voice's saved settings, applied to one generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TtsVoiceSettings {
    /// How consistent the voice stays across generations. Lower values give more expressive, varied speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// How closely the output adheres to the original voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
    /// How strongly the speaking style is exaggerated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub style: Option<f64>,
    /// Whether to boost similarity to the original speaker, at some latency cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
    /// The speed of the generated speech, where 1.0 is the voice's natural pace.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
}

impl TtsVoiceSettings {
    pub fn builder() -> TtsVoiceSettingsBuilder {
        <TtsVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsVoiceSettingsBuilder {
    stability: Option<f64>,
    similarity_boost: Option<f64>,
    style: Option<f64>,
    use_speaker_boost: Option<bool>,
    speed: Option<f64>,
}

impl TtsVoiceSettingsBuilder {
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

    /// Consumes the builder and constructs a [`TtsVoiceSettings`].
    pub fn build(self) -> Result<TtsVoiceSettings, BuildError> {
        Ok(TtsVoiceSettings {
            stability: self.stability,
            similarity_boost: self.similarity_boost,
            style: self.style,
            use_speaker_boost: self.use_speaker_boost,
            speed: self.speed,
        })
    }
}
