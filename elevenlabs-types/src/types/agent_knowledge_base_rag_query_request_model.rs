pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagQueryRequestModel {
    /// Retrieval strategy to run, as in the agent's knowledge_base tool: semantic or keyword search for a query, cat to read one document in full, or ls to list the entries of a folder. Any strategy runs, whether or not the agent has it enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<SearchStrategy>,
    /// Query to search the knowledge base for. Required for the semantic and keyword strategies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// IDs or names of knowledge base documents or folders. Semantic and keyword search only search these entries; cat reads the single document given; ls lists the single folder given, or the top level of the knowledge base when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<String>>,
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
    strategy: Option<SearchStrategy>,
    query: Option<String>,
    documents: Option<Vec<String>>,
    use_agent_defaults: Option<bool>,
    max_documents_length: Option<i64>,
    max_retrieved_rag_chunks_count: Option<i64>,
}

impl AgentKnowledgeBaseRagQueryRequestModelBuilder {
    pub fn strategy(mut self, value: SearchStrategy) -> Self {
        self.strategy = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn documents(mut self, value: Vec<String>) -> Self {
        self.documents = Some(value);
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
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryRequestModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryRequestModel {
            strategy: self.strategy,
            query: self.query,
            documents: self.documents,
            use_agent_defaults: self.use_agent_defaults,
            max_documents_length: self.max_documents_length,
            max_retrieved_rag_chunks_count: self.max_retrieved_rag_chunks_count,
        })
    }
}
