pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDocumentChunkingOverrideRequestModel {
    /// RAG chunking override for a document or folder. Null clears the override so ancestors and embedding model defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_override: Option<ChunkingOverrideRequestModel>,
}

impl UpdateDocumentChunkingOverrideRequestModel {
    pub fn builder() -> UpdateDocumentChunkingOverrideRequestModelBuilder {
        <UpdateDocumentChunkingOverrideRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDocumentChunkingOverrideRequestModelBuilder {
    chunking_override: Option<ChunkingOverrideRequestModel>,
}

impl UpdateDocumentChunkingOverrideRequestModelBuilder {
    pub fn chunking_override(mut self, value: ChunkingOverrideRequestModel) -> Self {
        self.chunking_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDocumentChunkingOverrideRequestModel`].
    pub fn build(self) -> Result<UpdateDocumentChunkingOverrideRequestModel, BuildError> {
        Ok(UpdateDocumentChunkingOverrideRequestModel {
            chunking_override: self.chunking_override,
        })
    }
}

