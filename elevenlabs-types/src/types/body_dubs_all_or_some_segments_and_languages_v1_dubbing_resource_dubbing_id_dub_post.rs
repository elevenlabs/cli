pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost {
    /// Dub only this list of segments.
    #[serde(default)]
    pub segments: Vec<String>,
    /// Dub only these languages for each segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
}

impl BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost {
    pub fn builder() -> BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPostBuilder {
        <BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPostBuilder {
    segments: Option<Vec<String>>,
    languages: Option<Vec<String>>,
}

impl BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPostBuilder {
    pub fn segments(mut self, value: Vec<String>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPostBuilder::segments)
    pub fn build(self) -> Result<BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost, BuildError> {
        Ok(BodyDubsAllOrSomeSegmentsAndLanguagesV1DubbingResourceDubbingIdDubPost {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
            languages: self.languages,
        })
    }
}

