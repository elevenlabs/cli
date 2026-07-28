pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddChapterResponseModel {
    pub chapter: ChapterWithContentResponseModel,
}

impl AddChapterResponseModel {
    pub fn builder() -> AddChapterResponseModelBuilder {
        <AddChapterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddChapterResponseModelBuilder {
    chapter: Option<ChapterWithContentResponseModel>,
}

impl AddChapterResponseModelBuilder {
    pub fn chapter(mut self, value: ChapterWithContentResponseModel) -> Self {
        self.chapter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddChapterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter`](AddChapterResponseModelBuilder::chapter)
    pub fn build(self) -> Result<AddChapterResponseModel, BuildError> {
        Ok(AddChapterResponseModel {
            chapter: self.chapter.ok_or_else(|| BuildError::missing_field("chapter"))?,
        })
    }
}
