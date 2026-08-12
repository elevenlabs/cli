pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChapterContentBlockResponseModel {
    #[serde(default)]
    pub block_id: String,
    #[serde(default)]
    pub nodes: Vec<ChapterContentBlockResponseModelNodesItem>,
}

impl ChapterContentBlockResponseModel {
    pub fn builder() -> ChapterContentBlockResponseModelBuilder {
        <ChapterContentBlockResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentBlockResponseModelBuilder {
    block_id: Option<String>,
    nodes: Option<Vec<ChapterContentBlockResponseModelNodesItem>>,
}

impl ChapterContentBlockResponseModelBuilder {
    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }

    pub fn nodes(mut self, value: Vec<ChapterContentBlockResponseModelNodesItem>) -> Self {
        self.nodes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentBlockResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`block_id`](ChapterContentBlockResponseModelBuilder::block_id)
    /// - [`nodes`](ChapterContentBlockResponseModelBuilder::nodes)
    pub fn build(self) -> Result<ChapterContentBlockResponseModel, BuildError> {
        Ok(ChapterContentBlockResponseModel {
            block_id: self.block_id.ok_or_else(|| BuildError::missing_field("block_id"))?,
            nodes: self.nodes.ok_or_else(|| BuildError::missing_field("nodes"))?,
        })
    }
}
