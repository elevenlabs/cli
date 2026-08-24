pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A newly queued media generation; fetch the GET endpoint for the output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MediaGenerationCreateResponse {
    /// The unique identifier of the generation. Pass it to the corresponding GET endpoint to retrieve the output.
    #[serde(default)]
    pub id: String,
    /// A newly created generation is always `pending`.
    pub status: String,
}

impl MediaGenerationCreateResponse {
    pub fn builder() -> MediaGenerationCreateResponseBuilder {
        <MediaGenerationCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MediaGenerationCreateResponseBuilder {
    id: Option<String>,
    status: Option<String>,
}

impl MediaGenerationCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MediaGenerationCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MediaGenerationCreateResponseBuilder::id)
    /// - [`status`](MediaGenerationCreateResponseBuilder::status)
    pub fn build(self) -> Result<MediaGenerationCreateResponse, BuildError> {
        Ok(MediaGenerationCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
