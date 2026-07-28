pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TtsConversationalConfigOverrideConfig {
    /// Whether to allow overriding the model_id field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<bool>,
    /// Whether to allow overriding the voice_id field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<bool>,
    /// Whether to allow overriding the stability field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<bool>,
    /// Whether to allow overriding the speed field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<bool>,
    /// Whether to allow overriding the similarity_boost field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<bool>,
}

impl TtsConversationalConfigOverrideConfig {
    pub fn builder() -> TtsConversationalConfigOverrideConfigBuilder {
        <TtsConversationalConfigOverrideConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsConversationalConfigOverrideConfigBuilder {
    model_id: Option<bool>,
    voice_id: Option<bool>,
    stability: Option<bool>,
    speed: Option<bool>,
    similarity_boost: Option<bool>,
}

impl TtsConversationalConfigOverrideConfigBuilder {
    pub fn model_id(mut self, value: bool) -> Self {
        self.model_id = Some(value);
        self
    }

    pub fn voice_id(mut self, value: bool) -> Self {
        self.voice_id = Some(value);
        self
    }

    pub fn stability(mut self, value: bool) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn speed(mut self, value: bool) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn similarity_boost(mut self, value: bool) -> Self {
        self.similarity_boost = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TtsConversationalConfigOverrideConfig`].
    pub fn build(self) -> Result<TtsConversationalConfigOverrideConfig, BuildError> {
        Ok(TtsConversationalConfigOverrideConfig {
            model_id: self.model_id,
            voice_id: self.voice_id,
            stability: self.stability,
            speed: self.speed,
            similarity_boost: self.similarity_boost,
        })
    }
}
