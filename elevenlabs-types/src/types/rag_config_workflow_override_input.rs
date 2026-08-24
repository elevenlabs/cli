pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RagConfigWorkflowOverrideInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<EmbeddingModelEnum>,
    /// Maximum vector distance of retrieved chunks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_vector_distance: Option<f64>,
    /// Maximum total length of document chunks retrieved from RAG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_documents_length: Option<i64>,
    /// Maximum number of RAG document chunks to initially retrieve from the vector store. These are then further filtered by vector distance and total length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retrieved_rag_chunks_count: Option<i64>,
    /// Number of candidates evaluated in ANN vector search. Higher number means better results, but higher latency. Minimum recommended value is 100. If disabled, the default value is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_candidates: Option<i64>,
    /// Custom prompt for rewriting user queries before RAG retrieval. The conversation history will be automatically appended at the end. If not set, the default prompt will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_rewrite_prompt_override: Option<String>,
}

impl RagConfigWorkflowOverrideInput {
    pub fn builder() -> RagConfigWorkflowOverrideInputBuilder {
        <RagConfigWorkflowOverrideInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagConfigWorkflowOverrideInputBuilder {
    enabled: Option<bool>,
    embedding_model: Option<EmbeddingModelEnum>,
    max_vector_distance: Option<f64>,
    max_documents_length: Option<i64>,
    max_retrieved_rag_chunks_count: Option<i64>,
    num_candidates: Option<i64>,
    query_rewrite_prompt_override: Option<String>,
}

impl RagConfigWorkflowOverrideInputBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    pub fn max_vector_distance(mut self, value: f64) -> Self {
        self.max_vector_distance = Some(value);
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

    pub fn num_candidates(mut self, value: i64) -> Self {
        self.num_candidates = Some(value);
        self
    }

    pub fn query_rewrite_prompt_override(mut self, value: impl Into<String>) -> Self {
        self.query_rewrite_prompt_override = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RagConfigWorkflowOverrideInput`].
    pub fn build(self) -> Result<RagConfigWorkflowOverrideInput, BuildError> {
        Ok(RagConfigWorkflowOverrideInput {
            enabled: self.enabled,
            embedding_model: self.embedding_model,
            max_vector_distance: self.max_vector_distance,
            max_documents_length: self.max_documents_length,
            max_retrieved_rag_chunks_count: self.max_retrieved_rag_chunks_count,
            num_candidates: self.num_candidates,
            query_rewrite_prompt_override: self.query_rewrite_prompt_override,
        })
    }
}
