pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseDocumentsChunksListQueryRequest {
    /// The embedding model used to retrieve the chunk.
    pub embedding_model: EmbeddingModelEnum,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsKnowledgeBaseDocumentsChunksListQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseDocumentsChunksListQueryRequestBuilder {
        <AgentsKnowledgeBaseDocumentsChunksListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseDocumentsChunksListQueryRequestBuilder {
    embedding_model: Option<EmbeddingModelEnum>,
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl AgentsKnowledgeBaseDocumentsChunksListQueryRequestBuilder {
    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseDocumentsChunksListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`embedding_model`](AgentsKnowledgeBaseDocumentsChunksListQueryRequestBuilder::embedding_model)
    pub fn build(self) -> Result<AgentsKnowledgeBaseDocumentsChunksListQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseDocumentsChunksListQueryRequest {
            embedding_model: self.embedding_model.ok_or_else(|| BuildError::missing_field("embedding_model"))?,
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}

