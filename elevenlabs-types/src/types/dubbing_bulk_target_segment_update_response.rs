pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result of a bulk target edit: the updated segments and the new revision.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingBulkTargetSegmentUpdateResponse {
    /// The edited target segments in their updated state.
    #[serde(default)]
    pub segments: Vec<DubbingTargetTranscriptSegment>,
    /// The target's revision after the edits.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingBulkTargetSegmentUpdateResponse {
    pub fn builder() -> DubbingBulkTargetSegmentUpdateResponseBuilder {
        <DubbingBulkTargetSegmentUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingBulkTargetSegmentUpdateResponseBuilder {
    segments: Option<Vec<DubbingTargetTranscriptSegment>>,
    revision: Option<i64>,
}

impl DubbingBulkTargetSegmentUpdateResponseBuilder {
    pub fn segments(mut self, value: Vec<DubbingTargetTranscriptSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingBulkTargetSegmentUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingBulkTargetSegmentUpdateResponseBuilder::segments)
    /// - [`revision`](DubbingBulkTargetSegmentUpdateResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingBulkTargetSegmentUpdateResponse, BuildError> {
        Ok(DubbingBulkTargetSegmentUpdateResponse {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
