pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChapterContentBlockInputModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<ChapterContentBlockInputModelSubType>,
    #[serde(default)]
    pub nodes: Vec<ChapterContentParagraphTtsNodeInputModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

impl ChapterContentBlockInputModel {
    pub fn builder() -> ChapterContentBlockInputModelBuilder {
        <ChapterContentBlockInputModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChapterContentBlockInputModelBuilder {
    sub_type: Option<ChapterContentBlockInputModelSubType>,
    nodes: Option<Vec<ChapterContentParagraphTtsNodeInputModel>>,
    block_id: Option<String>,
}

impl ChapterContentBlockInputModelBuilder {
    pub fn sub_type(mut self, value: ChapterContentBlockInputModelSubType) -> Self {
        self.sub_type = Some(value);
        self
    }

    pub fn nodes(mut self, value: Vec<ChapterContentParagraphTtsNodeInputModel>) -> Self {
        self.nodes = Some(value);
        self
    }

    pub fn block_id(mut self, value: impl Into<String>) -> Self {
        self.block_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChapterContentBlockInputModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`nodes`](ChapterContentBlockInputModelBuilder::nodes)
    pub fn build(self) -> Result<ChapterContentBlockInputModel, BuildError> {
        Ok(ChapterContentBlockInputModel {
            sub_type: self.sub_type,
            nodes: self.nodes.ok_or_else(|| BuildError::missing_field("nodes"))?,
            block_id: self.block_id,
        })
    }
}
