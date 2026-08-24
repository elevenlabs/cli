pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PendingBlocksMetadataModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_global_offset_ms: Option<i64>,
    #[serde(default)]
    pub block_ids: Vec<String>,
}

impl PendingBlocksMetadataModel {
    pub fn builder() -> PendingBlocksMetadataModelBuilder {
        <PendingBlocksMetadataModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PendingBlocksMetadataModelBuilder {
    target_global_offset_ms: Option<i64>,
    block_ids: Option<Vec<String>>,
}

impl PendingBlocksMetadataModelBuilder {
    pub fn target_global_offset_ms(mut self, value: i64) -> Self {
        self.target_global_offset_ms = Some(value);
        self
    }

    pub fn block_ids(mut self, value: Vec<String>) -> Self {
        self.block_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PendingBlocksMetadataModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`block_ids`](PendingBlocksMetadataModelBuilder::block_ids)
    pub fn build(self) -> Result<PendingBlocksMetadataModel, BuildError> {
        Ok(PendingBlocksMetadataModel {
            target_global_offset_ms: self.target_global_offset_ms,
            block_ids: self.block_ids.ok_or_else(|| BuildError::missing_field("block_ids"))?,
        })
    }
}
