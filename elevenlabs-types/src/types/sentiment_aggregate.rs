pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SentimentAggregate {
    #[serde(default)]
    pub scored_conversation_count: i64,
    #[serde(default)]
    pub positive_count: i64,
    #[serde(default)]
    pub neutral_count: i64,
    #[serde(default)]
    pub negative_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_sentiment_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_frustration_score: Option<f64>,
    #[serde(default)]
    pub recent_scored_conversation_count: i64,
    #[serde(default)]
    pub recent_positive_count: i64,
    #[serde(default)]
    pub recent_neutral_count: i64,
    #[serde(default)]
    pub recent_negative_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub recent_average_sentiment_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub recent_average_frustration_score: Option<f64>,
}

impl SentimentAggregate {
    pub fn builder() -> SentimentAggregateBuilder {
        <SentimentAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SentimentAggregateBuilder {
    scored_conversation_count: Option<i64>,
    positive_count: Option<i64>,
    neutral_count: Option<i64>,
    negative_count: Option<i64>,
    average_sentiment_score: Option<f64>,
    average_frustration_score: Option<f64>,
    recent_scored_conversation_count: Option<i64>,
    recent_positive_count: Option<i64>,
    recent_neutral_count: Option<i64>,
    recent_negative_count: Option<i64>,
    recent_average_sentiment_score: Option<f64>,
    recent_average_frustration_score: Option<f64>,
}

impl SentimentAggregateBuilder {
    pub fn scored_conversation_count(mut self, value: i64) -> Self {
        self.scored_conversation_count = Some(value);
        self
    }

    pub fn positive_count(mut self, value: i64) -> Self {
        self.positive_count = Some(value);
        self
    }

    pub fn neutral_count(mut self, value: i64) -> Self {
        self.neutral_count = Some(value);
        self
    }

    pub fn negative_count(mut self, value: i64) -> Self {
        self.negative_count = Some(value);
        self
    }

    pub fn average_sentiment_score(mut self, value: f64) -> Self {
        self.average_sentiment_score = Some(value);
        self
    }

    pub fn average_frustration_score(mut self, value: f64) -> Self {
        self.average_frustration_score = Some(value);
        self
    }

    pub fn recent_scored_conversation_count(mut self, value: i64) -> Self {
        self.recent_scored_conversation_count = Some(value);
        self
    }

    pub fn recent_positive_count(mut self, value: i64) -> Self {
        self.recent_positive_count = Some(value);
        self
    }

    pub fn recent_neutral_count(mut self, value: i64) -> Self {
        self.recent_neutral_count = Some(value);
        self
    }

    pub fn recent_negative_count(mut self, value: i64) -> Self {
        self.recent_negative_count = Some(value);
        self
    }

    pub fn recent_average_sentiment_score(mut self, value: f64) -> Self {
        self.recent_average_sentiment_score = Some(value);
        self
    }

    pub fn recent_average_frustration_score(mut self, value: f64) -> Self {
        self.recent_average_frustration_score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SentimentAggregate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scored_conversation_count`](SentimentAggregateBuilder::scored_conversation_count)
    /// - [`positive_count`](SentimentAggregateBuilder::positive_count)
    /// - [`neutral_count`](SentimentAggregateBuilder::neutral_count)
    /// - [`negative_count`](SentimentAggregateBuilder::negative_count)
    /// - [`recent_scored_conversation_count`](SentimentAggregateBuilder::recent_scored_conversation_count)
    /// - [`recent_positive_count`](SentimentAggregateBuilder::recent_positive_count)
    /// - [`recent_neutral_count`](SentimentAggregateBuilder::recent_neutral_count)
    /// - [`recent_negative_count`](SentimentAggregateBuilder::recent_negative_count)
    pub fn build(self) -> Result<SentimentAggregate, BuildError> {
        Ok(SentimentAggregate {
            scored_conversation_count: self.scored_conversation_count.ok_or_else(|| BuildError::missing_field("scored_conversation_count"))?,
            positive_count: self.positive_count.ok_or_else(|| BuildError::missing_field("positive_count"))?,
            neutral_count: self.neutral_count.ok_or_else(|| BuildError::missing_field("neutral_count"))?,
            negative_count: self.negative_count.ok_or_else(|| BuildError::missing_field("negative_count"))?,
            average_sentiment_score: self.average_sentiment_score,
            average_frustration_score: self.average_frustration_score,
            recent_scored_conversation_count: self.recent_scored_conversation_count.ok_or_else(|| BuildError::missing_field("recent_scored_conversation_count"))?,
            recent_positive_count: self.recent_positive_count.ok_or_else(|| BuildError::missing_field("recent_positive_count"))?,
            recent_neutral_count: self.recent_neutral_count.ok_or_else(|| BuildError::missing_field("recent_neutral_count"))?,
            recent_negative_count: self.recent_negative_count.ok_or_else(|| BuildError::missing_field("recent_negative_count"))?,
            recent_average_sentiment_score: self.recent_average_sentiment_score,
            recent_average_frustration_score: self.recent_average_frustration_score,
        })
    }
}
