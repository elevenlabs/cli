pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequest {
    /// The embedding model used to retrieve the chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<EmbeddingModelEnum>,
}

impl ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequest {
    pub fn builder() -> ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
        <ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
    embedding_model: Option<EmbeddingModelEnum>,
}

impl ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequestBuilder {
    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequest, BuildError> {
        Ok(ConversationalAiKnowledgeBaseDocumentsChunkGetQueryRequest {
            embedding_model: self.embedding_model,
        })
    }
}

