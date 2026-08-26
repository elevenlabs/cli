pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAgentTopicsResponseModel {
    #[serde(default)]
    pub topics: Vec<AgentTopicResponseModel>,
    #[serde(default)]
    pub window_start_unix_secs: i64,
    #[serde(default)]
    pub window_end_unix_secs: i64,
    /// Number of daily topic-discovery runs the returned metrics were summed over.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_run_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl GetAgentTopicsResponseModel {
    pub fn builder() -> GetAgentTopicsResponseModelBuilder {
        <GetAgentTopicsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentTopicsResponseModelBuilder {
    topics: Option<Vec<AgentTopicResponseModel>>,
    window_start_unix_secs: Option<i64>,
    window_end_unix_secs: Option<i64>,
    aggregated_run_count: Option<i64>,
    has_more: Option<bool>,
    next_cursor: Option<String>,
}

impl GetAgentTopicsResponseModelBuilder {
    pub fn topics(mut self, value: Vec<AgentTopicResponseModel>) -> Self {
        self.topics = Some(value);
        self
    }

    pub fn window_start_unix_secs(mut self, value: i64) -> Self {
        self.window_start_unix_secs = Some(value);
        self
    }

    pub fn window_end_unix_secs(mut self, value: i64) -> Self {
        self.window_end_unix_secs = Some(value);
        self
    }

    pub fn aggregated_run_count(mut self, value: i64) -> Self {
        self.aggregated_run_count = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAgentTopicsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`topics`](GetAgentTopicsResponseModelBuilder::topics)
    /// - [`window_start_unix_secs`](GetAgentTopicsResponseModelBuilder::window_start_unix_secs)
    /// - [`window_end_unix_secs`](GetAgentTopicsResponseModelBuilder::window_end_unix_secs)
    pub fn build(self) -> Result<GetAgentTopicsResponseModel, BuildError> {
        Ok(GetAgentTopicsResponseModel {
            topics: self.topics.ok_or_else(|| BuildError::missing_field("topics"))?,
            window_start_unix_secs: self.window_start_unix_secs.ok_or_else(|| BuildError::missing_field("window_start_unix_secs"))?,
            window_end_unix_secs: self.window_end_unix_secs.ok_or_else(|| BuildError::missing_field("window_end_unix_secs"))?,
            aggregated_run_count: self.aggregated_run_count,
            has_more: self.has_more,
            next_cursor: self.next_cursor,
        })
    }
}
