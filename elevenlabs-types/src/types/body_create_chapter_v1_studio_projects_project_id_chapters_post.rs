pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateChapterV1StudioProjectsProjectIdChaptersPost {
    /// The name of the chapter, used for identification only.
    #[serde(default)]
    pub name: String,
    /// An optional URL from which we will extract content to initialize the Studio project. If this is set, 'from_url' and 'from_content' must be null. If neither 'from_url', 'from_document', 'from_content' are provided we will initialize the Studio project as blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_url: Option<String>,
}

impl BodyCreateChapterV1StudioProjectsProjectIdChaptersPost {
    pub fn builder() -> BodyCreateChapterV1StudioProjectsProjectIdChaptersPostBuilder {
        <BodyCreateChapterV1StudioProjectsProjectIdChaptersPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateChapterV1StudioProjectsProjectIdChaptersPostBuilder {
    name: Option<String>,
    from_url: Option<String>,
}

impl BodyCreateChapterV1StudioProjectsProjectIdChaptersPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn from_url(mut self, value: impl Into<String>) -> Self {
        self.from_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateChapterV1StudioProjectsProjectIdChaptersPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateChapterV1StudioProjectsProjectIdChaptersPostBuilder::name)
    pub fn build(self) -> Result<BodyCreateChapterV1StudioProjectsProjectIdChaptersPost, BuildError> {
        Ok(BodyCreateChapterV1StudioProjectsProjectIdChaptersPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            from_url: self.from_url,
        })
    }
}

