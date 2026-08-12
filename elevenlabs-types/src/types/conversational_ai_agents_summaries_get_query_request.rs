pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAgentsSummariesGetQueryRequest {
    /// List of agent IDs to fetch summaries for
    #[serde(default)]
    pub agent_ids: Vec<Option<String>>,
}

impl ConversationalAiAgentsSummariesGetQueryRequest {
    pub fn builder() -> ConversationalAiAgentsSummariesGetQueryRequestBuilder {
        <ConversationalAiAgentsSummariesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAgentsSummariesGetQueryRequestBuilder {
    agent_ids: Option<Vec<Option<String>>>,
}

impl ConversationalAiAgentsSummariesGetQueryRequestBuilder {
    pub fn agent_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.agent_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAgentsSummariesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_ids`](ConversationalAiAgentsSummariesGetQueryRequestBuilder::agent_ids)
    pub fn build(self) -> Result<ConversationalAiAgentsSummariesGetQueryRequest, BuildError> {
        Ok(ConversationalAiAgentsSummariesGetQueryRequest {
            agent_ids: self.agent_ids.ok_or_else(|| BuildError::missing_field("agent_ids"))?,
        })
    }
}

