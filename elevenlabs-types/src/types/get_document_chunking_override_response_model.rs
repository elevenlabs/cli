pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDocumentChunkingOverrideResponseModel {
    /// ID of the document or folder whose chunking settings were read.
    #[serde(default)]
    pub id: String,
    /// Override stored directly on this entity. Null means every setting is inherited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_override: Option<ChunkingOverride>,
    /// Effective override after merging this entity with its ancestor folders. Null fields use embedding model defaults.
    #[serde(default)]
    pub effective_chunking_override: ChunkingOverride,
    /// Override inherited from ancestor folders, ignoring this entity's own override. Null fields use embedding model defaults.
    #[serde(default)]
    pub inherited_chunking_override: ChunkingOverride,
}

impl GetDocumentChunkingOverrideResponseModel {
    pub fn builder() -> GetDocumentChunkingOverrideResponseModelBuilder {
        <GetDocumentChunkingOverrideResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDocumentChunkingOverrideResponseModelBuilder {
    id: Option<String>,
    chunking_override: Option<ChunkingOverride>,
    effective_chunking_override: Option<ChunkingOverride>,
    inherited_chunking_override: Option<ChunkingOverride>,
}

impl GetDocumentChunkingOverrideResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn chunking_override(mut self, value: ChunkingOverride) -> Self {
        self.chunking_override = Some(value);
        self
    }

    pub fn effective_chunking_override(mut self, value: ChunkingOverride) -> Self {
        self.effective_chunking_override = Some(value);
        self
    }

    pub fn inherited_chunking_override(mut self, value: ChunkingOverride) -> Self {
        self.inherited_chunking_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDocumentChunkingOverrideResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDocumentChunkingOverrideResponseModelBuilder::id)
    /// - [`effective_chunking_override`](GetDocumentChunkingOverrideResponseModelBuilder::effective_chunking_override)
    /// - [`inherited_chunking_override`](GetDocumentChunkingOverrideResponseModelBuilder::inherited_chunking_override)
    pub fn build(self) -> Result<GetDocumentChunkingOverrideResponseModel, BuildError> {
        Ok(GetDocumentChunkingOverrideResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            chunking_override: self.chunking_override,
            effective_chunking_override: self.effective_chunking_override.ok_or_else(|| BuildError::missing_field("effective_chunking_override"))?,
            inherited_chunking_override: self.inherited_chunking_override.ok_or_else(|| BuildError::missing_field("inherited_chunking_override"))?,
        })
    }
}
