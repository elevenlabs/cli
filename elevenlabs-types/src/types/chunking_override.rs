pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChunkingOverride {
    /// Maximum chunk size in tokens. Null uses the inherited value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_chunk_size: Option<i64>,
    /// Chunk overlap as a percentage of the chunk size. Null uses the inherited value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap_percentage: Option<i64>,
    /// Custom text separators used when splitting. Null uses the inherited value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_separators: Option<Vec<String>>,
}

impl ChunkingOverride {
    pub fn builder() -> ChunkingOverrideBuilder {
        <ChunkingOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChunkingOverrideBuilder {
    max_chunk_size: Option<i64>,
    overlap_percentage: Option<i64>,
    custom_separators: Option<Vec<String>>,
}

impl ChunkingOverrideBuilder {
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

    /// Consumes the builder and constructs a [`ChunkingOverride`].
    pub fn build(self) -> Result<ChunkingOverride, BuildError> {
        Ok(ChunkingOverride {
            max_chunk_size: self.max_chunk_size,
            overlap_percentage: self.overlap_percentage,
            custom_separators: self.custom_separators,
        })
    }
}
