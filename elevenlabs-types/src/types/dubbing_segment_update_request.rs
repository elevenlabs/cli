pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A partial update to a source segment; omitted fields are left unchanged.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingSegmentUpdateRequest {
    /// New text for the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// New speaker ID for the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    /// New start time, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start_s: Option<f64>,
    /// New end time, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end_s: Option<f64>,
}

impl DubbingSegmentUpdateRequest {
    pub fn builder() -> DubbingSegmentUpdateRequestBuilder {
        <DubbingSegmentUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingSegmentUpdateRequestBuilder {
    text: Option<String>,
    speaker_id: Option<String>,
    start_s: Option<f64>,
    end_s: Option<f64>,
}

impl DubbingSegmentUpdateRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    pub fn start_s(mut self, value: f64) -> Self {
        self.start_s = Some(value);
        self
    }

    pub fn end_s(mut self, value: f64) -> Self {
        self.end_s = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingSegmentUpdateRequest`].
    pub fn build(self) -> Result<DubbingSegmentUpdateRequest, BuildError> {
        Ok(DubbingSegmentUpdateRequest {
            text: self.text,
            speaker_id: self.speaker_id,
            start_s: self.start_s,
            end_s: self.end_s,
        })
    }
}
