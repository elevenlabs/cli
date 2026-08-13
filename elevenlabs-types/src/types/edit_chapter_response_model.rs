pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditChapterResponseModel {
    pub chapter: ChapterWithContentResponseModel,
}

impl EditChapterResponseModel {
    pub fn builder() -> EditChapterResponseModelBuilder {
        <EditChapterResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditChapterResponseModelBuilder {
    chapter: Option<ChapterWithContentResponseModel>,
}

impl EditChapterResponseModelBuilder {
    pub fn chapter(mut self, value: ChapterWithContentResponseModel) -> Self {
        self.chapter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditChapterResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter`](EditChapterResponseModelBuilder::chapter)
    pub fn build(self) -> Result<EditChapterResponseModel, BuildError> {
        Ok(EditChapterResponseModel {
            chapter: self.chapter.ok_or_else(|| BuildError::missing_field("chapter"))?,
        })
    }
}
