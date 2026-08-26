pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentTopicResponseModel {
    #[serde(default)]
    pub topic_id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub conversation_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_topic_id: Option<String>,
    #[serde(rename = "x_2d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub x2d: Option<f64>,
    #[serde(rename = "y_2d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub y2d: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<TopicMetricsAggregate>,
    /// Success rate across the topic's evaluation criteria, weighted by scored conversations. Returned regardless of include_evaluation_criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub success_rate: Option<f64>,
}

impl AgentTopicResponseModel {
    pub fn builder() -> AgentTopicResponseModelBuilder {
        <AgentTopicResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentTopicResponseModelBuilder {
    topic_id: Option<String>,
    label: Option<String>,
    description: Option<String>,
    conversation_count: Option<i64>,
    parent_topic_id: Option<String>,
    x2d: Option<f64>,
    y2d: Option<f64>,
    metrics: Option<TopicMetricsAggregate>,
    success_rate: Option<f64>,
}

impl AgentTopicResponseModelBuilder {
    pub fn topic_id(mut self, value: impl Into<String>) -> Self {
        self.topic_id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn conversation_count(mut self, value: i64) -> Self {
        self.conversation_count = Some(value);
        self
    }

    pub fn parent_topic_id(mut self, value: impl Into<String>) -> Self {
        self.parent_topic_id = Some(value.into());
        self
    }

    pub fn x2d(mut self, value: f64) -> Self {
        self.x2d = Some(value);
        self
    }

    pub fn y2d(mut self, value: f64) -> Self {
        self.y2d = Some(value);
        self
    }

    pub fn metrics(mut self, value: TopicMetricsAggregate) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn success_rate(mut self, value: f64) -> Self {
        self.success_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentTopicResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`topic_id`](AgentTopicResponseModelBuilder::topic_id)
    /// - [`label`](AgentTopicResponseModelBuilder::label)
    /// - [`description`](AgentTopicResponseModelBuilder::description)
    /// - [`conversation_count`](AgentTopicResponseModelBuilder::conversation_count)
    pub fn build(self) -> Result<AgentTopicResponseModel, BuildError> {
        Ok(AgentTopicResponseModel {
            topic_id: self.topic_id.ok_or_else(|| BuildError::missing_field("topic_id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            conversation_count: self.conversation_count.ok_or_else(|| BuildError::missing_field("conversation_count"))?,
            parent_topic_id: self.parent_topic_id,
            x2d: self.x2d,
            y2d: self.y2d,
            metrics: self.metrics,
            success_rate: self.success_rate,
        })
    }
}
