pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RagRetrievalInfo {
    #[serde(default)]
    pub chunks: Vec<RagChunkMetadata>,
    pub embedding_model: EmbeddingModelEnum,
    #[serde(default)]
    pub retrieval_query: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub rag_latency_secs: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_chunk_ids: Option<Vec<String>>,
}

impl RagRetrievalInfo {
    pub fn builder() -> RagRetrievalInfoBuilder {
        <RagRetrievalInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagRetrievalInfoBuilder {
    chunks: Option<Vec<RagChunkMetadata>>,
    embedding_model: Option<EmbeddingModelEnum>,
    retrieval_query: Option<String>,
    rag_latency_secs: Option<f64>,
    used_chunk_ids: Option<Vec<String>>,
}

impl RagRetrievalInfoBuilder {
    pub fn chunks(mut self, value: Vec<RagChunkMetadata>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn embedding_model(mut self, value: EmbeddingModelEnum) -> Self {
        self.embedding_model = Some(value);
        self
    }

    pub fn retrieval_query(mut self, value: impl Into<String>) -> Self {
        self.retrieval_query = Some(value.into());
        self
    }

    pub fn rag_latency_secs(mut self, value: f64) -> Self {
        self.rag_latency_secs = Some(value);
        self
    }

    pub fn used_chunk_ids(mut self, value: Vec<String>) -> Self {
        self.used_chunk_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagRetrievalInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](RagRetrievalInfoBuilder::chunks)
    /// - [`embedding_model`](RagRetrievalInfoBuilder::embedding_model)
    /// - [`retrieval_query`](RagRetrievalInfoBuilder::retrieval_query)
    /// - [`rag_latency_secs`](RagRetrievalInfoBuilder::rag_latency_secs)
    pub fn build(self) -> Result<RagRetrievalInfo, BuildError> {
        Ok(RagRetrievalInfo {
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            embedding_model: self.embedding_model.ok_or_else(|| BuildError::missing_field("embedding_model"))?,
            retrieval_query: self.retrieval_query.ok_or_else(|| BuildError::missing_field("retrieval_query"))?,
            rag_latency_secs: self.rag_latency_secs.ok_or_else(|| BuildError::missing_field("rag_latency_secs"))?,
            used_chunk_ids: self.used_chunk_ids,
        })
    }
}
