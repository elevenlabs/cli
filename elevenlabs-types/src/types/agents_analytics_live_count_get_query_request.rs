pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsAnalyticsLiveCountGetQueryRequest {
    /// The id of an agent to restrict the analytics to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Restrict analytics to the union of the given agents. Takes precedence over `agent_id` when both are supplied.
    #[serde(default)]
    pub agent_ids: Vec<Option<String>>,
}

impl AgentsAnalyticsLiveCountGetQueryRequest {
    pub fn builder() -> AgentsAnalyticsLiveCountGetQueryRequestBuilder {
        <AgentsAnalyticsLiveCountGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsAnalyticsLiveCountGetQueryRequestBuilder {
    agent_id: Option<String>,
    agent_ids: Option<Vec<Option<String>>>,
}

impl AgentsAnalyticsLiveCountGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsAnalyticsLiveCountGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_ids`](AgentsAnalyticsLiveCountGetQueryRequestBuilder::agent_ids)
    pub fn build(self) -> Result<AgentsAnalyticsLiveCountGetQueryRequest, BuildError> {
        Ok(AgentsAnalyticsLiveCountGetQueryRequest {
            agent_id: self.agent_id,
            agent_ids: self.agent_ids.ok_or_else(|| BuildError::missing_field("agent_ids"))?,
        })
    }
}

