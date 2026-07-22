pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TtsConversationalConfigOverride {
    /// The model to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<TtsConversationalModel>,
    /// The voice ID to use for TTS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// The stability of generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// The speed of generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
    /// The similarity boost for generated speech
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
}

impl TtsConversationalConfigOverride {
    pub fn builder() -> TtsConversationalConfigOverrideBuilder {
        <TtsConversationalConfigOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsConversationalConfigOverrideBuilder {
    model_id: Option<TtsConversationalModel>,
    voice_id: Option<String>,
    stability: Option<f64>,
    speed: Option<f64>,
    similarity_boost: Option<f64>,
}

impl TtsConversationalConfigOverrideBuilder {
    pub fn model_id(mut self, value: TtsConversationalModel) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: f64) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TtsConversationalConfigOverride`].
    pub fn build(self) -> Result<TtsConversationalConfigOverride, BuildError> {
        Ok(TtsConversationalConfigOverride {
            model_id: self.model_id,
            voice_id: self.voice_id,
            stability: self.stability,
            speed: self.speed,
            similarity_boost: self.similarity_boost,
        })
    }
}
