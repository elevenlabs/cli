pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseRagChunkModel {
    #[serde(default)]
    pub chunk_id: String,
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub content: String,
}

impl KnowledgeBaseRagChunkModel {
    pub fn builder() -> KnowledgeBaseRagChunkModelBuilder {
        <KnowledgeBaseRagChunkModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseRagChunkModelBuilder {
    chunk_id: Option<String>,
    document_id: Option<String>,
    content: Option<String>,
}

impl KnowledgeBaseRagChunkModelBuilder {
    pub fn chunk_id(mut self, value: impl Into<String>) -> Self {
        self.chunk_id = Some(value.into());
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseRagChunkModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunk_id`](KnowledgeBaseRagChunkModelBuilder::chunk_id)
    /// - [`document_id`](KnowledgeBaseRagChunkModelBuilder::document_id)
    /// - [`content`](KnowledgeBaseRagChunkModelBuilder::content)
    pub fn build(self) -> Result<KnowledgeBaseRagChunkModel, BuildError> {
        Ok(KnowledgeBaseRagChunkModel {
            chunk_id: self.chunk_id.ok_or_else(|| BuildError::missing_field("chunk_id"))?,
            document_id: self.document_id.ok_or_else(|| BuildError::missing_field("document_id"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
