pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseDocumentsChunkGetQueryRequest {
    /// The embedding model used to retrieve the chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<EmbeddingModelEnum>,
}

impl AgentsKnowledgeBaseDocumentsChunkGetQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
        <AgentsKnowledgeBaseDocumentsChunkGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
    embedding_model: Option<EmbeddingModelEnum>,
}

impl AgentsKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseDocumentsChunkGetQueryRequest`].
    pub fn build(self) -> Result<AgentsKnowledgeBaseDocumentsChunkGetQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseDocumentsChunkGetQueryRequest {
            embedding_model: self.embedding_model,
        })
    }
}

