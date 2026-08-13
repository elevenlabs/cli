pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChapterContentResponseModel {
    #[serde(default)]
    pub blocks: Vec<ChapterContentBlockResponseModel>,
}

impl ChapterContentResponseModel {
    pub fn builder() -> ChapterContentResponseModelBuilder {
        <ChapterContentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentResponseModelBuilder {
    blocks: Option<Vec<ChapterContentBlockResponseModel>>,
}

impl ChapterContentResponseModelBuilder {
    pub fn blocks(mut self, value: Vec<ChapterContentBlockResponseModel>) -> Self {
        self.blocks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blocks`](ChapterContentResponseModelBuilder::blocks)
    pub fn build(self) -> Result<ChapterContentResponseModel, BuildError> {
        Ok(ChapterContentResponseModel {
            blocks: self.blocks.ok_or_else(|| BuildError::missing_field("blocks"))?,
        })
    }
}
