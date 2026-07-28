pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TopicMetricsAggregate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<TopicSentimentAggregate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_criteria: Option<Vec<TopicEvaluationCriteriaAggregate>>,
}

impl TopicMetricsAggregate {
    pub fn builder() -> TopicMetricsAggregateBuilder {
        <TopicMetricsAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopicMetricsAggregateBuilder {
    conversation_count: Option<i64>,
    sentiment: Option<TopicSentimentAggregate>,
    evaluation_criteria: Option<Vec<TopicEvaluationCriteriaAggregate>>,
}

impl TopicMetricsAggregateBuilder {
    pub fn conversation_count(mut self, value: i64) -> Self {
        self.conversation_count = Some(value);
        self
    }

    pub fn sentiment(mut self, value: TopicSentimentAggregate) -> Self {
        self.sentiment = Some(value);
        self
    }

    pub fn evaluation_criteria(mut self, value: Vec<TopicEvaluationCriteriaAggregate>) -> Self {
        self.evaluation_criteria = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TopicMetricsAggregate`].
    pub fn build(self) -> Result<TopicMetricsAggregate, BuildError> {
        Ok(TopicMetricsAggregate {
            conversation_count: self.conversation_count,
            sentiment: self.sentiment,
            evaluation_criteria: self.evaluation_criteria,
        })
    }
}
