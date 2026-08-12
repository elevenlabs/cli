pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationTurnMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, MetricRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convai_asr_provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convai_tts_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convai_tts_cascade: Option<String>,
}

impl ConversationTurnMetrics {
    pub fn builder() -> ConversationTurnMetricsBuilder {
        <ConversationTurnMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationTurnMetricsBuilder {
    metrics: Option<HashMap<String, MetricRecord>>,
    convai_asr_provider: Option<String>,
    convai_tts_model: Option<String>,
    convai_tts_cascade: Option<String>,
}

impl ConversationTurnMetricsBuilder {
    pub fn metrics(mut self, value: HashMap<String, MetricRecord>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn convai_asr_provider(mut self, value: impl Into<String>) -> Self {
        self.convai_asr_provider = Some(value.into());
        self
    }

    pub fn convai_tts_model(mut self, value: impl Into<String>) -> Self {
        self.convai_tts_model = Some(value.into());
        self
    }

    pub fn convai_tts_cascade(mut self, value: impl Into<String>) -> Self {
        self.convai_tts_cascade = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationTurnMetrics`].
    pub fn build(self) -> Result<ConversationTurnMetrics, BuildError> {
        Ok(ConversationTurnMetrics {
            metrics: self.metrics,
            convai_asr_provider: self.convai_asr_provider,
            convai_tts_model: self.convai_tts_model,
            convai_tts_cascade: self.convai_tts_cascade,
        })
    }
}
