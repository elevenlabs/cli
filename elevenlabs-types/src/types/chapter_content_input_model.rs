pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChapterContentInputModel {
    #[serde(default)]
    pub blocks: Vec<ChapterContentBlockInputModel>,
}

impl ChapterContentInputModel {
    pub fn builder() -> ChapterContentInputModelBuilder {
        <ChapterContentInputModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentInputModelBuilder {
    blocks: Option<Vec<ChapterContentBlockInputModel>>,
}

impl ChapterContentInputModelBuilder {
    pub fn blocks(mut self, value: Vec<ChapterContentBlockInputModel>) -> Self {
        self.blocks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentInputModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`blocks`](ChapterContentInputModelBuilder::blocks)
    pub fn build(self) -> Result<ChapterContentInputModel, BuildError> {
        Ok(ChapterContentInputModel {
            blocks: self.blocks.ok_or_else(|| BuildError::missing_field("blocks"))?,
        })
    }
}
