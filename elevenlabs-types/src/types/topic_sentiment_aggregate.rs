pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TopicSentimentAggregate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<NumericDistributionAggregate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frustration: Option<NumericDistributionAggregate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_count: Option<i64>,
}

impl TopicSentimentAggregate {
    pub fn builder() -> TopicSentimentAggregateBuilder {
        <TopicSentimentAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopicSentimentAggregateBuilder {
    sentiment: Option<NumericDistributionAggregate>,
    frustration: Option<NumericDistributionAggregate>,
    positive_count: Option<i64>,
    neutral_count: Option<i64>,
    negative_count: Option<i64>,
}

impl TopicSentimentAggregateBuilder {
    pub fn sentiment(mut self, value: NumericDistributionAggregate) -> Self {
        self.sentiment = Some(value);
        self
    }

    pub fn frustration(mut self, value: NumericDistributionAggregate) -> Self {
        self.frustration = Some(value);
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

    /// Consumes the builder and constructs a [`TopicSentimentAggregate`].
    pub fn build(self) -> Result<TopicSentimentAggregate, BuildError> {
        Ok(TopicSentimentAggregate {
            sentiment: self.sentiment,
            frustration: self.frustration,
            positive_count: self.positive_count,
            neutral_count: self.neutral_count,
            negative_count: self.negative_count,
        })
    }
}
