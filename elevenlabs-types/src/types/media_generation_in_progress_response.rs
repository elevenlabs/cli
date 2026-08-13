pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A media generation that has not finished yet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MediaGenerationInProgressResponse {
    /// The unique identifier of the generation.
    #[serde(default)]
    pub id: String,
}

impl MediaGenerationInProgressResponse {
    pub fn builder() -> MediaGenerationInProgressResponseBuilder {
        <MediaGenerationInProgressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaGenerationInProgressResponseBuilder {
    id: Option<String>,
}

impl MediaGenerationInProgressResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaGenerationInProgressResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MediaGenerationInProgressResponseBuilder::id)
    pub fn build(self) -> Result<MediaGenerationInProgressResponse, BuildError> {
        Ok(MediaGenerationInProgressResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
