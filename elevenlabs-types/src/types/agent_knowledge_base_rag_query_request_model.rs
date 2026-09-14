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
    /// Optional maximum total character length of document chunks returned. Overrides the selected RAG settings for this query only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_documents_length: Option<i64>,
    /// Optional maximum number of document chunks retrieved. Overrides the selected RAG settings for this query only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retrieved_rag_chunks_count: Option<i64>,
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
    max_documents_length: Option<i64>,
    max_retrieved_rag_chunks_count: Option<i64>,
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

    pub fn max_documents_length(mut self, value: i64) -> Self {
        self.max_documents_length = Some(value);
        self
    }

    pub fn max_retrieved_rag_chunks_count(mut self, value: i64) -> Self {
        self.max_retrieved_rag_chunks_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagQueryRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](AgentKnowledgeBaseRagQueryRequestModelBuilder::query)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryRequestModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryRequestModel {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            use_agent_defaults: self.use_agent_defaults,
            max_documents_length: self.max_documents_length,
            max_retrieved_rag_chunks_count: self.max_retrieved_rag_chunks_count,
        })
    }
}
