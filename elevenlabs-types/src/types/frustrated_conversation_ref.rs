pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FrustratedConversationRef {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub start_time_unix_secs: i64,
    pub overall_label: FrustratedConversationRefOverallLabel,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub overall_sentiment_score: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub overall_frustration_score: f64,
}

impl FrustratedConversationRef {
    pub fn builder() -> FrustratedConversationRefBuilder {
        <FrustratedConversationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrustratedConversationRefBuilder {
    conversation_id: Option<String>,
    agent_id: Option<String>,
    start_time_unix_secs: Option<i64>,
    overall_label: Option<FrustratedConversationRefOverallLabel>,
    overall_sentiment_score: Option<f64>,
    overall_frustration_score: Option<f64>,
}

impl FrustratedConversationRefBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn start_time_unix_secs(mut self, value: i64) -> Self {
        self.start_time_unix_secs = Some(value);
        self
    }

    pub fn overall_label(mut self, value: FrustratedConversationRefOverallLabel) -> Self {
        self.overall_label = Some(value);
        self
    }

    pub fn overall_sentiment_score(mut self, value: f64) -> Self {
        self.overall_sentiment_score = Some(value);
        self
    }

    pub fn overall_frustration_score(mut self, value: f64) -> Self {
        self.overall_frustration_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FrustratedConversationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](FrustratedConversationRefBuilder::conversation_id)
    /// - [`agent_id`](FrustratedConversationRefBuilder::agent_id)
    /// - [`start_time_unix_secs`](FrustratedConversationRefBuilder::start_time_unix_secs)
    /// - [`overall_label`](FrustratedConversationRefBuilder::overall_label)
    /// - [`overall_sentiment_score`](FrustratedConversationRefBuilder::overall_sentiment_score)
    /// - [`overall_frustration_score`](FrustratedConversationRefBuilder::overall_frustration_score)
    pub fn build(self) -> Result<FrustratedConversationRef, BuildError> {
        Ok(FrustratedConversationRef {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            start_time_unix_secs: self.start_time_unix_secs.ok_or_else(|| BuildError::missing_field("start_time_unix_secs"))?,
            overall_label: self.overall_label.ok_or_else(|| BuildError::missing_field("overall_label"))?,
            overall_sentiment_score: self.overall_sentiment_score.ok_or_else(|| BuildError::missing_field("overall_sentiment_score"))?,
            overall_frustration_score: self.overall_frustration_score.ok_or_else(|| BuildError::missing_field("overall_frustration_score"))?,
        })
    }
}
