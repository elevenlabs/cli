pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationVoiceUsageModel {
    #[serde(default)]
    pub voice_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audio_output_seconds: Option<f64>,
}

impl ConversationVoiceUsageModel {
    pub fn builder() -> ConversationVoiceUsageModelBuilder {
        <ConversationVoiceUsageModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationVoiceUsageModelBuilder {
    voice_id: Option<String>,
    audio_output_seconds: Option<f64>,
}

impl ConversationVoiceUsageModelBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn audio_output_seconds(mut self, value: f64) -> Self {
        self.audio_output_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationVoiceUsageModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](ConversationVoiceUsageModelBuilder::voice_id)
    pub fn build(self) -> Result<ConversationVoiceUsageModel, BuildError> {
        Ok(ConversationVoiceUsageModel {
            voice_id: self.voice_id.ok_or_else(|| BuildError::missing_field("voice_id"))?,
            audio_output_seconds: self.audio_output_seconds,
        })
    }
}
