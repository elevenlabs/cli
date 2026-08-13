pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One page of the caller's generations, newest first.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MediaGenerationListResponse {
    /// The generations on this page, newest first. Each item has the same shape as the corresponding GET endpoint's response.
    #[serde(default)]
    pub generations: Vec<MediaGenerationResponse>,
    /// Pass as `cursor` to fetch the next page. `null` when there is no further page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Whether more generations exist beyond this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl MediaGenerationListResponse {
    pub fn builder() -> MediaGenerationListResponseBuilder {
        <MediaGenerationListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaGenerationListResponseBuilder {
    generations: Option<Vec<MediaGenerationResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl MediaGenerationListResponseBuilder {
    pub fn generations(mut self, value: Vec<MediaGenerationResponse>) -> Self {
        self.generations = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MediaGenerationListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generations`](MediaGenerationListResponseBuilder::generations)
    pub fn build(self) -> Result<MediaGenerationListResponse, BuildError> {
        Ok(MediaGenerationListResponse {
            generations: self.generations.ok_or_else(|| BuildError::missing_field("generations"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more,
        })
    }
}
