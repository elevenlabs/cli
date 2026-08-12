pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SupportedVoice {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub voice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_family: Option<TtsModelFamily>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_streaming_latency: Option<TtsOptimizeStreamingLatency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity_boost: Option<f64>,
}

impl SupportedVoice {
    pub fn builder() -> SupportedVoiceBuilder {
        <SupportedVoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportedVoiceBuilder {
    label: Option<String>,
    voice_id: Option<String>,
    description: Option<String>,
    language: Option<String>,
    model_family: Option<TtsModelFamily>,
    optimize_streaming_latency: Option<TtsOptimizeStreamingLatency>,
    stability: Option<f64>,
    speed: Option<f64>,
    similarity_boost: Option<f64>,
}

impl SupportedVoiceBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model_family(mut self, value: TtsModelFamily) -> Self {
        self.model_family = Some(value);
        self
    }

    pub fn optimize_streaming_latency(mut self, value: TtsOptimizeStreamingLatency) -> Self {
        self.optimize_streaming_latency = Some(value);
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

    /// Consumes the builder and constructs a [`SupportedVoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](SupportedVoiceBuilder::label)
    /// - [`voice_id`](SupportedVoiceBuilder::voice_id)
    pub fn build(self) -> Result<SupportedVoice, BuildError> {
        Ok(SupportedVoice {
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            description: self.description,
            language: self.language,
            model_family: self.model_family,
            optimize_streaming_latency: self.optimize_streaming_latency,
            stability: self.stability,
            speed: self.speed,
            similarity_boost: self.similarity_boost,
        })
    }
}
