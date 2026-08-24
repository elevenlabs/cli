pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseRagToolResultModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<KnowledgeBaseRagToolStatus>,
    /// Number of relevant chunks retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_count: Option<i64>,
    /// Human-readable status for the LLM about the search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Retrieved chunks; populated only in the rag-result-in-tool-result mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunks: Option<Vec<KnowledgeBaseRagChunkModel>>,
}

impl KnowledgeBaseRagToolResultModel {
    pub fn builder() -> KnowledgeBaseRagToolResultModelBuilder {
        <KnowledgeBaseRagToolResultModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseRagToolResultModelBuilder {
    status: Option<KnowledgeBaseRagToolStatus>,
    chunk_count: Option<i64>,
    message: Option<String>,
    chunks: Option<Vec<KnowledgeBaseRagChunkModel>>,
}

impl KnowledgeBaseRagToolResultModelBuilder {
    pub fn status(mut self, value: KnowledgeBaseRagToolStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn chunk_count(mut self, value: i64) -> Self {
        self.chunk_count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn chunks(mut self, value: Vec<KnowledgeBaseRagChunkModel>) -> Self {
        self.chunks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseRagToolResultModel`].
    pub fn build(self) -> Result<KnowledgeBaseRagToolResultModel, BuildError> {
        Ok(KnowledgeBaseRagToolResultModel {
            status: self.status,
            chunk_count: self.chunk_count,
            message: self.message,
            chunks: self.chunks,
        })
    }
}
