pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost {
    /// The Target language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost {
    pub fn builder() -> BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePostBuilder {
        <BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePostBuilder {
    language: Option<String>,
}

impl BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePostBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost`].
    pub fn build(self) -> Result<BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost, BuildError> {
        Ok(BodyAddALanguageToTheResourceV1DubbingResourceDubbingIdLanguagePost {
            language: self.language,
        })
    }
}

