pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingTargetTranscriptResponse {
    /// BCP-47 language tag of the source transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    /// BCP-47 language tag this target is translated into.
    #[serde(default)]
    pub target_language: String,
    /// The target segments, in playback order.
    #[serde(default)]
    pub segments: Vec<DubbingTargetTranscriptSegment>,
    /// The target's revision at read time.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingTargetTranscriptResponse {
    pub fn builder() -> DubbingTargetTranscriptResponseBuilder {
        <DubbingTargetTranscriptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingTargetTranscriptResponseBuilder {
    source_language: Option<String>,
    target_language: Option<String>,
    segments: Option<Vec<DubbingTargetTranscriptSegment>>,
    revision: Option<i64>,
}

impl DubbingTargetTranscriptResponseBuilder {
    pub fn source_language(mut self, value: impl Into<String>) -> Self {
        self.source_language = Some(value.into());
        self
    }

    pub fn target_language(mut self, value: impl Into<String>) -> Self {
        self.target_language = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<DubbingTargetTranscriptSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingTargetTranscriptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target_language`](DubbingTargetTranscriptResponseBuilder::target_language)
    /// - [`segments`](DubbingTargetTranscriptResponseBuilder::segments)
    /// - [`revision`](DubbingTargetTranscriptResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingTargetTranscriptResponse, BuildError> {
        Ok(DubbingTargetTranscriptResponse {
            source_language: self.source_language,
            target_language: self.target_language.ok_or_else(|| BuildError::missing_field("target_language"))?,
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
