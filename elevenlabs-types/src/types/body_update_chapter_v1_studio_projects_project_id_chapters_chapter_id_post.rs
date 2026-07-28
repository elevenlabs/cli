pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost {
    /// The name of the chapter, used for identification only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The chapter content to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ChapterContentInputModel>,
}

impl BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost {
    pub fn builder() -> BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPostBuilder {
        <BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPostBuilder {
    name: Option<String>,
    content: Option<ChapterContentInputModel>,
}

impl BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: ChapterContentInputModel) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost`].
    pub fn build(self) -> Result<BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost, BuildError> {
        Ok(BodyUpdateChapterV1StudioProjectsProjectIdChaptersChapterIdPost {
            name: self.name,
            content: self.content,
        })
    }
}

