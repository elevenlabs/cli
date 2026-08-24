pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseDocumentChunksResponseModel {
    #[serde(default)]
    pub chunks: Vec<KnowledgeBaseDocumentChunkResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl KnowledgeBaseDocumentChunksResponseModel {
    pub fn builder() -> KnowledgeBaseDocumentChunksResponseModelBuilder {
        <KnowledgeBaseDocumentChunksResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseDocumentChunksResponseModelBuilder {
    chunks: Option<Vec<KnowledgeBaseDocumentChunkResponseModel>>,
    next_cursor: Option<String>,
}

impl KnowledgeBaseDocumentChunksResponseModelBuilder {
    pub fn chunks(mut self, value: Vec<KnowledgeBaseDocumentChunkResponseModel>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseDocumentChunksResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](KnowledgeBaseDocumentChunksResponseModelBuilder::chunks)
    pub fn build(self) -> Result<KnowledgeBaseDocumentChunksResponseModel, BuildError> {
        Ok(KnowledgeBaseDocumentChunksResponseModel {
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            next_cursor: self.next_cursor,
        })
    }
}
