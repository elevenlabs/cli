pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsSummariesGetQueryRequest {
    /// List of agent IDs to fetch summaries for
    #[serde(default)]
    pub agent_ids: Vec<Option<String>>,
}

impl AgentsSummariesGetQueryRequest {
    pub fn builder() -> AgentsSummariesGetQueryRequestBuilder {
        <AgentsSummariesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsSummariesGetQueryRequestBuilder {
    agent_ids: Option<Vec<Option<String>>>,
}

impl AgentsSummariesGetQueryRequestBuilder {
    pub fn agent_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsSummariesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_ids`](AgentsSummariesGetQueryRequestBuilder::agent_ids)
    pub fn build(self) -> Result<AgentsSummariesGetQueryRequest, BuildError> {
        Ok(AgentsSummariesGetQueryRequest {
            agent_ids: self.agent_ids.ok_or_else(|| BuildError::missing_field("agent_ids"))?,
        })
    }
}

