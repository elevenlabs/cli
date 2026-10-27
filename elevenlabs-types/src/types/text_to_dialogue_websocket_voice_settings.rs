pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Voice settings for dialogue generation. Only `stability` is supported for `eleven_v3` dialogue models.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TextToDialogueWebsocketVoiceSettings {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// Controls how closely the voice matches the professional voice clone samples, from 0 (low) to 1 (high). We recommend 0 for most use cases. Increase this value only if you experience issues with the default settings. Only applies to professional voice clones fine-tuned for the selected v4 model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub clone_strength: Option<f64>,
}

impl TextToDialogueWebsocketVoiceSettings {
    pub fn builder() -> TextToDialogueWebsocketVoiceSettingsBuilder {
        <TextToDialogueWebsocketVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueWebsocketVoiceSettingsBuilder {
    stability: Option<f64>,
    clone_strength: Option<f64>,
}

impl TextToDialogueWebsocketVoiceSettingsBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn clone_strength(mut self, value: f64) -> Self {
        self.clone_strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueWebsocketVoiceSettings`].
    pub fn build(self) -> Result<TextToDialogueWebsocketVoiceSettings, BuildError> {
        Ok(TextToDialogueWebsocketVoiceSettings {
            stability: self.stability,
            clone_strength: self.clone_strength,
        })
    }
}
