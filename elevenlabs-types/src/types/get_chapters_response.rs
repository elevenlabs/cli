pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetChaptersResponse {
    #[serde(default)]
    pub chapters: Vec<ChapterResponse>,
}

impl GetChaptersResponse {
    pub fn builder() -> GetChaptersResponseBuilder {
        <GetChaptersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetChaptersResponseBuilder {
    chapters: Option<Vec<ChapterResponse>>,
}

impl GetChaptersResponseBuilder {
    pub fn chapters(mut self, value: Vec<ChapterResponse>) -> Self {
        self.chapters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetChaptersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapters`](GetChaptersResponseBuilder::chapters)
    pub fn build(self) -> Result<GetChaptersResponse, BuildError> {
        Ok(GetChaptersResponse {
            chapters: self.chapters.ok_or_else(|| BuildError::missing_field("chapters"))?,
        })
    }
}
