pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingSourceTranscriptResponse {
    /// BCP-47 language tag of the source transcript (null if unknown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The source segments, in playback order.
    #[serde(default)]
    pub segments: Vec<DubbingTranscriptSegment>,
    /// The project's source-transcript revision at read time.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingSourceTranscriptResponse {
    pub fn builder() -> DubbingSourceTranscriptResponseBuilder {
        <DubbingSourceTranscriptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingSourceTranscriptResponseBuilder {
    language: Option<String>,
    segments: Option<Vec<DubbingTranscriptSegment>>,
    revision: Option<i64>,
}

impl DubbingSourceTranscriptResponseBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<DubbingTranscriptSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingSourceTranscriptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingSourceTranscriptResponseBuilder::segments)
    /// - [`revision`](DubbingSourceTranscriptResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingSourceTranscriptResponse, BuildError> {
        Ok(DubbingSourceTranscriptResponse {
            language: self.language,
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
