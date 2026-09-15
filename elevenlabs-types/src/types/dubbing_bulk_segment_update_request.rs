pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingBulkSegmentUpdateRequest {
    /// Map of segment ID to the partial update to apply to that segment. At least one entry and at most 500.
    #[serde(default)]
    pub segments: HashMap<String, DubbingSegmentUpdateRequest>,
}

impl DubbingBulkSegmentUpdateRequest {
    pub fn builder() -> DubbingBulkSegmentUpdateRequestBuilder {
        <DubbingBulkSegmentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingBulkSegmentUpdateRequestBuilder {
    segments: Option<HashMap<String, DubbingSegmentUpdateRequest>>,
}

impl DubbingBulkSegmentUpdateRequestBuilder {
    pub fn segments(mut self, value: HashMap<String, DubbingSegmentUpdateRequest>) -> Self {
        self.segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingBulkSegmentUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`segments`](DubbingBulkSegmentUpdateRequestBuilder::segments)
    pub fn build(self) -> Result<DubbingBulkSegmentUpdateRequest, BuildError> {
        Ok(DubbingBulkSegmentUpdateRequest {
            segments: self.segments.ok_or_else(|| BuildError::missing_field("segments"))?,
        })
    }
}

