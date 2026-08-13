pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RagChunkMetadata {
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub chunk_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub vector_distance: f64,
}

impl RagChunkMetadata {
    pub fn builder() -> RagChunkMetadataBuilder {
        <RagChunkMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagChunkMetadataBuilder {
    document_id: Option<String>,
    chunk_id: Option<String>,
    vector_distance: Option<f64>,
}

impl RagChunkMetadataBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn chunk_id(mut self, value: impl Into<String>) -> Self {
        self.chunk_id = Some(value.into());
        self
    }

    pub fn vector_distance(mut self, value: f64) -> Self {
        self.vector_distance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagChunkMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](RagChunkMetadataBuilder::document_id)
    /// - [`chunk_id`](RagChunkMetadataBuilder::chunk_id)
    /// - [`vector_distance`](RagChunkMetadataBuilder::vector_distance)
    pub fn build(self) -> Result<RagChunkMetadata, BuildError> {
        Ok(RagChunkMetadata {
            document_id: self.document_id.ok_or_else(|| BuildError::missing_field("document_id"))?,
            chunk_id: self.chunk_id.ok_or_else(|| BuildError::missing_field("chunk_id"))?,
            vector_distance: self.vector_distance.ok_or_else(|| BuildError::missing_field("vector_distance"))?,
        })
    }
}
