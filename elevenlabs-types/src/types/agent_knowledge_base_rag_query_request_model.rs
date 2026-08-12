pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagQueryRequestModel {
    /// Query to run against the agent's knowledge base RAG index.
    #[serde(default)]
    pub query: String,
    /// When true (the default), retrieval uses the agent's own RAG settings, reproducing exactly what the agent would retrieve. Set to false to retrieve with neutral default RAG settings instead (the agent's embedding model is always kept, since it determines which vector index exists). Useful for auditing the knowledge base independently of how a particular agent is tuned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_agent_defaults: Option<bool>,
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
    use_agent_defaults: Option<bool>,
}

impl AgentKnowledgeBaseRagQueryRequestModelBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn use_agent_defaults(mut self, value: bool) -> Self {
        self.use_agent_defaults = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagQueryRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](AgentKnowledgeBaseRagQueryRequestModelBuilder::query)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryRequestModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryRequestModel {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            use_agent_defaults: self.use_agent_defaults,
        })
    }
}
