pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSettings {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// This setting boosts the similarity to the original speaker. Using this setting requires a slightly higher computational load, which in turn increases latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
    /// Determines how closely the AI should adhere to the original voice when attempting to replicate it.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
    /// Determines the style exaggeration of the voice. This setting attempts to amplify the style of the original speaker. It does consume additional computational resources and might increase latency if set to anything other than 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub style: Option<f64>,
    /// Adjusts the speed of the voice. A value of 1.0 is the default speed, while values less than 1.0 slow down the speech, and values greater than 1.0 speed it up.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
}

impl VoiceSettings {
    pub fn builder() -> VoiceSettingsBuilder {
        <VoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSettingsBuilder {
    stability: Option<f64>,
    use_speaker_boost: Option<bool>,
    similarity_boost: Option<f64>,
    style: Option<f64>,
    speed: Option<f64>,
}

impl VoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn use_speaker_boost(mut self, value: bool) -> Self {
        self.use_speaker_boost = Some(value);
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

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSettings`].
    pub fn build(self) -> Result<VoiceSettings, BuildError> {
        Ok(VoiceSettings {
            stability: self.stability,
            use_speaker_boost: self.use_speaker_boost,
            similarity_boost: self.similarity_boost,
            style: self.style,
            speed: self.speed,
        })
    }
}
