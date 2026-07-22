pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result of a source-segment add or edit: the segment and the new revision.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingSourceSegmentUpdateResponse {
    /// The segment in its updated state.
    #[serde(default)]
    pub segment: DubbingTranscriptSegment,
    /// The project's source-transcript revision after this edit.
    #[serde(default)]
    pub revision: i64,
}

impl DubbingSourceSegmentUpdateResponse {
    pub fn builder() -> DubbingSourceSegmentUpdateResponseBuilder {
        <DubbingSourceSegmentUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingSourceSegmentUpdateResponseBuilder {
    segment: Option<DubbingTranscriptSegment>,
    revision: Option<i64>,
}

impl DubbingSourceSegmentUpdateResponseBuilder {
    pub fn segment(mut self, value: DubbingTranscriptSegment) -> Self {
        self.segment = Some(value);
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingSourceSegmentUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segment`](DubbingSourceSegmentUpdateResponseBuilder::segment)
    /// - [`revision`](DubbingSourceSegmentUpdateResponseBuilder::revision)
    pub fn build(self) -> Result<DubbingSourceSegmentUpdateResponse, BuildError> {
        Ok(DubbingSourceSegmentUpdateResponse {
            segment: self.segment.ok_or_else(|| BuildError::missing_field("segment"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
