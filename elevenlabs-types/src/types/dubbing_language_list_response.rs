pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingLanguageListResponse {
    /// The page of language targets for the project.
    #[serde(default)]
    pub languages: Vec<DubbingLanguageResponse>,
    /// Cursor for the next page, or null when there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl DubbingLanguageListResponse {
    pub fn builder() -> DubbingLanguageListResponseBuilder {
        <DubbingLanguageListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingLanguageListResponseBuilder {
    languages: Option<Vec<DubbingLanguageResponse>>,
    next_cursor: Option<String>,
}

impl DubbingLanguageListResponseBuilder {
    pub fn languages(mut self, value: Vec<DubbingLanguageResponse>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingLanguageListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`languages`](DubbingLanguageListResponseBuilder::languages)
    pub fn build(self) -> Result<DubbingLanguageListResponse, BuildError> {
        Ok(DubbingLanguageListResponse {
            languages: self.languages.ok_or_else(|| BuildError::missing_field("languages"))?,
            next_cursor: self.next_cursor,
        })
    }
}
