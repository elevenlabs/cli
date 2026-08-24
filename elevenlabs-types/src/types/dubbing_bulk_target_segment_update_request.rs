pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingBulkTargetSegmentUpdateRequest {
    /// Map of segment id to the translation edit to apply to that segment.
    #[serde(default)]
    pub segments: HashMap<String, DubbingTargetSegmentUpdateRequest>,
}

impl DubbingBulkTargetSegmentUpdateRequest {
    pub fn builder() -> DubbingBulkTargetSegmentUpdateRequestBuilder {
        <DubbingBulkTargetSegmentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingBulkTargetSegmentUpdateRequestBuilder {
    segments: Option<HashMap<String, DubbingTargetSegmentUpdateRequest>>,
}

impl DubbingBulkTargetSegmentUpdateRequestBuilder {
    pub fn segments(mut self, value: HashMap<String, DubbingTargetSegmentUpdateRequest>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingBulkTargetSegmentUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingBulkTargetSegmentUpdateRequestBuilder::segments)
    pub fn build(self) -> Result<DubbingBulkTargetSegmentUpdateRequest, BuildError> {
        Ok(DubbingBulkTargetSegmentUpdateRequest {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
        })
    }
}

