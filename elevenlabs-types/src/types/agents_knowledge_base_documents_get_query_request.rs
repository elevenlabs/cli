pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseDocumentsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl AgentsKnowledgeBaseDocumentsGetQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseDocumentsGetQueryRequestBuilder {
        <AgentsKnowledgeBaseDocumentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseDocumentsGetQueryRequestBuilder {
    agent_id: Option<String>,
}

impl AgentsKnowledgeBaseDocumentsGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseDocumentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsKnowledgeBaseDocumentsGetQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseDocumentsGetQueryRequest {
            agent_id: self.agent_id,
        })
    }
}

