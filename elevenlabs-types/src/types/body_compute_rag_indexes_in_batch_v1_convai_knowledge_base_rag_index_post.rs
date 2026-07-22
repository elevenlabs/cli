pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost {
    /// List of requested RAG indexes. Minimum 1, maximum 100 items.
    #[serde(default)]
    pub items: Vec<GetOrCreateRagIndexRequestModel>,
}

impl BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost {
    pub fn builder() -> BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPostBuilder {
        <BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPostBuilder {
    items: Option<Vec<GetOrCreateRagIndexRequestModel>>,
}

impl BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPostBuilder {
    pub fn items(mut self, value: Vec<GetOrCreateRagIndexRequestModel>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPostBuilder::items)
    pub fn build(self) -> Result<BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost, BuildError> {
        Ok(BodyComputeRagIndexesInBatchV1ConvaiKnowledgeBaseRagIndexPost {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}

