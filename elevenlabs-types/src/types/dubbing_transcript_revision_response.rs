pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The new revision after a source edit that returns no segment (e.g. a delete).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingTranscriptRevisionResponse {
    /// The project's source-transcript revision after this edit.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingTranscriptRevisionResponse {
    pub fn builder() -> DubbingTranscriptRevisionResponseBuilder {
        <DubbingTranscriptRevisionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTranscriptRevisionResponseBuilder {
    revision: Option<i64>,
}

impl DubbingTranscriptRevisionResponseBuilder {
    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTranscriptRevisionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](DubbingTranscriptRevisionResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingTranscriptRevisionResponse, BuildError> {
        Ok(DubbingTranscriptRevisionResponse {
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
