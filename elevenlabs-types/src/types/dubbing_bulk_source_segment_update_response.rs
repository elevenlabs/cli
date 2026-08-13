pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result of a bulk source edit: the updated segments and the new revision.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingBulkSourceSegmentUpdateResponse {
    /// The edited segments in their updated state.
    #[serde(default)]
    pub segments: Vec<DubbingTranscriptSegment>,
    /// The project's source-transcript revision after the edits.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingBulkSourceSegmentUpdateResponse {
    pub fn builder() -> DubbingBulkSourceSegmentUpdateResponseBuilder {
        <DubbingBulkSourceSegmentUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingBulkSourceSegmentUpdateResponseBuilder {
    segments: Option<Vec<DubbingTranscriptSegment>>,
    revision: Option<i64>,
}

impl DubbingBulkSourceSegmentUpdateResponseBuilder {
    pub fn segments(mut self, value: Vec<DubbingTranscriptSegment>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingBulkSourceSegmentUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingBulkSourceSegmentUpdateResponseBuilder::segments)
    /// - [`revision`](DubbingBulkSourceSegmentUpdateResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingBulkSourceSegmentUpdateResponse, BuildError> {
        Ok(DubbingBulkSourceSegmentUpdateResponse {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
