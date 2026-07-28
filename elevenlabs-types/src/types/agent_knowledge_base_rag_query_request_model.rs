pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagQueryRequestModel {
    /// Query to run against the agent's knowledge base RAG index.
    #[serde(default)]
    pub query: String,
}

impl AgentKnowledgeBaseRagQueryRequestModel {
    pub fn builder() -> AgentKnowledgeBaseRagQueryRequestModelBuilder {
        <AgentKnowledgeBaseRagQueryRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagQueryRequestModelBuilder {
    query: Option<String>,
}

impl AgentKnowledgeBaseRagQueryRequestModelBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagQueryRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](AgentKnowledgeBaseRagQueryRequestModelBuilder::query)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryRequestModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryRequestModel {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
        })
    }
}
