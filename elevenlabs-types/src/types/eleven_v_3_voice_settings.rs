pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Overrides for the voice's saved settings, applied to one generation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ElevenV3VoiceSettings {
    /// How consistent the voice stays across generations. Lower values give more expressive, varied speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
}

impl ElevenV3VoiceSettings {
    pub fn builder() -> ElevenV3VoiceSettingsBuilder {
        <ElevenV3VoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ElevenV3VoiceSettingsBuilder {
    stability: Option<f64>,
}

impl ElevenV3VoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ElevenV3VoiceSettings`].
    pub fn build(self) -> Result<ElevenV3VoiceSettings, BuildError> {
        Ok(ElevenV3VoiceSettings {
            stability: self.stability,
        })
    }
}
