pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDocumentChunkingOverrideResponseModel {
    /// ID of the document or folder whose chunking override was updated.
    #[serde(default)]
    pub id: String,
    /// The resulting chunking override. Null means the entity inherits from ancestors or the embedding model default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_override: Option<ChunkingOverride>,
    /// Whether the effective configuration changed and background reindexing was queued.
    #[serde(default)]
    pub reindex_queued: bool,
}

impl UpdateDocumentChunkingOverrideResponseModel {
    pub fn builder() -> UpdateDocumentChunkingOverrideResponseModelBuilder {
        <UpdateDocumentChunkingOverrideResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDocumentChunkingOverrideResponseModelBuilder {
    id: Option<String>,
    chunking_override: Option<ChunkingOverride>,
    reindex_queued: Option<bool>,
}

impl UpdateDocumentChunkingOverrideResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn chunking_override(mut self, value: ChunkingOverride) -> Self {
        self.chunking_override = Some(value);
        self
    }

    pub fn reindex_queued(mut self, value: bool) -> Self {
        self.reindex_queued = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDocumentChunkingOverrideResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateDocumentChunkingOverrideResponseModelBuilder::id)
    /// - [`reindex_queued`](UpdateDocumentChunkingOverrideResponseModelBuilder::reindex_queued)
    pub fn build(self) -> Result<UpdateDocumentChunkingOverrideResponseModel, BuildError> {
        Ok(UpdateDocumentChunkingOverrideResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            chunking_override: self.chunking_override,
            reindex_queued: self.reindex_queued.ok_or_else(|| BuildError::missing_field("reindex_queued"))?,
        })
    }
}
