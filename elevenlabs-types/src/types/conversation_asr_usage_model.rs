pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Aggregated ASR usage for a conversation (analytics-only, not billing).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationAsrUsageModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_transcription_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_audio_input_seconds: Option<f64>,
}

impl ConversationAsrUsageModel {
    pub fn builder() -> ConversationAsrUsageModelBuilder {
        <ConversationAsrUsageModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAsrUsageModelBuilder {
    asr_model: Option<String>,
    total_transcription_calls: Option<i64>,
    total_audio_input_seconds: Option<f64>,
}

impl ConversationAsrUsageModelBuilder {
    pub fn asr_model(mut self, value: impl Into<String>) -> Self {
        self.asr_model = Some(value.into());
        self
    }

    pub fn total_transcription_calls(mut self, value: i64) -> Self {
        self.total_transcription_calls = Some(value);
        self
    }

    pub fn total_audio_input_seconds(mut self, value: f64) -> Self {
        self.total_audio_input_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationAsrUsageModel`].
    pub fn build(self) -> Result<ConversationAsrUsageModel, BuildError> {
        Ok(ConversationAsrUsageModel {
            asr_model: self.asr_model,
            total_transcription_calls: self.total_transcription_calls,
            total_audio_input_seconds: self.total_audio_input_seconds,
        })
    }
}
