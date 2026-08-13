pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Overrides for the voice's saved settings, applied to one generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ElevenFlashV25VoiceSettings {
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
    /// The speed of the generated speech, where 1.0 is the voice's natural pace.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
}

impl ElevenFlashV25VoiceSettings {
    pub fn builder() -> ElevenFlashV25VoiceSettingsBuilder {
        <ElevenFlashV25VoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElevenFlashV25VoiceSettingsBuilder {
    stability: Option<f64>,
    similarity_boost: Option<f64>,
    speed: Option<f64>,
}

impl ElevenFlashV25VoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: f64) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ElevenFlashV25VoiceSettings`].
    pub fn build(self) -> Result<ElevenFlashV25VoiceSettings, BuildError> {
        Ok(ElevenFlashV25VoiceSettings {
            stability: self.stability,
            similarity_boost: self.similarity_boost,
            speed: self.speed,
        })
    }
}
