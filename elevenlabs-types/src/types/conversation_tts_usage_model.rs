pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Aggregated TTS usage for a conversation (analytics-only, not billing).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationTtsUsageModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_tts_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_audio_output_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_characters: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_voice_usage: Option<Vec<ConversationVoiceUsageModel>>,
}

impl ConversationTtsUsageModel {
    pub fn builder() -> ConversationTtsUsageModelBuilder {
        <ConversationTtsUsageModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationTtsUsageModelBuilder {
    primary_tts_model: Option<String>,
    total_audio_output_seconds: Option<f64>,
    total_characters: Option<i64>,
    per_voice_usage: Option<Vec<ConversationVoiceUsageModel>>,
}

impl ConversationTtsUsageModelBuilder {
    pub fn primary_tts_model(mut self, value: impl Into<String>) -> Self {
        self.primary_tts_model = Some(value.into());
        self
    }

    pub fn total_audio_output_seconds(mut self, value: f64) -> Self {
        self.total_audio_output_seconds = Some(value);
        self
    }

    pub fn total_characters(mut self, value: i64) -> Self {
        self.total_characters = Some(value);
        self
    }

    pub fn per_voice_usage(mut self, value: Vec<ConversationVoiceUsageModel>) -> Self {
        self.per_voice_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationTtsUsageModel`].
    pub fn build(self) -> Result<ConversationTtsUsageModel, BuildError> {
        Ok(ConversationTtsUsageModel {
            primary_tts_model: self.primary_tts_model,
            total_audio_output_seconds: self.total_audio_output_seconds,
            total_characters: self.total_characters,
            per_voice_usage: self.per_voice_usage,
        })
    }
}
