pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChunkingOverrideRequestModel {
    /// Maximum chunk size in tokens. Null inherits from ancestors or the embedding model default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chunk_size: Option<i64>,
    /// Chunk overlap as a percentage of the final chunk size, applied after the chunk size is clamped to the embedding model. Null inherits from ancestors or the embedding model default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_percentage: Option<i64>,
    /// Custom text separators used when splitting. Null inherits from ancestors; an empty list clears inherited separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_separators: Option<Vec<String>>,
}

impl ChunkingOverrideRequestModel {
    pub fn builder() -> ChunkingOverrideRequestModelBuilder {
        <ChunkingOverrideRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChunkingOverrideRequestModelBuilder {
    max_chunk_size: Option<i64>,
    overlap_percentage: Option<i64>,
    custom_separators: Option<Vec<String>>,
}

impl ChunkingOverrideRequestModelBuilder {
    pub fn max_chunk_size(mut self, value: i64) -> Self {
        self.max_chunk_size = Some(value);
        self
    }

    pub fn overlap_percentage(mut self, value: i64) -> Self {
        self.overlap_percentage = Some(value);
        self
    }

    pub fn custom_separators(mut self, value: Vec<String>) -> Self {
        self.custom_separators = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChunkingOverrideRequestModel`].
    pub fn build(self) -> Result<ChunkingOverrideRequestModel, BuildError> {
        Ok(ChunkingOverrideRequestModel {
            max_chunk_size: self.max_chunk_size,
            overlap_percentage: self.overlap_percentage,
            custom_separators: self.custom_separators,
        })
    }
}
