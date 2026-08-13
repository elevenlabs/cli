pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TopicEvaluationCriteriaAggregate {
    #[serde(default)]
    pub criteria_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_count: Option<i64>,
}

impl TopicEvaluationCriteriaAggregate {
    pub fn builder() -> TopicEvaluationCriteriaAggregateBuilder {
        <TopicEvaluationCriteriaAggregateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TopicEvaluationCriteriaAggregateBuilder {
    criteria_id: Option<String>,
    success_count: Option<i64>,
    failure_count: Option<i64>,
    unknown_count: Option<i64>,
}

impl TopicEvaluationCriteriaAggregateBuilder {
    pub fn criteria_id(mut self, value: impl Into<String>) -> Self {
        self.criteria_id = Some(value.into());
        self
    }

    pub fn success_count(mut self, value: i64) -> Self {
        self.success_count = Some(value);
        self
    }

    pub fn failure_count(mut self, value: i64) -> Self {
        self.failure_count = Some(value);
        self
    }

    pub fn unknown_count(mut self, value: i64) -> Self {
        self.unknown_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TopicEvaluationCriteriaAggregate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`criteria_id`](TopicEvaluationCriteriaAggregateBuilder::criteria_id)
    pub fn build(self) -> Result<TopicEvaluationCriteriaAggregate, BuildError> {
        Ok(TopicEvaluationCriteriaAggregate {
            criteria_id: self.criteria_id.ok_or_else(|| BuildError::missing_field("criteria_id"))?,
            success_count: self.success_count,
            failure_count: self.failure_count,
            unknown_count: self.unknown_count,
        })
    }
}
