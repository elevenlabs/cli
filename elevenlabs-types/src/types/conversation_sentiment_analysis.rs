pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationSentimentAnalysis {
    pub overall_label: ConversationSentimentAnalysisOverallLabel,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub overall_sentiment_score: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub overall_frustration_score: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min_user_sentiment_score: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub max_user_frustration_score: f64,
    #[serde(default)]
    pub num_scored_user_turns: i64,
}

impl ConversationSentimentAnalysis {
    pub fn builder() -> ConversationSentimentAnalysisBuilder {
        <ConversationSentimentAnalysisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationSentimentAnalysisBuilder {
    overall_label: Option<ConversationSentimentAnalysisOverallLabel>,
    overall_sentiment_score: Option<f64>,
    overall_frustration_score: Option<f64>,
    min_user_sentiment_score: Option<f64>,
    max_user_frustration_score: Option<f64>,
    num_scored_user_turns: Option<i64>,
}

impl ConversationSentimentAnalysisBuilder {
    pub fn overall_label(mut self, value: ConversationSentimentAnalysisOverallLabel) -> Self {
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

    pub fn min_user_sentiment_score(mut self, value: f64) -> Self {
        self.min_user_sentiment_score = Some(value);
        self
    }

    pub fn max_user_frustration_score(mut self, value: f64) -> Self {
        self.max_user_frustration_score = Some(value);
        self
    }

    pub fn num_scored_user_turns(mut self, value: i64) -> Self {
        self.num_scored_user_turns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationSentimentAnalysis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`overall_label`](ConversationSentimentAnalysisBuilder::overall_label)
    /// - [`overall_sentiment_score`](ConversationSentimentAnalysisBuilder::overall_sentiment_score)
    /// - [`overall_frustration_score`](ConversationSentimentAnalysisBuilder::overall_frustration_score)
    /// - [`min_user_sentiment_score`](ConversationSentimentAnalysisBuilder::min_user_sentiment_score)
    /// - [`max_user_frustration_score`](ConversationSentimentAnalysisBuilder::max_user_frustration_score)
    /// - [`num_scored_user_turns`](ConversationSentimentAnalysisBuilder::num_scored_user_turns)
    pub fn build(self) -> Result<ConversationSentimentAnalysis, BuildError> {
        Ok(ConversationSentimentAnalysis {
            overall_label: self.overall_label.ok_or_else(|| BuildError::missing_field("overall_label"))?,
            overall_sentiment_score: self.overall_sentiment_score.ok_or_else(|| BuildError::missing_field("overall_sentiment_score"))?,
            overall_frustration_score: self.overall_frustration_score.ok_or_else(|| BuildError::missing_field("overall_frustration_score"))?,
            min_user_sentiment_score: self.min_user_sentiment_score.ok_or_else(|| BuildError::missing_field("min_user_sentiment_score"))?,
            max_user_frustration_score: self.max_user_frustration_score.ok_or_else(|| BuildError::missing_field("max_user_frustration_score"))?,
            num_scored_user_turns: self.num_scored_user_turns.ok_or_else(|| BuildError::missing_field("num_scored_user_turns"))?,
        })
    }
}
